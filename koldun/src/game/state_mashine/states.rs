use crate::game::{events::Event, flash::Flash};
use crate::ili9486::GameDisplay;
use alloc::boxed::Box;
use async_trait::async_trait;
extern crate alloc;

pub mod initial;
pub mod level;
pub mod spell;
pub mod start_menu;

#[async_trait]
pub trait State<D: GameDisplay, F: Flash> {
    async fn on_event(&mut self, event: Event, display: &mut D) -> Option<Box<dyn State<D, F>>>;

    async fn on_init(&mut self, display: &mut D, flash: &mut F);
}
