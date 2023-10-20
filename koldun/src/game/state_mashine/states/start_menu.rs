use crate::events::{Buttons, Event, States};
use crate::game::colors;
use crate::game::flash::Flash;
use crate::game::state_mashine::states::level::{Level, Level1};
use crate::game::state_mashine::states::State;
use crate::ili9486::Display;
use crate::ili9486::GameDisplay;
use alloc::boxed::Box;
use async_trait::async_trait;
use core::marker::Send;
use defmt::info;
use embedded_graphics::pixelcolor::Rgb565;
use embedded_graphics::prelude::{Point, Size};
use embedded_graphics::primitives::Rectangle;
extern crate alloc;

const MAX_COMMANDS: u16 = 3;

pub enum StartMenuCommands {
    NewGame,
    Options,
}

pub struct StartMenu {
    command: u16,
}

impl StartMenu {
    pub fn new() -> Self {
        StartMenu { command: 0 }
    }

    async fn on_up<D, F>(&mut self, display: &mut D) -> Option<Box<dyn State<D, F>>>
    where
        D: GameDisplay + Send + Display<u8, Color = Rgb565>,
        F: Flash + Send + Sync,
    {
        self.command = match self.command < MAX_COMMANDS - 1 {
            true => self.command + 1,
            false => 0,
        };

        self.redraw(display).await;
        None
    }

    async fn on_down<D, F>(&mut self, display: &mut D) -> Option<Box<dyn State<D, F>>>
    where
        D: GameDisplay + Send + Display<u8, Color = Rgb565>,
        F: Flash + Send + Sync,
    {
        self.command = match self.command > 0 {
            true => self.command - 1,
            false => MAX_COMMANDS - 1,
        };

        self.redraw(display).await;
        None
    }

    async fn on_select<D, F>(&mut self, _display: &mut D) -> Option<Box<dyn State<D, F>>>
    where
        D: GameDisplay + Display<u8, Color = Rgb565> + Send,
        F: Flash + Send + Sync,
    {
        match self.command {
            0 => Some(Box::new(Level::<Level1>::new())),
            _ => None,
        }
    }

    async fn redraw<D>(&mut self, display: &mut D)
    where
        D: GameDisplay + Display<u8, Color = Rgb565>,
    {
        display
            .draw_solid_area(
                Rectangle::new(Point::new(45, 55), Size::new(70, 40)),
                colors::START_MENU_BG,
            )
            .await;

        display.draw_text(
            "KOLDUN the Game",
            Point::new(50, 50),
            colors::START_MENU_TILE,
            None,
        );

        display.draw_text(
            "New game",
            Point::new(50, 70),
            colors::START_MENU_TEXT,
            match self.command {
                0 => Some(colors::START_MENU_TEXT_BG),
                _ => None,
            },
        );

        display.draw_text(
            "Continue",
            Point::new(50, 85),
            colors::START_MENU_TEXT,
            match self.command {
                1 => Some(colors::START_MENU_TEXT_BG),
                _ => None,
            },
        );

        display.draw_text(
            "Options",
            Point::new(50, 100),
            colors::START_MENU_TEXT,
            match self.command {
                2 => Some(colors::START_MENU_TEXT_BG),
                _ => None,
            },
        );
    }
}

#[async_trait]
impl<D, F> State<D, F> for StartMenu
where
    D: GameDisplay + Send + Display<u8, Color = Rgb565>,
    F: Flash + Send + Sync,
{
    async fn on_event(&mut self, event: Event, display: &mut D) -> Option<Box<dyn State<D, F>>> {
        match event {
            Event::Button(Buttons::Up(States::Pressed)) => self.on_up::<D, F>(display).await,
            Event::Button(Buttons::Down(States::Pressed)) => self.on_down::<D, F>(display).await,
            Event::Button(Buttons::Right(States::Pressed)) => self.on_select::<D, F>(display).await,
            _ => None,
        }
    }

    async fn on_init(&mut self, display: &mut D, _flash: &mut F) {
        info!("StartMenu Init");
        display.clear(colors::START_MENU_BG).unwrap();

        self.redraw(display).await;
    }
}
