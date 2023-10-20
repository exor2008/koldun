use super::{Coord, Drawable, Tick, ZLevel, MAX_X, MAX_Y};
use core::marker::PhantomData;
use embedded_graphics::prelude::Point;
extern crate alloc;

pub mod sprite;
pub mod wizard;

pub trait ItemTrait: Tick + Coord<MAX_X, MAX_Y> + Drawable + ZLevel + Send {}

pub struct Item<I> {
    z_order: usize,
    coords: Point,
    img_id: usize,
    state: u8,
    kind: PhantomData<I>,
}

impl<I> Coord<MAX_X, MAX_Y> for Item<I> {
    fn coords(&self) -> Point {
        self.coords
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
            kind: Default::default(),
        }
    }
}
