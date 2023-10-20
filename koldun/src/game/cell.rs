use super::{Drawable, Tick, MAX_X, MAX_Y};
use crate::game::items::{sprite::StaticSprite, Item, ItemTrait};
use alloc::boxed::Box;
use core::ops::{Index, IndexMut};
use embedded_graphics::prelude::Point;
use heapless::Vec;
extern crate alloc;

const LAYERS: usize = 3;

// #[derive(Clone, Copy)]
pub struct Cell {
    coords: Point,
    items: Vec<Option<Box<dyn ItemTrait>>, LAYERS>,
}

impl Cell {
    fn new(coords: Point) -> Self {
        let mut items: Vec<Option<Box<dyn ItemTrait>>, LAYERS> = Vec::new();
        for _ in 0..LAYERS {
            unsafe {
                items.push_unchecked(None);
            }
        }
        Cell { coords, items }
    }

    fn new_static_sprite(coords: Point, img_id: usize) -> Self {
        let mut items: Vec<Option<Box<dyn ItemTrait>>, LAYERS> = Vec::new();
        let sprite: Item<StaticSprite> = Item::new(coords, 0, img_id);
        unsafe {
            items.push_unchecked(Some(Box::new(sprite)));
        }
        for _ in 0..LAYERS - 1 {
            unsafe {
                items.push_unchecked(None);
            }
        }
        Cell { coords, items }
    }

    pub fn set_item(&mut self, item: Box<dyn ItemTrait>) {
        let z_level = item.z_level();
        self.items[z_level] = Some(item);
    }
}

impl Tick for Cell {
    fn tick(&mut self, time: u128) -> bool {
        self.items
            .iter_mut()
            .map(|i| if let Some(i) = i { i.tick(time) } else { false })
            .any(|r| r)
    }
}

impl Default for Cell {
    fn default() -> Self {
        Cell::new(Point::default())
    }
}

impl Drawable for Cell {
    fn tile_id(&self) -> usize {
        for item in self.items.iter().rev() {
            if let Some(item) = item {
                return item.tile_id();
            }
        }
        unreachable!();
    }
}

pub struct Grid([[Cell; MAX_X]; MAX_Y]);

impl From<[[usize; MAX_X]; MAX_Y]> for Grid {
    fn from(array: [[usize; MAX_X]; MAX_Y]) -> Self {
        let mut grid: [[Cell; MAX_X]; MAX_Y] = Default::default();
        for x in 0..MAX_X {
            for y in 0..MAX_Y {
                let img_id = array[y][x];
                grid[y][x] = Cell::new_static_sprite(Point::new(x as i32, y as i32), img_id);
            }
        }
        Grid(grid)
    }
}

impl Index<usize> for Grid {
    type Output = [Cell; MAX_X];

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl IndexMut<usize> for Grid {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}
