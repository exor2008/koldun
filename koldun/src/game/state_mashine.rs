use crate::control::Controls;
use crate::game::flash::Flash;
use crate::game::state_mashine::states::initial::Initial;
use crate::game::state_mashine::states::State;
use crate::ili9486::Display;
use crate::ili9486::GameDisplay;
use alloc::boxed::Box;
use core::marker::Send;
use embedded_graphics::pixelcolor::Rgb565;
extern crate alloc;

pub mod states;

pub struct StateMachine<D: GameDisplay, F: Flash> {
    state: Box<dyn State<D, F>>,
    display: D,
    flash: F,
}

impl<D, F> StateMachine<D, F>
where
    D: GameDisplay + Send + Display<u8, Color = Rgb565> + 'static,
    F: Flash + Send + Sync,
{
    pub fn new(display: D, flash: F) -> StateMachine<D, F> {
        let state: Box<Initial> = Box::new(Initial::new());
        let state = state as Box<dyn State<D, F>>;
        StateMachine {
            state,
            display,
            flash,
        }
    }

    pub async fn on_control(&mut self, event: Controls) {
        if let Some(mut state) = self.state.on_control(event, &mut self.display).await {
            state.on_init(&mut self.display, &mut self.flash).await;
            self.state = state;
        }
    }
}
