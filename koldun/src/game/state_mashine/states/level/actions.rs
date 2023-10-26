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
    Move { dest: MoveDestination },
    Redraw,
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
