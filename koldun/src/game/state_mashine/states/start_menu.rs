use crate::game::state_mashine::states::level::Level;
use crate::game::state_mashine::states::ControlEvent;
use crate::game::state_mashine::states::State;
use alloc::boxed::Box;
use defmt::info;
extern crate alloc;

pub enum StartMenuCommands {
    NewGame,
    Options,
}

pub struct StartMenu {
    commands: u16,
}

impl StartMenu {
    pub fn new() -> Self {
        StartMenu { commands: 8 }
    }
}

impl State for StartMenu {
    fn on_control(&mut self, event: ControlEvent) -> Option<Box<dyn State>> {
        match event {
            ControlEvent::ButtonDown => {
                info!("Start menu working");
                None
            }
            _ => {
                info!("Level created");
                Some(Box::new(Level::new()))
            }
        }
    }
}
