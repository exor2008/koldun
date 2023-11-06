use heapless::Vec;

use super::{Action, Item, ItemTrait, Kind, Kinds, OnEvent, OnReaction, MAX_ACTIONS_PER_EVENT};
use crate::{game::events::Event, h_vec};
extern crate alloc;

pub struct StaticSprite;

impl OnEvent for Item<StaticSprite> {
    fn on_event(&mut self, _event: &Event) -> Vec<Action, MAX_ACTIONS_PER_EVENT> {
        h_vec!(MAX_ACTIONS_PER_EVENT;)
    }
}

impl OnReaction for Item<StaticSprite> {
    fn on_reaction(&mut self, _action: &Action) {
        // unimplemented!("Static sprites shouldn't receive reactions")
    }
}

impl Kind for Item<StaticSprite> {
    fn kind(&self) -> Kinds {
        Kinds::Sprite
    }
}

impl ItemTrait for Item<StaticSprite> {}
