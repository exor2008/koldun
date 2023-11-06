use super::{
    level::{grid::Grid, level1::Level1, Level, Levels},
    State,
};
use crate::{
    game::{
        colors,
        events::{Buttons, Event, States},
        flash::Flash,
    },
    ili9486::{Display, GameDisplay},
};
use alloc::boxed::Box;
use async_trait::async_trait;
use defmt::info;
use embedded_graphics::{pixelcolor::Rgb565, prelude::Point};
use heapless::Vec;

extern crate alloc;

pub const MAX_COMMANDS: usize = 32;

#[derive(Clone, defmt::Format, Debug)]
pub enum SpellCommands {
    Left,
    Right,
    Up,
    Down,
    In,
    Out,
}

pub struct Spell {
    grid: Grid,
    level: Levels,
    commands: Vec<SpellCommands, MAX_COMMANDS>,
}

impl Spell {
    pub fn from_grid(grid: &mut Grid, level: Levels) -> Self {
        let grid = Grid::new_from(grid);
        let commands: Vec<SpellCommands, MAX_COMMANDS> = Vec::new();
        Spell {
            grid,
            level,
            commands,
        }
    }
}

#[async_trait]
impl<D, F> State<D, F> for Spell
where
    D: GameDisplay + Display<u8, Color = Rgb565> + Send,
    F: Flash + Send + Sync,
{
    async fn on_event(&mut self, event: Event, _display: &mut D) -> Option<Box<dyn State<D, F>>> {
        let mut commands: Vec<SpellCommands, MAX_COMMANDS> =
            Vec::from_slice(self.commands.as_slice()).unwrap();

        commands.push(SpellCommands::Right).unwrap();

        match event {
            Event::Button(Buttons::Reset(States::Pressed)) => match self.level {
                Levels::Level1 => {
                    return Some(Box::new(Level::<Level1>::from_spell(
                        &mut self.grid,
                        commands,
                    )));
                }
            },
            _ => return None,
        }
    }

    async fn on_init(&mut self, display: &mut D, _flash: &mut F) {
        info!("Spell screen");
        display.clear(colors::WALL_BG).unwrap();

        display.draw_text(
            "Spell screen",
            Point::new(210, 100),
            colors::START_MENU_TILE,
            None,
        );
    }
}
