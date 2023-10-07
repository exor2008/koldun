use crate::game::state_mashine::states::level::Level;
use crate::game::state_mashine::states::ControlEvent;
use crate::game::state_mashine::states::State;
use crate::ili9431::GameDisplay;
use alloc::boxed::Box;
use async_trait::async_trait;
use core::marker::Send;
use defmt::info;
use embedded_graphics::pixelcolor::Rgb565;
use embedded_graphics::prelude::RgbColor;
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
impl<D: GameDisplay + Send> State<D> for StartMenu {
    async fn on_control(
        &mut self,
        event: ControlEvent,
        display: &mut D,
    ) -> Option<Box<dyn State<D>>> {
        match event {
            ControlEvent::ButtonDown => {
                info!("Start menu working");
                None
            }
            _ => {
                info!("Level created");
                Some(Box::new(Level::new()))
            }
        }
    }

    async fn on_init(&mut self, display: &mut D) {
        info!("StartMenu Init");
        display.clear(Rgb565::MAGENTA).unwrap();
    }
}
