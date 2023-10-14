pub enum Controls {
    BUTTON(Buttons),
}

pub enum Buttons {
    UP(States),
    DOWN(States),
    LEFT(States),
    RIGHT(States),
}

pub enum States {
    PRESSED,
    RELEASED,
}
