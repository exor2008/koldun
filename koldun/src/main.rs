#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use defmt::*;
use defmt_rtt as _;
use embassy_executor::Spawner;
use embassy_rp::bind_interrupts;
use embassy_rp::gpio::{Level, Output};
use embassy_rp::peripherals::PIO0;
use embassy_rp::pio::{InterruptHandler, Pio};
use embassy_time::{Duration, Timer};
use embedded_graphics::{pixelcolor::Rgb565, prelude::*};
use heapless::Vec;
use koldun::ili9431::{Command, ILI9431};
use panic_probe as _;
use tinytga::Tga;

bind_interrupts!(struct Irqs {
    PIO0_IRQ_0 => InterruptHandler<PIO0>;
});

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    info!("Start");
    let p = embassy_rp::init(Default::default());

    let mut led = Output::new(p.PIN_26, Level::Low);
    let mut reset = Output::new(p.PIN_27, Level::Low);

    // reset
    reset.set_low();
    Timer::after(Duration::from_millis(10)).await;
    reset.set_high();
    Timer::after(Duration::from_millis(120)).await;
    led.set_high();
    info!("Begin");

    let Pio {
        mut common, sm0, ..
    } = Pio::new(p.PIO0, Irqs);

    let mut display = ILI9431::new(
        &mut common,
        sm0,
        p.DMA_CH0,
        p.PIN_2,
        p.PIN_3,
        p.PIN_4,
        p.PIN_5,
        p.PIN_6,
        p.PIN_7,
        p.PIN_8,
        p.PIN_9,
        p.PIN_10,
        p.PIN_11,
        p.PIN_12,
        p.PIN_13,
        p.PIN_14,
        p.PIN_15,
        p.PIN_16,
        p.PIN_17,
        p.PIN_18,
        p.PIN_19,
        p.PIN_20,
        p.PIN_21,
    );

    display
        .write_command(Command::InterfacePixelFormat, &[0b01010101])
        .await;

    display.write_command(Command::SleepOut, &[]).await;
    display
        .write_command(Command::DisplayInversionOff, &[])
        .await;
    display
        .write_command(Command::MemoryAccessControl, &[0b10001000])
        .await;

    display.write_command(Command::NormalDisplayMode, &[]).await;
    display.write_command(Command::DisplayOn, &[]).await;
    display.write_command(Command::IdleModeOff, &[]).await;

    let start: u16 = 0;
    let end: u16 = 31;
    let data = [
        ((start >> 8) as u8) as u16,
        ((start & 0xff) as u8) as u16,
        ((end >> 8) as u8) as u16,
        ((end & 0xff) as u8) as u16,
    ];
    display
        .write_command(Command::ColumnAddressSet, &data)
        .await;

    let start: u16 = 0;
    let end: u16 = 31;
    let data = [
        ((start >> 8) as u8) as u16,
        ((start & 0xff) as u8) as u16,
        ((end >> 8) as u8) as u16,
        ((end & 0xff) as u8) as u16,
    ];
    display.write_command(Command::PageAddressSet, &data).await;

    let data = include_bytes!("./face.tga");
    let tga: Tga<Rgb565> = Tga::from_slice(data).unwrap();

    let p: Vec<_, { 240 * 260 }> = tga.pixels().map(|p| convert(p)).collect();

    display
        .write_command(Command::MemoryWrite, p.as_slice())
        .await;

    loop {}
}

fn convert(p: Pixel<Rgb565>) -> u16 {
    let b = p.1.to_ne_bytes();
    (b[1] as u16) << 8 | (b[0]) as u16
}
