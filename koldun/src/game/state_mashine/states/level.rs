use crate::game::state_mashine::states::ControlEvent;
use crate::game::state_mashine::State;
use crate::ili9431::GameDisplay;
use alloc::boxed::Box;
use async_trait::async_trait;
use defmt::info;
use embedded_graphics::pixelcolor::Rgb565;
use embedded_graphics::prelude::RgbColor;
extern crate alloc;

pub struct Level {
    level: [[u16; 10]; 7],
}

impl Level {
    pub fn new() -> Self {
        Level {
            level: [[016; 10]; 7],
        }
    }
}

#[async_trait]
impl<D: GameDisplay + Send> State<D> for Level {
    async fn on_control(
        &mut self,
        event: ControlEvent,
        display: &mut D,
    ) -> Option<Box<dyn State<D>>> {
        info!("Level working");
        Some(Box::new(Level::new()))
        // state = None;
    }
    async fn on_init(&mut self, display: &mut D) {
        info!("Level Init");
        display.clear(Rgb565::GREEN).unwrap();
    }
}
