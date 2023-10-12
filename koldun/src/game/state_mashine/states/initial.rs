use crate::game::flash::Flash;
use crate::game::state_mashine::states::start_menu::StartMenu;
use crate::game::state_mashine::states::ControlEvent;
use crate::game::state_mashine::State;
use crate::ili9486::GameDisplay;
use alloc::boxed::Box;
use async_trait::async_trait;
use defmt::info;
extern crate alloc;

pub struct Initial {}

impl Initial {
    pub fn new() -> Self {
        Initial {}
    }
}

#[async_trait]
impl<D: GameDisplay + Send, F: Flash + Send + Sync> State<D, F> for Initial {
    async fn on_control(
        &mut self,
        _event: ControlEvent,
        _display: &mut D,
    ) -> Option<Box<dyn State<D, F>>> {
        info!("Init State");
        Some(Box::new(StartMenu::new()))
    }

    async fn on_init(&mut self, _display: &mut D, _flash: &mut F) {}
}
