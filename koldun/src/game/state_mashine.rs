use crate::game::state_mashine::states::start_menu::StartMenu;
use crate::game::state_mashine::states::{ControlEvent, State};
use alloc::boxed::Box;
extern crate alloc;

pub mod states;

pub struct StateMachine {
    state: Box<dyn State>,
}

impl StateMachine {
    pub fn new() -> StateMachine {
        let state = Box::new(StartMenu::new());
        StateMachine { state }
    }

    pub fn on_control(&mut self, event: ControlEvent) {
        if let Some(state) = self.state.on_control(event) {
            self.state = state;
        }
    }
}
