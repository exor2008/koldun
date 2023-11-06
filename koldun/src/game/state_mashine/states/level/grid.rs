use super::actions::{Action, Actions, MoveDestination, Pos, RedrawRequest, Target};
use super::items::Kinds;
use super::items::{sprite::StaticSprite, Drawable, Item, ItemTrait, MAX_ACTIONS_PER_EVENT};
use crate::game::events::Event;
use crate::game::{MAX_X, MAX_Y};
use crate::{add_to_redraw, h_vec};
use alloc::boxed::Box;
use core::error::Error;
use core::fmt;
use core::ops::{Index, IndexMut};
use embedded_graphics::prelude::Point;
use heapless::Vec;

extern crate alloc;

const LAYERS: usize = 2;
pub const MAX_EVENTS: usize = 128;

pub struct Cell {
    // coords: Point,
    items: Vec<Option<Box<dyn ItemTrait>>, LAYERS>,
}

impl Cell {
    fn new() -> Self {
        let mut items: Vec<Option<Box<dyn ItemTrait>>, LAYERS> = Vec::new();
        for _ in 0..LAYERS {
            unsafe {
                items.push_unchecked(None);
            }
        }
        Cell { items }
    }

    fn new_static_sprite(coords: Point, img_id: usize, z_order: usize) -> Self {
        let mut items: Vec<Option<Box<dyn ItemTrait>>, LAYERS> = Vec::new();
        let sprite: Item<StaticSprite> = Item::new(coords, z_order, img_id);

        if z_order == 0 {
            unsafe {
                items.push_unchecked(Some(Box::new(sprite)));
                items.push_unchecked(None);
            }
        } else {
            unsafe {
                items.push_unchecked(None);
                items.push_unchecked(Some(Box::new(sprite)));
            }
        }
        for _ in 0..LAYERS - 2 {
            unsafe {
                items.push_unchecked(None);
            }
        }
        Cell { items }
    }

    pub fn set_item(&mut self, item: Box<dyn ItemTrait>) {
        let z_level = item.z_level();
        self.items[z_level] = Some(item);
    }

    pub fn take_item(&mut self, z_level: usize) -> Option<Box<dyn ItemTrait>> {
        self.items[z_level].take()
    }

    pub fn has_item(&mut self, z_level: usize) -> bool {
        self.items[z_level].is_some()
    }

    fn get_items_len(&self) -> usize {
        let items: Vec<&Option<Box<dyn ItemTrait>>, LAYERS> =
            self.items.iter().filter(|item| item.is_some()).collect();
        items.len()
    }

    fn find_kind(&self, kind: Kinds) -> Option<Target> {
        for item in self.items.iter() {
            if let Some(item) = item {
                if item.kind() == kind {
                    return Some(item.target());
                }
            }
        }
        None
    }

    fn on_event(&mut self, event: &Event) -> Vec<Action, MAX_EVENTS> {
        self.items
            .iter_mut()
            .flat_map(|item| {
                if let Some(item) = item {
                    item.on_event(event)
                } else {
                    h_vec![MAX_ACTIONS_PER_EVENT;]
                }
            })
            .collect()
    }

    pub fn on_reactions(&mut self, reaction: Action) {
        if let Some(item) = self.items[reaction.target.z].as_deref_mut() {
            item.on_reaction(&reaction)
        }
    }
}

impl Default for Cell {
    fn default() -> Self {
        Cell::new()
    }
}

impl Drawable for Cell {
    fn tile_id(&self) -> usize {
        if let Some(item) = self.items.iter().rev().find_map(|item| item.as_deref()) {
            return item.tile_id();
        }
        unreachable!();
    }
}

pub struct Grid([[Cell; MAX_X]; MAX_Y]);

impl Grid {
    pub fn new() -> Self {
        let grid: [[Cell; MAX_X]; MAX_Y] = Default::default();
        Self(grid)
    }

