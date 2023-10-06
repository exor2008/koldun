pub mod level;
pub mod start_menu;
use alloc::boxed::Box;
extern crate alloc;

pub enum ControlEvent {
    Up,
    Down,
    ButtonDown,
    ButtonUp,
}

pub trait State {
    fn on_control(&mut self, event: ControlEvent) -> Option<Box<dyn State>>;
}
