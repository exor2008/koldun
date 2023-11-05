use defmt::Format;

#[derive(Format)]
pub enum Event {
    Button(Buttons),
    Tick(u128),
}

#[derive(Format)]
pub enum Buttons {
    Up(States),
    Down(States),
    Left(States),
    Right(States),
    Reset(States),
}

#[derive(Format)]
pub enum States {
    Pressed,
    Released,
}
