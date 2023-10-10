#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use defmt::*;
use defmt_rtt as _;
use embassy_executor::Spawner;
use embassy_rp::bind_interrupts;
use embassy_rp::flash::Flash as RPFlash;
use embassy_rp::gpio::{Level, Output};
use embassy_rp::peripherals::PIO0;
use embassy_rp::pio::{InterruptHandler, Pio};
use embassy_time::{Duration, Ticker, Timer};
use embedded_alloc::Heap;
use embedded_graphics::primitives::Rectangle;
use embedded_graphics::text::DecorationColor;
use embedded_graphics::{
    mono_font::{ascii::FONT_9X15_BOLD, iso_8859_5::FONT_9X18_BOLD, MonoTextStyle},
    pixelcolor::Rgb565,
    prelude::*,
    text::Text,
};
use heapless::Vec;
use koldun::game::flash::FlashAccess;
use koldun::game::state_mashine::states::ControlEvent;
use koldun::game::state_mashine::StateMachine;
use koldun::ili9431::{pio_parallel::PioParallel16, Display, Ili9431, Order, PixelFormat};
use panic_probe as _;
use tinytga::Tga;
use u8g2_fonts::fonts::u8g2_font_unifont_t_animals;
use u8g2_fonts::types::FontColor;
use u8g2_fonts::types::{HorizontalAlignment, VerticalPosition};
use u8g2_fonts::FontRenderer;
extern crate alloc;

#[global_allocator]
static HEAP: Heap = Heap::empty();

bind_interrupts!(struct Irqs {
    PIO0_IRQ_0 => InterruptHandler<PIO0>;
});

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    info!("Initializing heap...");
    {
        use core::mem::MaybeUninit;
        const HEAP_SIZE: usize = 1024 * 20; //kB
        static mut HEAP_MEM: [MaybeUninit<u8>; HEAP_SIZE] = [MaybeUninit::uninit(); HEAP_SIZE];
        unsafe { HEAP.init(HEAP_MEM.as_ptr() as usize, HEAP_SIZE) }
    }

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

    let pio_interface = PioParallel16::new(
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

    let mut display = Ili9431::new(pio_interface);
    display.set_pixel_format(PixelFormat::Bit16).await;
    display.sleep_out().await;
    display.inversion_off().await;
    display
        .memory_access_control(
            Order::Reverse,
            Order::Reverse,
            Order::Reverse,
            Order::default(),
            Order::default(),
            Order::Reverse,
        )
        .await;
    display.norma_display_mode().await;
    display.display_on().await;
    display.idle_mode_off().await;

    display.clear(Rgb565::BLACK).unwrap();

    let data = include_bytes!("./face.tga");
    let tga: Tga<Rgb565> = Tga::from_slice(data).unwrap();

    let pix: Vec<_, { 32 * 32 }> = tga.pixels().map(|p| convert(p)).collect();
    display
        .draw_data(
            Rectangle::new(Point::new(10, 10), Size::new(32, 32)),
            pix.as_slice(),
        )
        .await;

    let data = include_bytes!("./wizard.tga");
    let tga: Tga<Rgb565> = Tga::from_slice(data).unwrap();

    let mut pix: Vec<_, { 32 * 32 }> = tga.pixels().map(|p| convert(p)).collect();
    pix.reverse();
    display
        .draw_data(
            Rectangle::new(Point::new(30, 15), Size::new(32, 32)),
            pix.as_slice(),
        )
        .await;

    let style = MonoTextStyle::new(&FONT_9X15_BOLD, Rgb565::CSS_DARK_BLUE);
    Text::new("Hello Rust! FONT_10X20", Point::new(5, 50), style)
        .draw(&mut display)
        .unwrap();

    let mut style = MonoTextStyle::new(&FONT_9X18_BOLD, Rgb565::CSS_GREEN);
    style.background_color = Some(Rgb565::CSS_BLANCHED_ALMOND);
    style.underline_color = DecorationColor::Custom(Rgb565::CSS_TURQUOISE);

    Text::new("Хэллоу, пьяный волшкебник! ", Point::new(30, 100), style)
        .draw(&mut display)
        .unwrap();

    let font = FontRenderer::new::<u8g2_font_unifont_t_animals>();

    font.render_aligned(
        "animal icons 42",
        display.bounding_box().center() + Point::new(0, 16),
        VerticalPosition::Baseline,
        HorizontalAlignment::Center,
        FontColor::Transparent(Rgb565::CSS_AQUAMARINE),
        &mut display,
    )
    .unwrap();

    let flash = RPFlash::new(p.FLASH, p.DMA_CH1);
    let flash = FlashAccess::new(flash);
    let mut sm = StateMachine::new(display, flash);
    sm.on_control(ControlEvent::ButtonDown).await;
    Timer::after(Duration::from_millis(100)).await;
    sm.on_control(ControlEvent::ButtonDown).await;
    Timer::after(Duration::from_millis(100)).await;
    sm.on_control(ControlEvent::Down).await;
    Timer::after(Duration::from_millis(100)).await;
    sm.on_control(ControlEvent::ButtonDown).await;
    // let mut c = 0;
    // let mut ticker = Ticker::every(Duration::from_hz(10));
    loop {
        // c += 1;
        // c = if c >= 318 { 0 } else { c };

        // display
        //     .draw_data(
        //         Rectangle::new(Point::new(0, c), Size::new(32, 32)),
        //         &[0; 32 * 32],
        //     )
        //     .await;

        // display
        //     .draw_data(
        //         Rectangle::new(Point::new(0, c + 1), Size::new(32, 32)),
        //         p.as_slice(),
        //     )
        //     .await;

        // ticker.next().await;
    }
}

fn convert(p: Pixel<Rgb565>) -> u16 {
    let b = p.1.to_ne_bytes();
    (b[1] as u16) << 8 | (b[0]) as u16
}
