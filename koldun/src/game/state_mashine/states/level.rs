use crate::game::colors::*;
use crate::game::events::Event;
use crate::game::flash::Flash;
use crate::game::state_mashine::State;
use crate::game::tiles::*;
use crate::game::{MAX_X, MAX_Y};
use crate::ili9486::Display;
use crate::ili9486::GameDisplay;
use alloc::boxed::Box;
use async_trait::async_trait;
use core::fmt::Write;
use core::marker::PhantomData;
use defmt::info;
use embedded_graphics::pixelcolor::Rgb565;
use embedded_graphics::prelude::Point;
use grid::Grid;
use heapless::{FnvIndexMap, String};
use items::{wizard::Wizard, Item};
extern crate alloc;

pub mod actions;
pub mod grid;
pub mod items;

pub struct Level1;
pub struct Level2;

pub struct Level<L> {
    grid: Grid,
    tiles: FnvIndexMap<usize, [u8; 32 * 32 * 2], 32>,
    block: bool,
    idx: PhantomData<L>,
}

impl<L> Level<L> {
    pub async fn redraw_all<D>(&mut self, display: &mut D)
    where
        D: GameDisplay + Display<u8, Color = Rgb565> + Send,
    {
        for x in 0..MAX_X {
            for y in 0..MAX_Y {
                let img_id = self.grid.tile_id(x, y);

                let data = self.tiles.get(&img_id).expect(format_err(img_id).as_str());
                display
                    .draw_tile(Point::new(32 * x as i32, 32 * y as i32), data)
                    .await;
            }
        }
    }
}

fn format_err(img_id: usize) -> String<24> {
    let mut s: String<24> = String::new();
    write!(&mut s, "Unknown img_id: {}", img_id).unwrap();
    s
}

impl Level<Level1> {
    pub fn new() -> Self {
        let level: [[usize; MAX_X]; MAX_Y] = [
            [4, 2, 2, 3, 4, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
            [2, 1, 1, 1, 1, 2, 3, 4, 1, 1, 1, 1, 1, 1, 1],
            [2, 1, 1, 1, 2, 1, 1, 2, 1, 1, 1, 1, 1, 1, 1],
            [3, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
            [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
            [3, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
            [2, 1, 1, 1, 3, 1, 1, 2, 1, 1, 1, 1, 1, 1, 1],
            [3, 1, 1, 1, 3, 1, 1, 3, 1, 1, 1, 1, 1, 1, 1],
            [3, 1, 1, 1, 1, 3, 2, 4, 1, 1, 1, 1, 1, 1, 1],
            [4, 3, 3, 2, 4, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
        ];

        let tiles: FnvIndexMap<usize, [u8; 32 * 32 * 2], 32> = FnvIndexMap::new();

        ///////////
        let char: Item<Wizard> = Item::new(Point::new(10, 5), 1, Tile::wizard1_id());
        let mut grid: Grid = level.into();
        let cell = &mut grid[5][10];
        cell.set_item(Box::new(char));
        ///////////

        Level {
            grid: grid,
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
        if self.block {
            match event {
                Event::Button(_) => return None,
                _ => (),
            }
        }

        let requests = self.grid.on_event(&event);
        let (reactions, to_redraw, block) = self.grid.on_actions(requests);
        if let Some(block) = block {
            self.block = block
        };

        self.grid.on_reactions(reactions);

        // Redraw cells
        for request in to_redraw {
            let img_id = self.grid.tile_id(request.target.x, request.target.y);
            let data = self.tiles.get(&img_id).expect("Unknown img_id");
            display
                .draw_tile(
                    Point::new(
                        (TILE_SIZE_X as isize * request.target.x as isize + request.shift.x) as i32,
                        (TILE_SIZE_Y as isize * request.target.y as isize + request.shift.y) as i32,
                    ),
                    data,
                )
                .await;
        }

        None
    }

    async fn on_init(&mut self, display: &mut D, _flash: &mut F) {
        info!("Level1 Init");

        self.tiles
            .insert(Tile::floor_id(), Tile::floor(WALL_FG, WALL_BG))
            .unwrap();
        self.tiles
            .insert(Tile::wall1_id(), Tile::wall1(WALL_FG, WALL_BG))
            .unwrap();
        self.tiles
            .insert(Tile::wall2_id(), Tile::wall2(WALL_FG, WALL_BG))
            .unwrap();
        self.tiles
            .insert(Tile::wall3_id(), Tile::wall3(WALL_FG, WALL_BG))
            .unwrap();
        self.tiles
            .insert(Tile::wall4_id(), Tile::wall4(WALL_FG, WALL_BG))
            .unwrap();
        self.tiles
            .insert(Tile::wizard1_id(), Tile::wizard1(WIZARD_FG, WALL_BG))
            .unwrap();
        self.tiles
            .insert(Tile::wizard2_id(), Tile::wizard2(WIZARD_FG, WALL_BG))
            .unwrap();
        self.tiles
            .insert(
                Tile::wizard_left1_id(),
                Tile::wizard_left1(WIZARD_FG, WALL_BG),
            )
            .unwrap();
        self.tiles
            .insert(
                Tile::wizard_left2_id(),
                Tile::wizard_left2(WIZARD_FG, WALL_BG),
            )
            .unwrap();

        self.redraw_all(display).await
    }
}
