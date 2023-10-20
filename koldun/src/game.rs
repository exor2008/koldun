use crate::ili9486::GameDisplay;
use embedded_graphics::prelude::Point;

pub mod cell;
pub mod colors;
pub mod flash;
pub mod items;
pub mod state_mashine;
pub mod tiles;

pub const MAX_X: usize = 15;
pub const MAX_Y: usize = 10;

pub trait Tick {
    fn tick(&mut self, _time: u128) -> bool {
        false
    }
}

pub trait Draw<D: GameDisplay> {
    fn draw(&self, display: &mut D);
}

pub trait Drawable {
    fn tile_id(&self) -> usize;
}

pub trait ZLevel {
    fn z_level(&self) -> usize;
}

pub trait Coord<const MAX_X: usize, const MAX_Y: usize> {
    fn coords(&self) -> Point;

    fn move_up(&mut self) {
        self.move_if_in_bound(self.coords().x as usize, self.coords().y as usize + 1);
    }

    fn move_down(&mut self) {
        self.move_if_in_bound(self.coords().x as usize, self.coords().y as usize - 1);
    }

    fn move_right(&mut self) {
        self.move_if_in_bound(self.coords().x as usize + 1, self.coords().y as usize);
    }

    fn move_left(&mut self) {
        self.move_if_in_bound(self.coords().x as usize - 1, self.coords().y as usize);
    }

    fn move_if_in_bound(&mut self, x: usize, y: usize) {
        if self.in_bound(x, y) {
            self.set_x(x);
            self.set_y(y);
        }
    }

    fn set_x(&mut self, x: usize) {
        self.coords().x = x as i32;
    }

    fn set_y(&mut self, y: usize) {
        self.coords().y = y as i32
    }

    fn in_bound(&self, x: usize, y: usize) -> bool {
        x < MAX_X && y < MAX_Y
    }
}