    pub fn on_event(&mut self, event: &Event) -> Vec<Action, MAX_EVENTS> {
        let mut actions: Vec<Action, MAX_EVENTS> = Vec::new();

        for x in 0..MAX_X {
            for y in 0..MAX_Y {
                let cell = self.get_cell_mut(x, y).unwrap();
                let cell_actions = cell.on_event(event);
                actions.extend(cell_actions);
            }
        }
        actions
    }

    pub fn on_actions(
        &mut self,
        actions: Vec<Action, MAX_EVENTS>,
    ) -> (
        Vec<Action, MAX_EVENTS>,
        Vec<RedrawRequest, 32>,
        Option<bool>,
        bool,
    ) {
        let mut reactions: Vec<Action, MAX_EVENTS> = Vec::new();
        let mut to_redraw: Vec<RedrawRequest, 32> = Vec::new();
        let mut block: Option<bool> = None;
        let mut is_win: bool = false;

        for action in actions {
            match action {
                // Move
                Action {
                    target,
                    action: Actions::Move { dest, who },
                } => {
                    if let Ok(mut new_target) = self.move_item(target, dest) {
                        // Successfull move, add reaction
                        for z in 0..LAYERS {
                            new_target.z = z;
                            reactions
                                .push(Action::new(new_target, Actions::Move { dest, who }))
                                .unwrap();
                        }

                        // Block controlls
                        block = Some(true);
                    }
                }

                // Redraw
                Action {
                    target,
                    action: Actions::Redraw,
                } => {
                    let request = RedrawRequest::new(target);
                    add_to_redraw!(to_redraw, request);
                }

                // Redraw moving animated object
                Action {
                    target,
                    action: Actions::RedrawAnim(shift_x, shift_y, old_target),
                } => {
                    let request = RedrawRequest::new(old_target);
                    add_to_redraw!(to_redraw, request);

                    let request =
                        RedrawRequest::new_anim(target, Pos::new(shift_x.into(), shift_y.into()));
                    add_to_redraw!(to_redraw, request);
                }

                // Initialize spell
                Action {
                    target: _,
                    action: Actions::InitSpell(dest),
                } => {
                    if let Some(mut target) = self.find_kind(Kinds::Wizard) {
                        // info!("wizard found in {}", target);
                        match dest {
                            MoveDestination::Up => target.y -= 1,
                            MoveDestination::Down => target.y += 1,
                            MoveDestination::Left => target.x -= 1,
                            MoveDestination::Right => target.x += 1,
                        }

                        let init_cell = self.get_cell_mut(0, 0).unwrap();
                        // info!("items in 0 0 : {}", init_cell.get_items_len());
                        let mut spell = init_cell.take_item(0).unwrap();

                        if let Some(cell) = self.get_cell_mut(target.x, target.y) {
                            if !cell.has_item(target.z) {
                                spell.set_x(target.x);
                                spell.set_y(target.y);
                                spell.set_z(target.z);
                                cell.set_item(spell);
                                let request = RedrawRequest::new(target);
                                add_to_redraw!(to_redraw, request);
                            }
                        }
                    }
                }

                // Block control events
                Action {
                    target: _,
                    action: Actions::Block(block_),
                } => block = Some(block_),

                // Win!
                Action {
                    target: _,
                    action: Actions::Win,
                } => is_win = true,
            }
        }
        (reactions, to_redraw, block, is_win)
    }

    pub fn on_reactions(&mut self, reactions: Vec<Action, MAX_EVENTS>) {
        reactions.into_iter().for_each(|Action { target, action }| {
            self.0[target.y][target.x].on_reactions(Action::new(target, action))
        });
    }

    pub fn tile_id(&self, x: usize, y: usize) -> usize {
        self.0[y][x].tile_id()
    }

    pub fn set_item(&mut self, x: usize, y: usize, item: Box<dyn ItemTrait>) {
        if let Some(cell) = self.get_cell_mut(x, y) {
            cell.set_item(item);
        }
    }

