use super::items::{exit::Exit, wizard::Wizard, Item};
use super::{Grid, Level, Levels};
use crate::game::colors::*;
use crate::game::events::Event;
use crate::game::flash::Flash;
use crate::game::state_mashine::states::spell::Spell;
use crate::game::state_mashine::states::State;
use crate::game::tiles::Tile;
use crate::game::{MAX_X, MAX_Y};
use crate::ili9486::{Display, GameDisplay};
use alloc::boxed::Box;
use async_trait::async_trait;
use defmt::info;
use embedded_graphics::pixelcolor::Rgb565;
use embedded_graphics::prelude::Point;
use hashbrown::HashMap;

extern crate alloc;

pub struct Level1;

impl Level<Level1> {
    pub fn new() -> Self {
        let level: [[usize; MAX_X]; MAX_Y] = [
            [0, 0, 0, 0, 36, 0, 0, 0, 0, 38, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 37, 0, 0, 0, 0, 36, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 38, 0, 0, 0, 0, 36, 0, 0, 0, 0, 0],
            [42, 0, 0, 0, 36, 0, 0, 0, 0, 37, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 37, 0, 0, 0, 0, 38, 0, 0, 0, 0, 0],
            [0, 43, 0, 0, 38, 37, 4, 36, 37, 36, 0, 0, 0, 0, 0],
            [0, 42, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 7, 6],
            [0, 3, 41, 0, 3, 3, 2, 0, 0, 0, 0, 7, 6, 51, 50],
            [2, 3, 2, 3, 2, 3, 0, 0, 0, 0, 0, 6, 51, 50, 51],
            [42, 43, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 50, 51, 50],
        ];

        let tiles: HashMap<usize, [u8; 32 * 32 * 2]> = HashMap::with_capacity(24);

        ///////////
        let mut grid: Grid = level.into();

        let wizard: Item<Wizard> = Item::new(Point::new(10, 5), 1, Tile::wizard_idle1_id());
        let cell = &mut grid[5][10];
        cell.set_item(Box::new(wizard));

        let exit: Item<Exit> = Item::new(Point::new(10, 7), 0, Tile::exit_open_id());
        let cell = &mut grid[7][10];
        cell.set_item(Box::new(exit));
        ///////////

        Level {
            grid: grid,
            tiles,
            block: Default::default(),
            idx: Default::default(),
        }
    }

    pub fn from_grid(grid: &mut Grid) -> Self {
        let grid = Grid::new_from(grid);
        let tiles: HashMap<usize, [u8; 32 * 32 * 2]> = HashMap::with_capacity(24);

        Self {
            grid,
            tiles,
            block: Default::default(),
            idx: Default::default(),
        }
    }
}

#[async_trait]
impl<D, F> State<D, F> for Level<Level1>
where
    D: GameDisplay + Display<u8, Color = Rgb565> + Send,
    F: Flash + Send + Sync,
{
    async fn on_event(&mut self, event: Event, display: &mut D) -> Option<Box<dyn State<D, F>>> {
        let (is_win, is_spell) = self._on_event(event, display).await;

        match is_win {
            true => {
                self.tiles.clear();
                self.tiles.shrink_to_fit();
                return Some(Box::new(Level::<Level1>::new()));
            }
            false => (),
        }

        match is_spell {
            true => Some(Box::new(Spell::from_grid(&mut self.grid, Levels::Level1))),
            false => None,
        }
    }

    async fn on_init(&mut self, display: &mut D, _flash: &mut F) {
        info!("Level1 Init");

        self.tiles
            .insert(Tile::empty_id(), Tile::empty(WALL_FG, WALL_BG));
        self.tiles
            .insert(Tile::brick_wall1_id(), Tile::brick_wall1(WALL_FG, WALL_BG));
        self.tiles
            .insert(Tile::brick_wall2_id(), Tile::brick_wall2(WALL_FG, WALL_BG));
        self.tiles
            .insert(Tile::brick_wall3_id(), Tile::brick_wall3(WALL_FG, WALL_BG));

        self.tiles
            .insert(Tile::stone1_id(), Tile::stone1(WALL_FG, WALL_BG));
        self.tiles
            .insert(Tile::stone2_id(), Tile::stone2(WALL_FG, WALL_BG));
        self.tiles
            .insert(Tile::stone3_id(), Tile::stone3(WALL_FG, WALL_BG));

        self.tiles
            .insert(Tile::debris1_id(), Tile::debris1(WALL_FG, WALL_BG));
        self.tiles
            .insert(Tile::debris2_id(), Tile::debris2(WALL_FG, WALL_BG));

        self.tiles
            .insert(Tile::tree_id(), Tile::tree(WALL_FG, WALL_BG));
        self.tiles
            .insert(Tile::trees_id(), Tile::trees(WALL_FG, WALL_BG));

        self.tiles
            .insert(Tile::ground1_id(), Tile::ground1(WALL_FG, WALL_BG));
        self.tiles
            .insert(Tile::ground2_id(), Tile::ground2(WALL_FG, WALL_BG));

        self.tiles
            .insert(Tile::door_open_id(), Tile::door_open(WALL_FG, WALL_BG));

        self.tiles.insert(
            Tile::wizard_idle1_id(),
            Tile::wizard_idle1(WIZARD_FG, WALL_BG),
        );
        self.tiles.insert(
            Tile::wizard_idle2_id(),
            Tile::wizard_idle2(WIZARD_FG, WALL_BG),
        );
        self.tiles
            .insert(Tile::wizard_up1_id(), Tile::wizard_up1(WIZARD_FG, WALL_BG));
        self.tiles
            .insert(Tile::wizard_up2_id(), Tile::wizard_up2(WIZARD_FG, WALL_BG));
        self.tiles.insert(
            Tile::wizard_down1_id(),
            Tile::wizard_down1(WIZARD_FG, WALL_BG),
        );
        self.tiles.insert(
            Tile::wizard_down2_id(),
            Tile::wizard_down2(WIZARD_FG, WALL_BG),
        );
        self.tiles.insert(
            Tile::wizard_left1_id(),
            Tile::wizard_left1(WIZARD_FG, WALL_BG),
        );
        self.tiles.insert(
            Tile::wizard_left2_id(),
            Tile::wizard_left2(WIZARD_FG, WALL_BG),
        );
        self.tiles.insert(
            Tile::wizard_right1_id(),
            Tile::wizard_right1(WIZARD_FG, WALL_BG),
        );
        self.tiles.insert(
            Tile::wizard_right2_id(),
            Tile::wizard_right2(WIZARD_FG, WALL_BG),
        );

        self.tiles
            .insert(Tile::exit_open_id(), Tile::exit_open(WIZARD_FG, WALL_BG));
        self.tiles.insert(
            Tile::exit_closed_id(),
            Tile::exit_closed(WIZARD_FG, WALL_BG),
        );

        self.redraw_all(display).await
    }
}
