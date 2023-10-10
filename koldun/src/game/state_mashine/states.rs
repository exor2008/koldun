use crate::game::flash::Flash;
use crate::ili9431::GameDisplay;
use alloc::boxed::Box;
use async_trait::async_trait;
extern crate alloc;

pub mod initial;
pub mod level;
pub mod start_menu;

pub enum ControlEvent {
    Up,
    Down,
    ButtonDown,
    ButtonUp,
}

#[async_trait]
pub trait State<D: GameDisplay, F: Flash> {
    async fn on_control(
        &mut self,
        event: ControlEvent,
        display: &mut D,
    ) -> Option<Box<dyn State<D, F>>>;

    async fn on_init(&mut self, display: &mut D, flash: &mut F);
}
