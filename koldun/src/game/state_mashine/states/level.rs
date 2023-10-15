use crate::control::Controls;
use crate::game::flash::Flash;
use crate::game::state_mashine::State;
use crate::heap;
use crate::ili9486::GameDisplay;
use alloc::boxed::Box;
use async_trait::async_trait;
use core::marker::PhantomData;
use defmt::info;
use embedded_graphics::prelude::{Point, Size};
use embedded_graphics::primitives::Rectangle;
use heapless::FnvIndexMap;
extern crate alloc;

#[derive(Clone, Copy)]
pub enum TilesSize {
    Grass = 0x029b,
    Bush1 = 0x03bb,
    Bush2 = 0x03bc,
    Floor = 0x02ab,
    Wall1 = 0x0733,
    Wall2 = 0x07bb,
    Wall3 = 0x0813,
    Wall4 = 0x065b,
}

#[derive(Clone, Copy)]
pub enum TilesOffset {
    Grass = 0x0000,
    Bush1 = 0x1000,
    Bush2 = 0x2000,
    Floor = 0x3000,
    Wall1 = 0x4000,
    Wall2 = 0x5000,
    Wall3 = 0x6000,
    Wall4 = 0x7000,
}

#[derive(Clone, Copy)]
pub enum Tiles {
    Grass,
    Bush1,
    Bush2,
}

pub struct Level1;
pub struct Level2;

pub struct Level<'a, L> {
    level: [[u16; 10]; 15],
    tiles: FnvIndexMap<Tiles, &'a [u8], 32>,
    idx: PhantomData<L>,
}

impl<'a> Level<'a, Level1> {
    pub fn new() -> Self {
        let level = [
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 4, 2, 3, 2, 3, 4, 0, 0],
            [0, 0, 3, 0, 0, 0, 0, 1, 0, 0],
            [0, 0, 1, 0, 0, 0, 0, 2, 0, 0],
            [0, 0, 2, 0, 0, 0, 0, 3, 0, 0],
            [0, 0, 4, 2, 1, 0, 3, 4, 0, 0],
            [0, 0, 3, 0, 0, 0, 0, 1, 0, 0],
            [0, 0, 1, 0, 0, 0, 0, 2, 0, 0],
            [0, 0, 2, 0, 1, 2, 3, 4, 0, 0],
            [0, 0, 1, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 1, 0, 3, 0, 0, 0, 0, 0],
            [0, 0, 4, 2, 2, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        ];

        let tiles: FnvIndexMap<Tiles, &'a [u8], 32> = FnvIndexMap::new();
        //
        Level {
            level,
            tiles,
            idx: Default::default(),
        }
    }
}

#[async_trait]
impl<'a, D: GameDisplay + Send, F: Flash + Send + Sync> State<D, F> for Level<'a, Level1> {
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

        const FLOOR_SIZE: usize = TilesSize::Floor as usize;
        let floor = flash
            .load_tga::<{ FLOOR_SIZE / 4 + 1 }, { FLOOR_SIZE }>(TilesOffset::Floor.into())
            .await;
        let floor = D::tga_to_data(floor.as_slice());

        const W1_SIZE: usize = TilesSize::Wall1 as usize;
        let wall1 = flash
            .load_tga::<{ W1_SIZE / 4 + 1 }, { W1_SIZE }>(TilesOffset::Wall1.into())
            .await;
        let wall1 = D::tga_to_data(wall1.as_slice());

        const W2_SIZE: usize = TilesSize::Wall2 as usize;
        let wall2 = flash
            .load_tga::<{ W2_SIZE / 4 + 1 }, { W2_SIZE }>(TilesOffset::Wall2.into())
            .await;
        let wall2 = D::tga_to_data(wall2.as_slice());

        const W3_SIZE: usize = TilesSize::Wall3 as usize;
        let wall3 = flash
            .load_tga::<{ W3_SIZE / 4 + 1 }, { W3_SIZE }>(TilesOffset::Wall3.into())
            .await;
        let wall3 = D::tga_to_data(wall3.as_slice());

        const W4_SIZE: usize = TilesSize::Wall4 as usize;
        let wall4 = flash
            .load_tga::<{ W4_SIZE / 4 + 1 }, { W4_SIZE }>(TilesOffset::Wall4.into())
            .await;
        let wall4 = D::tga_to_data(wall4.as_slice());

        for x in 0..self.level.len() {
            for y in 0..self.level[0].len() {
                match self.level[x][y] {
                    1 => {
                        display
                            .draw_data(
                                Rectangle::new(
                                    Point::new((x * 32) as i32, (y * 32) as i32),
                                    Size::new(32, 32),
                                ),
                                wall1.as_slice(),
                            )
                            .await;
                    }
                    2 => {
                        display
                            .draw_data(
                                Rectangle::new(
                                    Point::new((x * 32) as i32, (y * 32) as i32),
                                    Size::new(32, 32),
                                ),
                                wall2.as_slice(),
                            )
                            .await;
                    }
                    3 => {
                        display
                            .draw_data(
                                Rectangle::new(
                                    Point::new((x * 32) as i32, (y * 32) as i32),
                                    Size::new(32, 32),
                                ),
                                wall3.as_slice(),
                            )
                            .await;
                    }
                    4 => {
                        display
                            .draw_data(
                                Rectangle::new(
                                    Point::new((x * 32) as i32, (y * 32) as i32),
                                    Size::new(32, 32),
                                ),
                                wall4.as_slice(),
                            )
                            .await;
                    }

                    _ => {
                        display
                            .draw_data(
                                Rectangle::new(
                                    Point::new((x * 32) as i32, (y * 32) as i32),
                                    Size::new(32, 32),
                                ),
                                floor.as_slice(),
                            )
                            .await;
                    }
                }
            }
        }
    }
}

impl From<TilesOffset> for usize {
    fn from(value: TilesOffset) -> Self {
        value as Self
    }
}
