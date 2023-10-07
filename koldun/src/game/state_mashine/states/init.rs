use crate::game::state_mashine::states::ControlEvent;
use crate::game::state_mashine::StartMenu;
use crate::game::state_mashine::State;
use crate::ili9431::GameDisplay;
use alloc::boxed::Box;
use async_trait::async_trait;
use defmt::info;
extern crate alloc;

pub struct Init {}

impl Init {
    pub fn new() -> Self {
        Init {}
    }
}

#[async_trait]
impl<D: GameDisplay + Send> State<D> for Init {
    async fn on_control(
        &mut self,
        _event: ControlEvent,
        _display: &mut D,
    ) -> Option<Box<dyn State<D>>> {
        info!("Init State");
        Some(Box::new(StartMenu::new()))
    }

    async fn on_init(&mut self, _display: &mut D) {}
}
