use super::{Action, Actions, Coord, Item, ItemTrait, OnEvent, OnReaction, Target};
use crate::game::events::{Buttons, Event, States};
use crate::game::state_mashine::states::level::actions::MoveDestination;
use crate::game::tiles::Tile;
extern crate alloc;

pub struct Wizard;

impl ItemTrait for Item<Wizard> {}

impl OnEvent for Item<Wizard> {
    fn on_event(&mut self, event: &Event) -> Option<Action> {
        match event {
            Event::Tick(time) => {
                if time % 5 == 0 {
                    self.swith_state();
                    match self.state {
                        1 => self.img_id = Tile::wizard2_id(),
                        0 => self.img_id = Tile::wizard1_id(),
                        _ => {}
                    }
                    let target = Target::new(
                        self.coords().x as usize,
                        self.coords().y as usize,
                        self.z_order,
                    );

                    Some(Action::new(target, Actions::Redraw))
                } else {
                    None
                }
            }
            Event::Button(Buttons::Up(States::Pressed)) => Some(Action::new(
                self.target(),
                Actions::Move {
                    dest: MoveDestination::Up,
                },
            )),
            Event::Button(Buttons::Down(States::Pressed)) => Some(Action::new(
                self.target(),
                Actions::Move {
                    dest: MoveDestination::Down,
                },
            )),
            Event::Button(Buttons::Left(States::Pressed)) => Some(Action::new(
                self.target(),
                Actions::Move {
                    dest: MoveDestination::Left,
                },
            )),
            Event::Button(Buttons::Right(States::Pressed)) => Some(Action::new(
                self.target(),
                Actions::Move {
                    dest: MoveDestination::Right,
                },
            )),
            _ => None,
        }
    }
}

impl OnReaction for Item<Wizard> {
    fn on_reaction(&mut self, action: &Action) {
        match action.action {
            Actions::Move {
                dest: MoveDestination::Up,
            } => self.move_up(),
            Actions::Move {
                dest: MoveDestination::Down,
            } => self.move_down(),
            Actions::Move {
                dest: MoveDestination::Left,
            } => self.move_left(),
            Actions::Move {
                dest: MoveDestination::Right,
            } => self.move_right(),
            _ => (),
        }
    }
}

impl Item<Wizard> {
    fn swith_state(&mut self) {
        self.state = if self.state == 0 { 1 } else { 0 }
    }
}
