#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use defmt::*;
use defmt_rtt as _;
use embassy_executor::Spawner;
// use embassy_rp::clocks::{clk_sys_freq, pll_sys_freq, PllConfig, SysClkConfig};
use embassy_rp::gpio::{Level, Output};
use embassy_time::{Duration, Timer};
use panic_probe as _;

macro_rules! write_bit {
    ($output_pin:expr, $bit_set:expr) => {
        if $bit_set {
            $output_pin.set_high();
        } else {
            $output_pin.set_low();
        }
    };
}

macro_rules! write_2bytes {
    ($byte:expr, $db0:expr,$db1:expr,$db2:expr,$db3:expr,$db4:expr,$db5:expr,$db6:expr,$db7:expr,) => {
        write_bit!($db0, (1 << 0) & $byte != 0);
        write_bit!($db1, (1 << 1) & $byte != 0);
        write_bit!($db2, (1 << 2) & $byte != 0);
        write_bit!($db3, (1 << 3) & $byte != 0);
        write_bit!($db4, (1 << 4) & $byte != 0);
        write_bit!($db5, (1 << 5) & $byte != 0);
        write_bit!($db6, (1 << 6) & $byte != 0);
        write_bit!($db7, (1 << 7) & $byte != 0);
    };
}

// fn encode_rgb565_16bit(pixel: &(u8, u8, u8)) -> u8 {
//     (((pixel.0 & 0b11111) as u8) << 10)
//         | (((pixel.1 & 0b111111) as u8) << 4)
//         | (pixel.2 & 0b11111) as u8
// }

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    info!("Start");
    let p = embassy_rp::init(Default::default());

    let mut db0 = Output::new(p.PIN_2, Level::Low);
    let mut db1 = Output::new(p.PIN_3, Level::Low);
    let mut db2 = Output::new(p.PIN_4, Level::Low);
    let mut db3 = Output::new(p.PIN_5, Level::Low);
    let mut db4 = Output::new(p.PIN_6, Level::Low);
    let mut db5 = Output::new(p.PIN_7, Level::Low);
    let mut db6 = Output::new(p.PIN_8, Level::Low);
    let mut db7 = Output::new(p.PIN_9, Level::Low);

    // let mut db8 = Output::new(p.PIN_10, Level::Low);
    // let mut db9 = Output::new(p.PIN_11, Level::Low);
    // let mut db10 = Output::new(p.PIN_12, Level::Low);
    // let mut db11 = Output::new(p.PIN_13, Level::Low);
    // let mut db12 = Output::new(p.PIN_16, Level::Low);
    // let mut db13 = Output::new(p.PIN_17, Level::Low);
    // let mut db14 = Output::new(p.PIN_18, Level::Low);
    // let mut db15 = Output::new(p.PIN_19, Level::Low);

    let mut reset = Output::new(p.PIN_22, Level::Low);
    let mut cs = Output::new(p.PIN_18, Level::High);
    let mut dc = Output::new(p.PIN_19, Level::High);
    let mut rd = Output::new(p.PIN_20, Level::High);
    let mut wr = Output::new(p.PIN_21, Level::High);

    // reset
    reset.set_low();
    Timer::after(Duration::from_millis(10)).await;
    reset.set_high();
    Timer::after(Duration::from_millis(120)).await;
    info!("Begin");

    // -> COLOR MODE
    rd.set_high();
    wr.set_high();
    cs.set_low();

    let command: u8 = 0x3A;

    // Command
    dc.set_low();
    wr.set_low();

    write_2bytes!(command, db0, db1, db2, db3, db4, db5, db6, db7,);
    wr.set_high();

    // Data
    let data: u8 = 0b01010101;

    dc.set_high();

    wr.set_low();
    write_2bytes!(data, db0, db1, db2, db3, db4, db5, db6, db7,);
    wr.set_high();

    cs.set_high();

    // -> SLEEP OUT
    rd.set_high();
    wr.set_high();
    cs.set_low();

    let command: u8 = 0x11;

    // Command
    dc.set_low();
    wr.set_low();

    write_2bytes!(command, db0, db1, db2, db3, db4, db5, db6, db7,);
    wr.set_high();

    cs.set_high();

    // -> DISPLAY INVERSION OFF
    rd.set_high();
    wr.set_high();
    cs.set_low();

    let command: u8 = 0x20;

    // Command
    dc.set_low();
    wr.set_low();

    write_2bytes!(command, db0, db1, db2, db3, db4, db5, db6, db7,);
    wr.set_high();

    cs.set_high();

    // -> MEMORY ACCESS CONTROL
    rd.set_high();
    wr.set_high();
    cs.set_low();

    let command: u8 = 0x36;

    // Command
    dc.set_low();
    wr.set_low();

    write_2bytes!(command, db0, db1, db2, db3, db4, db5, db6, db7,);
    wr.set_high();

    // Data
    let data: u8 = 0b10001000;
    dc.set_high();

    wr.set_low();
    write_2bytes!(data, db0, db1, db2, db3, db4, db5, db6, db7,);
    wr.set_high();

    cs.set_high();

    // -> NORMAL DISPALY MODE
    rd.set_high();
    wr.set_high();
    cs.set_low();

    let command: u8 = 0x13;

    // Command
    dc.set_low();
    wr.set_low();

    write_2bytes!(command, db0, db1, db2, db3, db4, db5, db6, db7,);
    wr.set_high();

    cs.set_high();

    // -> DISPLAY ON
    rd.set_high();
    wr.set_high();
    cs.set_low();

    let command: u8 = 0x29;

    // Command
    dc.set_low();
    wr.set_low();

    write_2bytes!(command, db0, db1, db2, db3, db4, db5, db6, db7,);
    wr.set_high();

    cs.set_high();

    // -> IDDLE OFF
    rd.set_high();
    wr.set_high();
    cs.set_low();

    let command: u8 = 0x38;

    // Command
    dc.set_low();
    wr.set_low();

    write_2bytes!(command, db0, db1, db2, db3, db4, db5, db6, db7,);
    wr.set_high();

    cs.set_high();

    // -> COLUMN ADDRESS SET
    rd.set_high();
    wr.set_high();
    cs.set_low();

    let command: u8 = 0x2A;

    // Command
    dc.set_low();
    wr.set_low();

    write_2bytes!(command, db0, db1, db2, db3, db4, db5, db6, db7,);
    wr.set_high();

    // Data
    let start: u16 = 10;
    let end: u16 = 19;
    let data = [
        (start >> 8) as u8,
        (start & 0xff) as u8,
        (end >> 8) as u8,
        (end & 0xff) as u8,
    ];

    dc.set_high();

    for word in data.into_iter() {
        wr.set_low();
        write_2bytes!(word, db0, db1, db2, db3, db4, db5, db6, db7,);
        wr.set_high();
    }
    cs.set_high();

    // -> PAGE ADDRESS SET
    rd.set_high();
    wr.set_high();
    cs.set_low();

    let command: u8 = 0x2B;

    // Command
    dc.set_low();
    wr.set_low();

    write_2bytes!(command, db0, db1, db2, db3, db4, db5, db6, db7,);
    wr.set_high();

    // Data
    let start: u8 = 10;
    let end: u8 = 19;
    let data = [
        (start >> 8) as u8,
        (start & 0xff) as u8,
        (end >> 8) as u8,
        (end & 0xff) as u8,
    ];

    dc.set_high();

    for word in data.into_iter() {
        wr.set_low();
        write_2bytes!(word, db0, db1, db2, db3, db4, db5, db6, db7,);
        wr.set_high();
    }
    cs.set_high();

    // -> WRITE PIXEL DATA
    rd.set_high();
    wr.set_high();
    cs.set_low();

    let command: u8 = 0x2C;

    // Command
    dc.set_low();
    wr.set_low();

    write_2bytes!(command, db0, db1, db2, db3, db4, db5, db6, db7,);
    wr.set_high();

    // Data
    // let data: u8 = encode_rgb565_16bit(&(255, 255, 255));
    let data = [0b0000_0000u8; 200];

    dc.set_high();

    for d in data.into_iter() {
        wr.set_low();
        write_2bytes!(d, db0, db1, db2, db3, db4, db5, db6, db7,);
        wr.set_high();
    }

    cs.set_high();

    Timer::after(Duration::from_millis(5000)).await;
    // led.set_low();
    info!("End");

    // Timer::after(Duration::from_millis(500)).await;

    loop {}
}
