use crate::control::Controls;
use crate::game::colors;
use crate::game::flash::Flash;
use crate::game::state_mashine::State;
use crate::game::tiles::*;
use crate::ili9486::Display;
use crate::ili9486::GameDisplay;
use alloc::boxed::Box;
use async_trait::async_trait;
use core::marker::PhantomData;
use defmt::info;
use embedded_graphics::pixelcolor::Rgb565;
use embedded_graphics::prelude::Point;
use heapless::FnvIndexMap;
use heapless::Vec;
use koldun_macro_derive::load_tga;
extern crate alloc;

pub struct Level1;
pub struct Level2;

pub struct Level<L> {
    level: [[u8; 15]; 10],
    tiles: FnvIndexMap<u8, Vec<u8, { 32 * 32 * 2 }>, 32>,
    idx: PhantomData<L>,
}

impl<L> Level<L> {
    pub async fn redraw_all<D>(&mut self, display: &mut D)
    where
        D: GameDisplay + Display<u8, Color = Rgb565> + Send,
    {
        for x in 0..self.level.len() {
            for y in 0..self.level[0].len() {
                display
                    .draw_tile(
                        Point::new((y * 32) as i32, (x * 32) as i32),
                        self.tiles.get(&self.level[x][y]).unwrap(),
                    )
                    .await;
            }
        }
    }
}

impl Level<Level1> {
    pub fn new() -> Self {
        let level = [
            [4, 2, 2, 3, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [2, 0, 0, 0, 1, 2, 3, 4, 0, 0, 0, 0, 0, 0, 0],
            [2, 0, 0, 0, 2, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0],
            [3, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0],
            [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [3, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0],
            [2, 0, 0, 0, 3, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0],
            [3, 0, 0, 0, 3, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0],
            [3, 0, 0, 0, 1, 3, 2, 4, 0, 0, 0, 0, 0, 0, 0],
            [4, 3, 3, 2, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        ];

        let tiles: FnvIndexMap<u8, Vec<u8, { 32 * 32 * 2 }>, 32> = FnvIndexMap::new();

        Level {
            level,
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
    async fn on_control(
        &mut self,
        _event: Controls,
        _display: &mut D,
    ) -> Option<Box<dyn State<D, F>>> {
        info!("Level working");
        None
    }
    async fn on_init(&mut self, display: &mut D, flash: &mut F) {
        info!("Level1 Init");

        let tiles = &mut self.tiles;
        load_tga!(tiles, Floor, WALL_FG, WALL_BG);
        load_tga!(tiles, Wall1, WALL_FG, WALL_BG);
        load_tga!(tiles, Wall2, WALL_FG, WALL_BG);
        load_tga!(tiles, Wall3, WALL_FG, WALL_BG);
        load_tga!(tiles, Wall4, WALL_FG, WALL_BG);

        self.redraw_all(display).await;
    }
}
