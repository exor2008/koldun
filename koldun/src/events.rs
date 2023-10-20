pub enum Event {
    Button(Buttons),
    Tick(u128),
}

pub enum Buttons {
    Up(States),
    Down(States),
    Left(States),
    Right(States),
}

pub enum States {
    Pressed,
    Released,
}
