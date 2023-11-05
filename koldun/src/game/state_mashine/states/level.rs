use crate::game::events::{Buttons, Event, States};
use crate::game::tiles::*;
use crate::game::{MAX_X, MAX_Y};
use crate::ili9486::Display;
use crate::ili9486::GameDisplay;
use core::fmt::Write;
use core::marker::PhantomData;
use embedded_graphics::pixelcolor::Rgb565;
use embedded_graphics::prelude::Point;
use grid::Grid;
use hashbrown::HashMap;
use heapless::String;

pub mod actions;
pub mod grid;
pub mod items;
pub mod level1;

pub enum Levels {
    Level1,
}

pub struct Level<L> {
    grid: Grid,
    tiles: HashMap<usize, [u8; 32 * 32 * 2]>,
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

    pub async fn _on_event<D>(&mut self, event: Event, display: &mut D) -> (bool, bool)
    where
        D: GameDisplay + Display<u8, Color = Rgb565> + Send,
    {
        match event {
            Event::Button(Buttons::Reset(States::Pressed)) => return (false, true),
            _ => (),
        }

        if self.block {
            match event {
                Event::Button(_) => return (false, false),
                _ => (),
            }
        }

        let requests = self.grid.on_event(&event);
        let (reactions, to_redraw, block, is_win) = self.grid.on_actions(requests);
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
        (is_win, false)
    }
}

fn format_err(img_id: usize) -> String<24> {
    let mut s: String<24> = String::new();
    write!(&mut s, "Unknown img_id: {}", img_id).unwrap();
    s
}
