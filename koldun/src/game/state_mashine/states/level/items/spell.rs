use super::{
    Action, Actions, Coord, Drawable, Item, ItemTrait, Kind, Kinds, OnEvent, OnReaction, Who,
    ZLevel, MAX_ACTIONS_PER_EVENT,
};
use crate::{
    game::{
        events::Event,
        state_mashine::states::{
            level::actions::{MoveDestination, Target},
            spell::{SpellCommands, MAX_COMMANDS},
        },
        MAX_X, MAX_Y,
    },
    h_vec,
};
use embedded_graphics::prelude::Point;
use heapless::Vec;

const UNINITIALIZED: u8 = 0;

pub struct Spell {
    z_order: usize,
    coords: Point,
    img_id: usize,
    state: u8,
    start_animation: u128,
    time: u128,
    commands: Vec<SpellCommands, MAX_COMMANDS>,
}

impl Spell {}

impl Coord<MAX_X, MAX_Y> for Spell {
    fn coords(&self) -> Point {
        self.coords
    }

    fn coords_mut(&mut self) -> &mut Point {
        &mut self.coords
    }

    fn target(&self) -> Target {
        Target::new(self.coords.x as usize, self.coords.y as usize, self.z_order)
    }

    fn set_z(&mut self, z: usize) {
        self.z_order = z
    }
}

impl ZLevel for Spell {
    fn z_level(&self) -> usize {
        self.z_order
    }
}

impl Drawable for Spell {
    fn tile_id(&self) -> usize {
        self.img_id
    }
}

impl Kind for Spell {
    fn kind(&self) -> Kinds {
        Kinds::Spell
    }
}

impl Spell {
    pub fn new(
        coords: Point,
        z_order: usize,
        img_id: usize,
        commands: Vec<SpellCommands, MAX_COMMANDS>,
    ) -> Self {
        Spell {
            z_order,
            coords,
            img_id,
            state: UNINITIALIZED,
            start_animation: Default::default(),
            time: Default::default(),
            commands,
        }
    }
}

impl ItemTrait for Spell {}

impl OnEvent for Spell {
    fn on_event(&mut self, _event: &Event) -> Vec<Action, MAX_ACTIONS_PER_EVENT> {
        match self.state {
            UNINITIALIZED => {
                self.state = 1;
                let dir: Option<MoveDestination> = match self.commands[0] {
                    SpellCommands::Up => Some(MoveDestination::Up),
                    SpellCommands::Down => Some(MoveDestination::Down),
                    SpellCommands::Left => Some(MoveDestination::Left),
                    SpellCommands::Right => Some(MoveDestination::Right),
                    _ => None,
                };

                match dir {
                    Some(dir) => {
                        let action = Action::new(self.target(), Actions::InitSpell(dir));
                        h_vec!(MAX_ACTIONS_PER_EVENT; action)
                    }
                    None => h_vec!(MAX_ACTIONS_PER_EVENT;),
                }
            }
            _ => h_vec!(MAX_ACTIONS_PER_EVENT;),
        }
    }
}

impl OnReaction for Spell {
    fn on_reaction(&mut self, _action: &Action) {}
}
