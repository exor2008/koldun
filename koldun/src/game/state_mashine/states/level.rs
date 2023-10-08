use crate::game::state_mashine::states::ControlEvent;
use crate::game::state_mashine::State;
use crate::ili9431::GameDisplay;
use alloc::boxed::Box;
use async_trait::async_trait;
use defmt::info;
use embedded_graphics::pixelcolor::Rgb565;
use embedded_graphics::prelude::*;
use embedded_graphics::prelude::{Point, RgbColor, Size};
use embedded_graphics::primitives::Rectangle;
use heapless::Vec;
use tinytga::Tga;
extern crate alloc;

pub struct Level {
    level: [[u16; 7]; 10],
}

impl Level {
    pub fn new() -> Self {
        let mut level = [[016; 7]; 10];
        level[5][5] = 1;
        level[0][2] = 1;
        level[7][6] = 1;
        level[3][3] = 1;

        level[5][2] = 2;
        level[9][1] = 2;
        Level { level }
    }
}

#[async_trait]
impl<D: GameDisplay + Send> State<D> for Level {
    async fn on_control(
        &mut self,
        _event: ControlEvent,
        _display: &mut D,
    ) -> Option<Box<dyn State<D>>> {
        info!("Level working");
        None
    }
    async fn on_init(&mut self, display: &mut D) {
        info!("Level Init");
        display.clear(Rgb565::GREEN).unwrap();
        let bush1 = include_bytes!("..\\..\\..\\Bush1.tga");
        let bush2 = include_bytes!("..\\..\\..\\Bush2.tga");
        let grass = include_bytes!("..\\..\\..\\Grass.tga");

        let bush1: Tga<Rgb565> = Tga::from_slice(bush1).unwrap();
        let bush2: Tga<Rgb565> = Tga::from_slice(bush2).unwrap();
        let grass: Tga<Rgb565> = Tga::from_slice(grass).unwrap();

        let mut bush1: Vec<_, { 32 * 32 }> = bush1.pixels().map(|p| convert(p)).collect();
        let mut bush2: Vec<_, { 32 * 32 }> = bush2.pixels().map(|p| convert(p)).collect();
        let mut grass: Vec<_, { 32 * 32 }> = grass.pixels().map(|p| convert(p)).collect();

        bush2.reverse();
        grass.reverse();
        bush1.reverse();

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
                            .await
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
                            .await
                    }
                    _ => {
                        // info!("{} {}", x as i32, y as i32);
                        display
                            .draw_data(
                                Rectangle::new(
                                    Point::new((x * 32) as i32, (y * 32) as i32),
                                    Size::new(32, 32),
                                ),
                                grass.as_slice(),
                            )
                            .await
                    }
                }
            }
        }
    }
}

fn convert(p: Pixel<Rgb565>) -> u16 {
    let b = p.1.to_ne_bytes();
    (b[1] as u16) << 8 | (b[0]) as u16
}
