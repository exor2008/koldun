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
use hashbrown::HashMap;
use heapless::String;
use items::{exit::Exit, wizard::Wizard, Item};

extern crate alloc;

pub mod actions;
pub mod grid;
pub mod items;

pub struct Level1;
pub struct Level2;

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

    pub async fn _on_event<D>(&mut self, event: Event, display: &mut D) -> bool
    where
        D: GameDisplay + Display<u8, Color = Rgb565> + Send,
    {
        if self.block {
            match event {
                Event::Button(_) => return false,
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
        is_win
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
}

#[async_trait]
impl<D, F> State<D, F> for Level<Level1>
where
    D: GameDisplay + Display<u8, Color = Rgb565> + Send,
    F: Flash + Send + Sync,
{
    async fn on_event(&mut self, event: Event, display: &mut D) -> Option<Box<dyn State<D, F>>> {
        let is_win = self._on_event(event, display).await;
        match is_win {
            true => {
                self.tiles.clear();
                self.tiles.shrink_to_fit();
                Some(Box::new(Level::<Level1>::new()))
            }
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
