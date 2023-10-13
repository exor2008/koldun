use crate::game::flash::Flash;
use crate::game::state_mashine::states::level::{Level, Level1};
use crate::game::state_mashine::states::ControlEvent;
use crate::game::state_mashine::states::State;
use crate::ili9486::GameDisplay;
use alloc::boxed::Box;
use async_trait::async_trait;
use core::marker::Send;
use defmt::info;
use embedded_graphics::pixelcolor::Rgb666;
use embedded_graphics::prelude::Point;
use embedded_graphics::prelude::RgbColor;
use embedded_graphics::prelude::WebColors;
extern crate alloc;

pub enum StartMenuCommands {
    NewGame,
    Options,
}

pub struct StartMenu {
    commands: u16,
}

impl StartMenu {
    pub fn new() -> Self {
        StartMenu { commands: 8 }
    }
}

#[async_trait]
impl<D: GameDisplay + Send, F: Flash + Send + Sync> State<D, F> for StartMenu {
    async fn on_control(
        &mut self,
        event: ControlEvent,
        _display: &mut D,
    ) -> Option<Box<dyn State<D, F>>> {
        match event {
            ControlEvent::ButtonDown => {
                info!("Start menu working");
                None
            }
            _ => {
                info!("Level created");
                Some(Box::new(Level::<Level1>::new()))
                // None
            }
        }
    }

    async fn on_init(&mut self, display: &mut D, flash: &mut F) {
        info!("StartMenu Init");
        display.clear(Rgb666::new(4, 1, 10).into()).unwrap();

        display.draw_text(
            "KOLDUN the Game",
            Point::new(50, 50),
            Rgb666::new(32, 55, 70).into(),
            None,
        );

        display.draw_text(
            "New game",
            Point::new(50, 70),
            Rgb666::new(65, 110, 143).into(),
            Some(Rgb666::new(11, 12, 43).into()),
        );

        display.draw_text(
            "Continue",
            Point::new(50, 85),
            Rgb666::new(65, 110, 143).into(),
            None,
        );

        display.draw_text(
            "Options",
            Point::new(50, 100),
            Rgb666::new(65, 110, 143).into(),
            None,
        );
    }
}
