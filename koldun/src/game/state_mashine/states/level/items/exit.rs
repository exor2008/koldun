use super::{
    Action, Actions, Coord, Item, ItemTrait, Kind, Kinds, OnEvent, OnReaction, Who,
    MAX_ACTIONS_PER_EVENT,
};
use crate::{game::events::Event, h_vec};
use heapless::Vec;

pub struct Exit;

impl ItemTrait for Item<Exit> {}

impl OnEvent for Item<Exit> {
    fn on_event(&mut self, _event: &Event) -> Vec<Action, MAX_ACTIONS_PER_EVENT> {
        match self.is_win {
            true => h_vec!(MAX_ACTIONS_PER_EVENT; Action::new(self.target(), Actions::Win)),
            false => h_vec!(MAX_ACTIONS_PER_EVENT;),
        }
    }
}

impl OnReaction for Item<Exit> {
    fn on_reaction(&mut self, action: &Action) {
        match action {
            Action {
                target: _,
                action:
                    Actions::Move {
                        dest: _,
                        who: Who::Wizard,
                    },
            } => self.is_win = true,
            _ => (),
        }
    }
}

impl Kind for Item<Exit> {
    fn kind(&self) -> Kinds {
        Kinds::Exit
    }
}
