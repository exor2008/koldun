use crate::game::state_mashine::states::start_menu::StartMenu;
use crate::game::state_mashine::states::{ControlEvent, State};
use crate::ili9431::GameDisplay;
use alloc::boxed::Box;
use core::marker::Send;

use self::states::init::Init;
extern crate alloc;

pub mod states;

pub struct StateMachine<D: GameDisplay> {
    state: Box<dyn State<D>>,
    display: D,
}

impl<D: GameDisplay + Send> StateMachine<D> {
    pub fn new(display: D) -> StateMachine<D>
    where
        D: GameDisplay + Send + 'static,
    {
        let state: Box<Init> = Box::new(Init::new());
        let state = state as Box<dyn State<D>>;
        StateMachine { state, display }
    }

    pub async fn on_control(&mut self, event: ControlEvent) {
        if let Some(mut state) = self.state.on_control(event, &mut self.display).await {
            state.on_init(&mut self.display).await;
            self.state = state;
        }
    }
}
