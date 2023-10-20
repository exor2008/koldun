use super::ItemTrait;
use crate::game::items::Item;
use crate::game::Tick;

pub struct StaticSprite;

impl Tick for Item<StaticSprite> {
    fn tick(&mut self, _time: u128) -> bool {
        false
    }
}

impl ItemTrait for Item<StaticSprite> {}
