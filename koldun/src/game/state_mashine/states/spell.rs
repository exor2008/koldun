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

extern crate alloc;

pub struct Spell {
    grid: Grid,
    level: Levels,
}

impl Spell {
    pub fn from_grid(grid: &mut Grid, level: Levels) -> Self {
        let grid = Grid::new_from(grid);
        Spell { grid, level }
    }
}

#[async_trait]
impl<D, F> State<D, F> for Spell
where
    D: GameDisplay + Display<u8, Color = Rgb565> + Send,
    F: Flash + Send + Sync,
{
    async fn on_event(&mut self, event: Event, _display: &mut D) -> Option<Box<dyn State<D, F>>> {
        match event {
            Event::Button(Buttons::Reset(States::Pressed)) => match self.level {
                Levels::Level1 => {
                    return Some(Box::new(Level::<Level1>::from_grid(&mut self.grid)))
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
