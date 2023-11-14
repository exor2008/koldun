#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]
#![feature(slice_flatten)]

use defmt::*;
use defmt_rtt as _;
use embassy_executor::Spawner;
use embassy_rp::bind_interrupts;
use embassy_rp::flash::Flash as RPFlash;
use embassy_rp::gpio::Pull;
use embassy_rp::gpio::{Input, Level, Output};
use embassy_rp::peripherals::{PIN_10, PIN_11, PIN_12, PIN_13, PIN_26, PIO0};
use embassy_rp::pio::{InterruptHandler, Pio};
use embassy_sync::blocking_mutex::raw::ThreadModeRawMutex;
use embassy_sync::channel::Channel;
use embassy_time::{Duration, Instant, Ticker, Timer};

use embedded_graphics::primitives::Rectangle;
use embedded_graphics::text::DecorationColor;
use embedded_graphics::{
    mono_font::{ascii::FONT_9X15_BOLD, iso_8859_5::FONT_9X18_BOLD, MonoTextStyle},
    // pixelcolor::BinaryColor,
    pixelcolor::Rgb565,
    prelude::*,
    text::Text,
};
// use heapless::Vec;
use koldun::game::events::{Buttons, Event, States};
// use koldun::game::colors;
use koldun::game::flash::FlashAccess;
use koldun::game::state_mashine::StateMachine;
use koldun::heap;
use koldun::ili9486::{pio_parallel::PioParallel8, Display, Ili9486, Order, PixelFormat};
use panic_probe as _;
// use tinytga::Tga;
use u8g2_fonts::fonts::u8g2_font_unifont_t_animals;
use u8g2_fonts::types::FontColor;
use u8g2_fonts::types::{HorizontalAlignment, VerticalPosition};
use u8g2_fonts::FontRenderer;
extern crate alloc;

bind_interrupts!(struct Irqs {
    PIO0_IRQ_0 => InterruptHandler<PIO0>;
});

const RATTLE_THRESHOLD: u64 = 100;
static CONTROL_CHANNEL: Channel<ThreadModeRawMutex, Event, 1> = Channel::new();

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    info!("Start");
    heap::init();
    let p = embassy_rp::init(Default::default());

    let mut reset = Output::new(p.PIN_22, Level::Low);

    let up = Input::new(p.PIN_13, Pull::Down);
    spawner.spawn(button_up_task(spawner, up)).unwrap();
    let down = Input::new(p.PIN_12, Pull::Down);
    spawner.spawn(button_down_task(spawner, down)).unwrap();
    let left = Input::new(p.PIN_11, Pull::Down);
    spawner.spawn(button_left_task(spawner, left)).unwrap();
    let right = Input::new(p.PIN_10, Pull::Down);
    spawner.spawn(button_right_task(spawner, right)).unwrap();
    let reset_btn = Input::new(p.PIN_26, Pull::Down);
    spawner
        .spawn(button_reset_btn_task(spawner, reset_btn))
        .unwrap();
    spawner.spawn(timer_task(spawner)).unwrap();

    // reset
    reset.set_low();
    Timer::after(Duration::from_millis(10)).await;
    reset.set_high();
    Timer::after(Duration::from_millis(120)).await;
    info!("Begin");

    let Pio {
        mut common, sm0, ..
    } = Pio::new(p.PIO0, Irqs);

    let pio_interface = PioParallel8::new(
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
        p.PIN_18,
        p.PIN_19,
        p.PIN_20,
        p.PIN_21,
    );

    let mut display = Ili9486::new(pio_interface);
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

    display.clear(Rgb565::RED).unwrap();

    let data = &[[0b1111_1000u8, 0b0001_1111u8]; 32 * 32];
    display
        .draw_data(
            Rectangle::new(Point::new(200, 200), Size::new(32, 32)),
            data.flatten(),
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

    Timer::after(Duration::from_millis(50)).await;

    let flash = RPFlash::new(p.FLASH, p.DMA_CH1);
    let flash = FlashAccess::new(flash);

    let mut sm = StateMachine::new(display, flash);
    sm.on_control(Event::Button(Buttons::Down(States::Pressed)))
        .await;
    sm.on_control(Event::Button(Buttons::Right(States::Pressed)))
        .await;

    // let mut c = 0;
    // let mut ticker = Ticker::every(Duration::from_hz(10));
    loop {
        let command = CONTROL_CHANNEL.receive().await;
        if let Event::Tick(_) = command {
        } else {
            info!("Heap used {}", heap::HEAP.used());
        }
        sm.on_control(command).await;
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
        // Timer::after(Duration::from_millis(10)).await;
    }
}

