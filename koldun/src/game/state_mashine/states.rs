use crate::ili9431::GameDisplay;
use alloc::boxed::Box;
use async_trait::async_trait;
extern crate alloc;

pub mod init;
pub mod level;
pub mod start_menu;

pub enum ControlEvent {
    Up,
    Down,
    ButtonDown,
    ButtonUp,
}

#[async_trait]
pub trait State<D: GameDisplay> {
    async fn on_control(
        &mut self,
        event: ControlEvent,
        display: &mut D,
    ) -> Option<Box<dyn State<D>>>;

    async fn on_init(&mut self, display: &mut D);
}
