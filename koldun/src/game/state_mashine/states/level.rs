use crate::game::state_mashine::states::ControlEvent;
use crate::game::state_mashine::State;
use alloc::boxed::Box;
use defmt::info;
extern crate alloc;

pub struct Level {
    level: [[u16; 10]; 7],
}

impl Level {
    pub fn new() -> Self {
        Level {
            level: [[016; 10]; 7],
        }
    }
}

impl State for Level {
    fn on_control(&mut self, _event: ControlEvent) -> Option<Box<dyn State>> {
        info!("Level working");
        None
    }
}
