use defmt::Format;

#[derive(Debug, Format)]
pub struct Action {
    pub target: Target,
    pub action: Actions,
}

impl Action {
    pub fn new(target: Target, action: Actions) -> Self {
        Action { target, action }
    }
}

#[derive(Debug, Format)]
pub enum Actions {
    Move { dest: MoveDestination, who: Who },
    RedrawAnim(i8, i8, Target),
    Redraw,
    Block(bool),
    InitSpell(MoveDestination),
    Win,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, Format)]
pub struct Target {
    pub x: usize,
    pub y: usize,
    pub z: usize,
}

impl Target {
    pub fn new(x: usize, y: usize, z: usize) -> Self {
        Target { x, y, z }
    }
}

#[derive(Debug, Clone, Copy, Format)]
pub enum MoveDestination {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, Format)]
pub enum Who {
    Wizard,
}

#[derive(Eq, Hash, PartialEq, Debug)]
pub struct RedrawRequest {
    pub target: Target,
    // pub old_target: Option<Target>,
    pub shift: Pos,
}

impl RedrawRequest {
    pub fn new(target: Target) -> Self {
        RedrawRequest {
            target,
            // old_target: None,
            shift: Pos::default(),
        }
    }
    pub fn new_anim(target: Target, shift: Pos) -> Self {
        RedrawRequest {
            target,
            // old_target: Some(old_target),
            shift,
        }
    }
}

#[derive(Eq, Hash, PartialEq, Debug)]
pub struct Pos {
    pub x: isize,
    pub y: isize,
}

impl Pos {
    pub fn new(x: isize, y: isize) -> Self {
        Pos { x, y }
    }
}

impl Default for Pos {
    fn default() -> Self {
        Pos {
            x: Default::default(),
            y: Default::default(),
        }
    }
}
