use super::{Action, Actions, Coord, Item, ItemTrait, OnEvent, OnReaction, MAX_ACTIONS_PER_EVENT};
use crate::game::events::{Buttons, Event, States};
use crate::game::state_mashine::states::level::actions::MoveDestination;
use crate::game::tiles::{Tile, TILE_SIZE_X, TILE_SIZE_Y};
use crate::h_vec;
use defmt::warn;
use heapless::Vec;
extern crate alloc;

const IDLE1: u8 = 0;
const IDLE2: u8 = 1;
const MOVE_UP1: u8 = 2;
const MOVE_UP2: u8 = 3;
const MOVE_DOWN1: u8 = 4;
const MOVE_DOWN2: u8 = 5;
const MOVE_LEFT1: u8 = 6;
const MOVE_LEFT2: u8 = 7;
const MOVE_RIGHT1: u8 = 8;
const MOVE_RIGHT2: u8 = 9;

const MOVE_ANIM_TIME: u128 = 10;

pub struct Wizard;

impl ItemTrait for Item<Wizard> {}

impl OnEvent for Item<Wizard> {
    fn on_event(&mut self, event: &Event) -> Vec<Action, MAX_ACTIONS_PER_EVENT> {
        match event {
            Event::Tick(time) => {
                self.time = *time;
                let deltat = time - self.start_animation;

                match self.state {
                    // Idle animation
                    IDLE1..=IDLE2 => {
                        if time % 5 == 0 {
                            self.swith_state(IDLE1, IDLE2);
                            match self.state {
                                IDLE1 => self.img_id = Tile::wizard_idle2_id(),
                                IDLE2 => self.img_id = Tile::wizard_idle1_id(),
                                _ => {}
                            }

                            h_vec!(
                                MAX_ACTIONS_PER_EVENT;
                                Action::new(self.target(), Actions::Redraw)
                            )
                        } else {
                            h_vec!(MAX_ACTIONS_PER_EVENT;)
                        }
                    }

                    // Move left animation
                    MOVE_LEFT1..=MOVE_LEFT2 => {
                        // Animation finished
                        if deltat > MOVE_ANIM_TIME {
                            self.state = IDLE1;

                            let mut old_target = self.target();
                            old_target.x += 1;

                            return h_vec!(MAX_ACTIONS_PER_EVENT;
                                Action::new(old_target, Actions::Redraw),
                                Action::new(self.target(), Actions::Redraw),
                                Action::new(old_target, Actions::Block(false)));
                        }

                        if time % 2 == 0 {
                            self.swith_state(MOVE_LEFT1, MOVE_LEFT2);
                            match self.state {
                                MOVE_LEFT2 => self.img_id = Tile::wizard_left1_id(),
                                MOVE_LEFT1 => self.img_id = Tile::wizard_left2_id(),
                                _ => {}
                            }
                            let shift_x = TILE_SIZE_X as f32
                                - TILE_SIZE_X as f32 * (deltat as f32 / MOVE_ANIM_TIME as f32);

                            let mut old_target = self.target();
                            old_target.x += 1;

                            h_vec!(MAX_ACTIONS_PER_EVENT; Action::new(
                                self.target(),
                                Actions::RedrawAnim(shift_x as i8, 0, old_target),
                            ))
                        } else {
                            h_vec!(MAX_ACTIONS_PER_EVENT; )
                        }
                    }

                    // Move right animation
                    MOVE_RIGHT1..=MOVE_RIGHT2 => {
                        // Animation finished
                        if deltat > MOVE_ANIM_TIME {
                            self.state = IDLE1;

                            let mut old_target = self.target();
                            old_target.x -= 1;

                            return h_vec!(MAX_ACTIONS_PER_EVENT;
                                Action::new(old_target, Actions::Redraw),
                                Action::new(self.target(), Actions::Redraw),
                                Action::new(old_target, Actions::Block(false)));
                        }

                        if time % 2 == 0 {
                            self.swith_state(MOVE_RIGHT1, MOVE_RIGHT2);
                            match self.state {
                                MOVE_RIGHT2 => self.img_id = Tile::wizard_right1_id(),
                                MOVE_RIGHT1 => self.img_id = Tile::wizard_right2_id(),
                                _ => {}
                            }
                            let shift_x = -(TILE_SIZE_X as f32)
                                + (TILE_SIZE_X as f32) * (deltat as f32 / MOVE_ANIM_TIME as f32);

                            let mut old_target = self.target();
                            old_target.x -= 1;

                            h_vec!(MAX_ACTIONS_PER_EVENT; Action::new(
                                self.target(),
                                Actions::RedrawAnim(shift_x as i8, 0, old_target),
                            ))
                        } else {
                            h_vec!(MAX_ACTIONS_PER_EVENT;)
                        }
                    }

                    // Move up animation
                    MOVE_UP1..=MOVE_UP2 => {
                        // Animation finished
                        if deltat > MOVE_ANIM_TIME {
                            self.state = IDLE1;

                            let mut old_target = self.target();
                            old_target.y += 1;

                            return h_vec!(MAX_ACTIONS_PER_EVENT;
                                Action::new(old_target, Actions::Redraw),
                                Action::new(self.target(), Actions::Redraw),
                                Action::new(old_target, Actions::Block(false)));
                        }

                        if time % 2 == 0 {
                            self.swith_state(MOVE_UP1, MOVE_UP2);
                            match self.state {
                                MOVE_UP2 => self.img_id = Tile::wizard_up1_id(),
                                MOVE_UP1 => self.img_id = Tile::wizard_up2_id(),
                                _ => {}
                            }
                            let shift_y = TILE_SIZE_Y as f32
                                - TILE_SIZE_Y as f32 * (deltat as f32 / MOVE_ANIM_TIME as f32);

                            let mut old_target = self.target();
                            old_target.y += 1;

                            h_vec!(MAX_ACTIONS_PER_EVENT; Action::new(
                                self.target(),
                                Actions::RedrawAnim(0, shift_y as i8, old_target),
                            ))
                        } else {
                            h_vec!(MAX_ACTIONS_PER_EVENT; )
                        }
                    }

                    // Move down animation
                    MOVE_DOWN1..=MOVE_DOWN2 => {
                        // Animation finished
                        if deltat > MOVE_ANIM_TIME {
                            self.state = IDLE1;

                            let mut old_target = self.target();
                            old_target.y -= 1;

                            return h_vec!(MAX_ACTIONS_PER_EVENT;
                                Action::new(old_target, Actions::Redraw),
                                Action::new(self.target(), Actions::Redraw),
                                Action::new(old_target, Actions::Block(false)));
                        }

                        if time % 2 == 0 {
                            self.swith_state(MOVE_DOWN1, MOVE_DOWN2);
                            match self.state {
                                MOVE_DOWN2 => self.img_id = Tile::wizard_down1_id(),
                                MOVE_DOWN1 => self.img_id = Tile::wizard_down2_id(),
                                _ => {}
                            }
                            let shift_y = -(TILE_SIZE_Y as f32)
                                + TILE_SIZE_Y as f32 * (deltat as f32 / MOVE_ANIM_TIME as f32);

                            let mut old_target = self.target();
                            old_target.y -= 1;

                            h_vec!(MAX_ACTIONS_PER_EVENT; Action::new(
                                self.target(),
                                Actions::RedrawAnim(0, shift_y as i8, old_target),
                            ))
                        } else {
                            h_vec!(MAX_ACTIONS_PER_EVENT; )
                        }
                    }

                    // Unknown state
                    _ => {
                        warn!("Unknown wizard's animation state");
                        h_vec!(MAX_ACTIONS_PER_EVENT; )
                    }
                }
            }

            Event::Button(Buttons::Up(States::Pressed)) => {
                h_vec!(MAX_ACTIONS_PER_EVENT; Action::new(
                    self.target(),
                    Actions::Move {
                        dest: MoveDestination::Up,
                    },
                ))
            }

            Event::Button(Buttons::Down(States::Pressed)) => {
                h_vec!(MAX_ACTIONS_PER_EVENT; Action::new(
                    self.target(),
                    Actions::Move {
                        dest: MoveDestination::Down,
                    },
                ))
            }

            Event::Button(Buttons::Left(States::Pressed)) => {
                h_vec!(MAX_ACTIONS_PER_EVENT; Action::new(
                    self.target(),
                    Actions::Move {
                        dest: MoveDestination::Left,
                    },
                ))
            }

            Event::Button(Buttons::Right(States::Pressed)) => {
                h_vec!(MAX_ACTIONS_PER_EVENT; Action::new(
                    self.target(),
                    Actions::Move {
                        dest: MoveDestination::Right,
                    },
                ))
            }

            _ => h_vec!(MAX_ACTIONS_PER_EVENT;),
        }
    }
}

impl OnReaction for Item<Wizard> {
    fn on_reaction(&mut self, action: &Action) {
        match action.action {
            Actions::Move {
                dest: MoveDestination::Up,
            } => {
                self.state = MOVE_UP1;
                self.start_animation = self.time;
                self.move_up();
            }
            Actions::Move {
                dest: MoveDestination::Down,
            } => {
                self.state = MOVE_DOWN1;
                self.start_animation = self.time;
                self.move_down();
            }
            Actions::Move {
                dest: MoveDestination::Left,
            } => {
                self.state = MOVE_LEFT1;
                self.start_animation = self.time;
                self.move_left();
            }
            Actions::Move {
                dest: MoveDestination::Right,
            } => {
                self.state = MOVE_RIGHT1;
                self.start_animation = self.time;
                self.move_right();
            }
            _ => (),
        }
    }
}

impl Item<Wizard> {
    fn swith_state(&mut self, state1: u8, state2: u8) {
        self.state = if self.state == state1 { state2 } else { state1 }
    }
}
