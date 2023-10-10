use crate::game::flash::Flash;
use crate::game::state_mashine::states::initial::Initial;
use crate::game::state_mashine::states::{ControlEvent, State};
use crate::ili9431::GameDisplay;
use alloc::boxed::Box;
use core::marker::Send;
extern crate alloc;

pub mod states;

pub struct StateMachine<D: GameDisplay, F: Flash> {
    state: Box<dyn State<D, F>>,
    display: D,
    flash: F,
}

impl<D: GameDisplay + Send, F: Flash + Send + Sync> StateMachine<D, F> {
    pub fn new(display: D, flash: F) -> StateMachine<D, F>
    where
        D: GameDisplay + Send + 'static,
    {
        let state: Box<Initial> = Box::new(Initial::new());
        let state = state as Box<dyn State<D, F>>;
        StateMachine {
            state,
            display,
            flash,
        }
    }

    pub async fn on_control(&mut self, event: ControlEvent) {
        if let Some(mut state) = self.state.on_control(event, &mut self.display).await {
            state.on_init(&mut self.display, &mut self.flash).await;
            self.state = state;
        }
    }
}
