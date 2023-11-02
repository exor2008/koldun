use super::actions::{Action, Actions, Target, Who};
use super::Event;
use crate::game::{MAX_X, MAX_Y};
use crate::h_vec;
use crate::ili9486::GameDisplay;
use core::marker::PhantomData;
use embedded_graphics::prelude::Point;
use heapless::Vec;
extern crate alloc;

pub mod exit;
pub mod sprite;
pub mod wizard;

pub const MAX_ACTIONS_PER_EVENT: usize = 3;

pub trait OnEvent {
    fn on_event(&mut self, _event: &Event) -> Vec<Action, MAX_ACTIONS_PER_EVENT> {
        h_vec!(MAX_ACTIONS_PER_EVENT;)
    }
}

pub trait OnReaction {
    fn on_reaction(&mut self, _action: &Action);
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
    fn coords_mut(&mut self) -> &mut Point;

    fn target(&self) -> Target;

    fn move_up(&mut self) {
        self._move(self.coords().x as usize, self.coords().y as usize - 1);
    }

    fn move_down(&mut self) {
        self._move(self.coords().x as usize, self.coords().y as usize + 1);
    }

    fn move_right(&mut self) {
        self._move(self.coords().x as usize + 1, self.coords().y as usize);
    }

    fn move_left(&mut self) {
        self._move(self.coords().x as usize - 1, self.coords().y as usize);
    }

    fn _move(&mut self, x: usize, y: usize) {
        self.set_x(x);
        self.set_y(y);
    }

    fn set_x(&mut self, x: usize) {
        self.coords_mut().x = x as i32;
    }

    fn set_y(&mut self, y: usize) {
        self.coords_mut().y = y as i32
    }
}

pub trait ItemTrait: OnEvent + OnReaction + Coord<MAX_X, MAX_Y> + Drawable + ZLevel + Send {}

pub struct Item<I> {
    z_order: usize,
    coords: Point,
    img_id: usize,
    state: u8,
    start_animation: u128,
    time: u128,
    is_win: bool,
    kind: PhantomData<I>,
}

impl<I> Coord<MAX_X, MAX_Y> for Item<I> {
    fn coords(&self) -> Point {
        self.coords
    }

    fn coords_mut(&mut self) -> &mut Point {
        &mut self.coords
    }

    fn target(&self) -> Target {
        Target::new(self.coords.x as usize, self.coords.y as usize, self.z_order)
    }
}

impl<I> ZLevel for Item<I> {
    fn z_level(&self) -> usize {
        self.z_order
    }
}

impl<I> Drawable for Item<I> {
    fn tile_id(&self) -> usize {
        self.img_id
    }
}

impl<I> Item<I> {
    pub fn new(coords: Point, z_order: usize, img_id: usize) -> Self {
        Item {
            z_order,
            coords,
            img_id,
            state: Default::default(),
            start_animation: Default::default(),
            time: Default::default(),
            is_win: Default::default(),
            kind: Default::default(),
        }
    }
}
