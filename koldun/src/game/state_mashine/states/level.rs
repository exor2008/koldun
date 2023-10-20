use crate::events::Event;
use crate::game::cell::Grid;
use crate::game::colors::*;
use crate::game::flash::Flash;
use crate::game::items::wizard::Wizard;
use crate::game::items::Item;
use crate::game::state_mashine::State;
use crate::game::tiles::*;
use crate::game::Drawable;
use crate::game::Tick;
use crate::game::{MAX_X, MAX_Y};
use crate::ili9486::Display;
use crate::ili9486::GameDisplay;
use alloc::boxed::Box;
use async_trait::async_trait;
use core::marker::PhantomData;
use defmt::info;
use embedded_graphics::pixelcolor::Rgb565;
use embedded_graphics::prelude::Point;
use heapless::FnvIndexMap;
extern crate alloc;

pub struct Level1;
pub struct Level2;

pub struct Level<L> {
    grid: Grid,
    tiles: FnvIndexMap<usize, [u8; 32 * 32 * 2], 32>,
    idx: PhantomData<L>,
}

impl<L> Level<L> {
    pub async fn redraw_all<D>(&mut self, display: &mut D)
    where
        D: GameDisplay + Display<u8, Color = Rgb565> + Send,
    {
        for x in 0..MAX_X {
            for y in 0..MAX_Y {
                let img_id = self.grid[y][x].tile_id();
                let data = self.tiles.get(&img_id).expect("Unknown img_id");
                display
                    .draw_tile(Point::new(32 * x as i32, 32 * y as i32), data)
                    .await;
            }
        }
    }

    pub async fn tick<D>(&mut self, time: u128, display: &mut D)
    where
        D: GameDisplay + Display<u8, Color = Rgb565> + Send,
    {
        for x in 0..MAX_X {
            for y in 0..MAX_Y {
                let redraw = self.grid[y][x].tick(time);
                if redraw {
                    let img_id = self.grid[y][x].tile_id();
                    let data = self.tiles.get(&img_id).expect("Unknown img_id");
                    display
                        .draw_tile(Point::new(32 * x as i32, 32 * y as i32), data)
                        .await;
                }
            }
        }
    }
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
        let char: Item<Wizard> = Item::new(Point::new(7, 7), 1, Tile::wizard1_id());
        let mut grid: Grid = level.into();
        let cell = &mut grid[5][10];
        cell.set_item(Box::new(char));
        ///////////

        Level {
            grid,
            tiles,
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
        match event {
            Event::Tick(time) => self.tick(time, display).await,
            _ => (),
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

        self.redraw_all(display).await
    }
}