    fn move_item(&mut self, src: Target, dest: MoveDestination) -> Result<Target, CellError> {
        if let Some(cell) = self.get_cell_mut(src.x, src.y) {
            let item = cell.take_item(src.z);
            if let Some(item) = item {
                let mut new_target = Target::new(0, 0, 0);

                let dest_cell = match dest {
                    MoveDestination::Up => {
                        new_target.x = src.x;
                        new_target.y = src.y - 1;
                        self.get_cell_mut(new_target.x, new_target.y)
                    }
                    MoveDestination::Down => {
                        new_target.x = src.x;
                        new_target.y = src.y + 1;
                        self.get_cell_mut(new_target.x, new_target.y)
                    }
                    MoveDestination::Left => {
                        new_target.x = src.x - 1;
                        new_target.y = src.y;
                        self.get_cell_mut(new_target.x, new_target.y)
                    }
                    MoveDestination::Right => {
                        new_target.x = src.x + 1;
                        new_target.y = src.y;
                        self.get_cell_mut(new_target.x, new_target.y)
                    }
                };

                if let Some(cell) = dest_cell {
                    if !cell.has_item(item.z_level()) {
                        cell.set_item(item);
                        new_target.z = src.z;
                        return Ok(new_target);
                    }
                }

                // Move unseccessful, rollback
                self.get_cell_mut(src.x, src.y).unwrap().set_item(item);
            }
        }

        Err(CellError::MoveError)
    }

    fn in_bound(&self, x: usize, y: usize) -> bool {
        x < MAX_X && y < MAX_Y
    }

    fn get_cell_mut(&mut self, x: usize, y: usize) -> Option<&mut Cell> {
        match self.in_bound(x, y) {
            true => Some(&mut self.0[y][x]),
            false => None,
        }
    }

    fn get_cell_ref(&self, x: usize, y: usize) -> Option<&Cell> {
        match self.in_bound(x, y) {
            true => Some(&self.0[y][x]),
            false => None,
        }
    }

    fn get_cell_items_len(&mut self, x: usize, y: usize) -> usize {
        if let Some(cell) = self.get_cell_mut(x, y) {
            let items: Vec<&Option<Box<dyn ItemTrait>>, LAYERS> =
                cell.items.iter().filter(|item| item.is_some()).collect();
            items.len()
        } else {
            0
        }
    }

    fn find_kind(&self, kind: Kinds) -> Option<Target> {
        for x in 0..MAX_X {
            for y in 0..MAX_Y {
                let src_cell = self.get_cell_ref(x, y).unwrap();
                if let Some(target) = src_cell.find_kind(kind) {
                    return Some(target);
                }
            }
        }
        None
    }

    pub fn new_from(other: &mut Grid) -> Self {
        let mut grid = Grid::new();
        for x in 0..MAX_X {
            for y in 0..MAX_Y {
                let src_cell = other.get_cell_mut(x, y).unwrap();
                let dst_cell = grid.get_cell_mut(x, y).unwrap();

                for z_level in 0..LAYERS {
                    if let Some(item) = src_cell.take_item(z_level) {
                        dst_cell.set_item(item);
                    }
                }
            }
        }
        grid
    }
}

impl From<[[usize; MAX_X]; MAX_Y]> for Grid {
    fn from(array: [[usize; MAX_X]; MAX_Y]) -> Self {
        let mut grid: [[Cell; MAX_X]; MAX_Y] = Default::default();
        for x in 0..MAX_X {
            for y in 0..MAX_Y {
                let img_id = array[y][x];
                let z_order = if img_id <= 32 { 0 } else { 1 };
                grid[y][x] =
                    Cell::new_static_sprite(Point::new(x as i32, y as i32), img_id, z_order);
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

#[derive(Debug)]
enum CellError {
    MoveError,
}

impl Error for CellError {}

impl fmt::Display for CellError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CellError::MoveError => write!(f, "Validation failed"),
        }
    }
}
