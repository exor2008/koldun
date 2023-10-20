use super::ItemTrait;
use crate::game::items::Item;
use crate::game::tiles::Tile;
use crate::game::Tick;
// use embedded_graphics::prelude::Point;

pub struct Wizard;

impl ItemTrait for Item<Wizard> {}

impl Tick for Item<Wizard> {
    fn tick(&mut self, time: u128) -> bool {
        if time % 5 == 0 {
            self.swith_state();
            match self.state {
                1 => self.img_id = Tile::wizard2_id(),
                0 => self.img_id = Tile::wizard1_id(),
                _ => {}
            }
            return true;
        }
        false
    }
}

impl Item<Wizard> {
    fn swith_state(&mut self) {
        self.state = if self.state == 0 { 1 } else { 0 }
    }
}
