use crate::control::Controls;
use crate::game::flash::Flash;
use crate::game::state_mashine::states::start_menu::StartMenu;
use crate::game::state_mashine::State;
use crate::ili9486::Display;
use crate::ili9486::GameDisplay;
use alloc::boxed::Box;
use async_trait::async_trait;
use defmt::info;
use embedded_graphics::pixelcolor::Rgb565;
extern crate alloc;

pub struct Initial {}

impl Initial {
    pub fn new() -> Self {
        Initial {}
    }
}

#[async_trait]
impl<D, F> State<D, F> for Initial
where
    D: GameDisplay + Send + Display<u8, Color = Rgb565>,
    F: Flash + Send + Sync,
{
    async fn on_control(
        &mut self,
        _event: Controls,
        _display: &mut D,
    ) -> Option<Box<dyn State<D, F>>> {
        info!("Init State");
        Some(Box::new(StartMenu::new()))
    }

    async fn on_init(&mut self, _display: &mut D, _flash: &mut F) {}
}
