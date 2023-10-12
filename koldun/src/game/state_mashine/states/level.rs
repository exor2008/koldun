use core::marker::PhantomData;

use crate::game::flash::Flash;
use crate::game::state_mashine::states::ControlEvent;
use crate::game::state_mashine::State;
use crate::ili9486::GameDisplay;
use alloc::boxed::Box;
use async_trait::async_trait;
use defmt::info;
use embedded_graphics::pixelcolor::Rgb565;
use embedded_graphics::prelude::{Point, RgbColor, Size};
use embedded_graphics::primitives::Rectangle;
use heapless::FnvIndexMap;
extern crate alloc;

#[derive(Clone, Copy)]
pub enum TilesSize {
    Grass = 0x029b,
    Bush1 = 0x03bb,
    Bush2 = 0x03bc,
}

#[derive(Clone, Copy)]
pub enum TilesOffset {
    Grass = 0x0000,
    Bush1 = 0x1000,
    Bush2 = 0x2000,
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
            [0, 0, 1, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 1, 0, 0, 1, 1, 1, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 1, 0, 0],
            [0, 0, 0, 0, 1, 0, 2, 0, 0, 0],
            [0, 2, 0, 0, 0, 1, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 2, 0, 1, 0, 2, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 2, 0],
            [0, 1, 0, 0, 0, 0, 1, 1, 0, 0],
            [0, 0, 1, 0, 0, 0, 0, 2, 0, 0],
            [0, 0, 0, 1, 0, 0, 0, 1, 0, 0],
            [0, 0, 0, 1, 1, 0, 2, 0, 2, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        ];

        let tiles: FnvIndexMap<Tiles, &'a [u8], 32> = FnvIndexMap::new();
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
        _event: ControlEvent,
        _display: &mut D,
    ) -> Option<Box<dyn State<D, F>>> {
        info!("Level working");
        None
    }
    async fn on_init(&mut self, display: &mut D, flash: &mut F) {
        info!("Level1 Init");
        display.clear(Rgb565::GREEN).unwrap();

        const GRASS_SIZE: usize = TilesSize::Grass as usize;
        let grass = flash
            .load_tga::<GRASS_SIZE, { GRASS_SIZE * 4 }>(TilesOffset::Grass.into())
            .await;
        let grass = D::tga_to_data(grass.as_slice());

        const B1_SIZE: usize = TilesSize::Bush1 as usize;
        let bush1 = flash
            .load_tga::<B1_SIZE, { B1_SIZE * 4 }>(TilesOffset::Bush1.into())
            .await;
        let bush1 = D::tga_to_data(bush1.as_slice());

        const B2_SIZE: usize = TilesSize::Bush1 as usize;
        let bush2 = flash
            .load_tga::<B2_SIZE, { B2_SIZE * 4 }>(TilesOffset::Bush2.into())
            .await;
        let bush2 = D::tga_to_data(bush2.as_slice());

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
                                bush1.as_slice(),
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
                                bush2.as_slice(),
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
                                grass.as_slice(),
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
