use super::{Action, Item, ItemTrait, OnEvent, OnReaction};
use crate::game::events::Event;
extern crate alloc;

pub struct StaticSprite;

impl OnEvent for Item<StaticSprite> {
    fn on_event(&mut self, _event: &Event) -> Option<Action> {
        None
    }
}

impl OnReaction for Item<StaticSprite> {
    fn on_reaction(&mut self, _action: &Action) {
        unimplemented!("Static sprites shouldn't receive reactions")
    }
}

impl ItemTrait for Item<StaticSprite> {}