// fn render(color: BinaryColor, fg: Rgb565, bg: Rgb565) -> [u8; 2] {
//     match color.is_on() {
//         true => color_to_data(fg),
//         false => color_to_data(bg),
//     }
// }

// fn render_raw(color: BinaryColor, fg: [u8; 2], bg: [u8; 2]) -> [u8; 2] {
//     match color.is_on() {
//         true => fg,
//         false => bg,
//     }
// }

// fn color_to_data(color: Rgb565) -> [u8; 2] {
//     let b = color.to_ne_bytes();
//     [b[1], b[0]]
// }

#[embassy_executor::task]
async fn button_up_task(_spawner: Spawner, mut up: Input<'static, PIN_13>) {
    loop {
        up.wait_for_any_edge().await;
        let message = match up.is_high() {
            true => Event::Button(Buttons::Up(States::Pressed)),
            false => Event::Button(Buttons::Up(States::Released)),
        };
        CONTROL_CHANNEL.send(message).await
    }
}

#[embassy_executor::task]
async fn button_down_task(_spawner: Spawner, mut down: Input<'static, PIN_12>) {
    let mut last_press = Instant::now();

    loop {
        down.wait_for_any_edge().await;

        let now = Instant::now();
        if (now - last_press).as_millis() < RATTLE_THRESHOLD {
            continue;
        }
        last_press = now;

        let message = match down.is_high() {
            true => Event::Button(Buttons::Down(States::Pressed)),
            false => Event::Button(Buttons::Down(States::Released)),
        };
        CONTROL_CHANNEL.send(message).await
    }
}

#[embassy_executor::task]
async fn button_left_task(_spawner: Spawner, mut left: Input<'static, PIN_11>) {
    let mut last_press = Instant::now();

    loop {
        left.wait_for_any_edge().await;

        let now = Instant::now();
        if (now - last_press).as_millis() < RATTLE_THRESHOLD {
            continue;
        }
        last_press = now;

        let message = match left.is_high() {
            true => Event::Button(Buttons::Left(States::Pressed)),
            false => Event::Button(Buttons::Left(States::Released)),
        };
        CONTROL_CHANNEL.send(message).await
    }
}

#[embassy_executor::task]
async fn button_right_task(_spawner: Spawner, mut right: Input<'static, PIN_10>) {
    let mut last_press = Instant::now();
    loop {
        right.wait_for_any_edge().await;

        let now = Instant::now();
        if (now - last_press).as_millis() < RATTLE_THRESHOLD {
            continue;
        }
        last_press = now;

        let message = match right.is_high() {
            true => Event::Button(Buttons::Right(States::Pressed)),
            false => Event::Button(Buttons::Right(States::Released)),
        };
        CONTROL_CHANNEL.send(message).await
    }
}

#[embassy_executor::task]
async fn button_reset_btn_task(_spawner: Spawner, mut reset: Input<'static, PIN_26>) {
    let mut last_press = Instant::now();

    loop {
        reset.wait_for_any_edge().await;

        let now = Instant::now();
        if (now - last_press).as_millis() < RATTLE_THRESHOLD {
            continue;
        }
        last_press = now;

        let message = match reset.is_high() {
            true => Event::Button(Buttons::Reset(States::Pressed)),
            false => Event::Button(Buttons::Reset(States::Released)),
        };
        CONTROL_CHANNEL.send(message).await
    }
}

#[embassy_executor::task]
async fn timer_task(_spawner: Spawner) {
    let mut ticker = Ticker::every(Duration::from_hz(10));
    let mut tick: u128 = Default::default();
    loop {
        ticker.next().await;
        CONTROL_CHANNEL.send(Event::Tick(tick)).await;
        tick = tick.wrapping_add(1);
    }
}
