pub mod pio_parallel;
use crate::ili9486::pio_parallel::PioParallel;
use alloc::boxed::Box;
use async_trait::async_trait;
use core::convert::Infallible;
use core::slice::SlicePattern;
use embassy_futures::block_on;
use embedded_graphics::draw_target::DrawTarget;
use embedded_graphics::mono_font::ascii::FONT_9X15_BOLD;
use embedded_graphics::mono_font::MonoTextStyle;
use embedded_graphics::pixelcolor::Rgb565;
use embedded_graphics::prelude::*;
use embedded_graphics::prelude::{Dimensions, Point, Size};
use embedded_graphics::primitives::Rectangle;
use embedded_graphics::text::Text;
use embedded_graphics::Pixel;
use heapless::Vec;
use tinytga::Tga;
extern crate alloc;

pub enum PixelFormat {
    Bit16 = 0b0101_0101,
    Bit18 = 0b0110_0110,
}

pub enum Order {
    Forward,
    Reverse,
}

impl Default for Order {
    fn default() -> Order {
        Order::Forward
    }
}

#[async_trait]
pub trait DrawTargetText: DrawTarget {
    fn draw_text(
        &mut self,
        text: &str,
        position: Point,
        color: Self::Color,
        bg: Option<Self::Color>,
    );
}

#[async_trait]
pub trait GameDisplay: Display<u8> + DrawTargetText<Color = Rgb565, Error = Infallible> {}

#[async_trait]
pub trait Display<DataFormat> {
    type Color: PixelColor;

    async fn set_active_area(&mut self, area: Rectangle);
    async fn set_pixel_format(&mut self, pixel: PixelFormat);
    async fn sleep_out(&mut self);
    async fn inversion_off(&mut self);
    async fn memory_access_control(
        &mut self,
        row_order: Order,
        column_order: Order,
        rc_exchange: Order,
        vert_refresh: Order,
        hor_refresh: Order,
        color: Order,
    );
    async fn norma_display_mode(&mut self);
    async fn display_on(&mut self);
    async fn idle_mode_off(&mut self);
    async fn draw_data(&mut self, area: Rectangle, data: &[DataFormat]);
    async fn draw_solid(&mut self, origin: Point, color: Self::Color);
    async fn draw_solid_area(&mut self, area: Rectangle, color: Self::Color);
    async fn draw_tile(&mut self, origin: Point, data: &[DataFormat]);
    async fn tearing_effect_line_on(&mut self);
    async fn column_address_set(&mut self, start: u16, end: u16);
    async fn page_address_set(&mut self, start: u16, end: u16);
    fn tga_to_data(data: &[u8]) -> Vec<DataFormat, { 32 * 32 * 2 }>;
}

pub struct Ili9486<C: PioParallel<u8>>
where
    C: Send,
{
    pio_interface: C,
}

impl<C: PioParallel<u8>> Ili9486<C>
where
    C: Send,
{
    pub fn new(pio_interface: C) -> Ili9486<C> {
        Ili9486 { pio_interface }
    }

    fn color_to_data(color: Rgb565) -> [u8; 2] {
        let b = color.to_ne_bytes();
        [b[1], b[0]]
    }
}

impl<C: PioParallel<u8>> Dimensions for Ili9486<C>
where
    C: Send,
{
    fn bounding_box(&self) -> Rectangle {
        Rectangle::new(Point::new(0, 0), Size::new(480, 320))
    }
}

impl<C: PioParallel<u8>> DrawTarget for Ili9486<C>
where
    C: Send,
{
    type Color = Rgb565;
    type Error = Infallible;

    fn clear(&mut self, color: Self::Color) -> Result<(), Self::Error> {
        let area = self.bounding_box();
        self.fill_solid(&area, color).unwrap();
        Ok(())
    }

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<Self::Color>>,
    {
        for Pixel(coord, color) in pixels.into_iter() {
            if let Ok((_x @ 0..=479, _y @ 0..=319)) = coord.try_into() {
                let data = Self::color_to_data(color);
                let area = Rectangle::new(Point::new(coord.x, coord.y), Size::new(1, 1));
                block_on(self.draw_data(area, &data));
            }
        }
        Ok(())
    }

    fn fill_contiguous<I>(&mut self, area: &Rectangle, colors: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Self::Color>,
    {
        let area = area.intersection(&self.bounding_box());

        let data: Vec<_, { 32 * 32 }> = colors
            .into_iter()
            .map(|p| Self::color_to_data(p))
            .flatten()
            .collect();
        block_on(self.draw_data(area, data.as_slice()));
        Ok(())
    }

    fn fill_solid(&mut self, area: &Rectangle, color: Self::Color) -> Result<(), Self::Error> {
        let color = Self::color_to_data(color);
        let data = &[color; 32 * 32];
        let mut v: Vec<[u8; 2], { 32 * 32 }> = Vec::new();
        v.extend_from_slice(data).unwrap();
        let v = v.flatten();

        for x in (0..480).step_by(32) {
            for y in (0..320).step_by(32) {
                let square = Rectangle::new(Point::new(x, y), Size::new(32, 32));
                let area = area.intersection(&square);

                block_on(self.draw_data(area, v.as_slice()));
            }
        }
        Ok(())
    }
}

impl<C: PioParallel<u8>> DrawTargetText for Ili9486<C>
where
    C: Send,
{
    fn draw_text(
        &mut self,
        text: &str,
        position: Point,
        color: Self::Color,
        bg: Option<Self::Color>,
    ) {
        let mut style = MonoTextStyle::new(&FONT_9X15_BOLD, color);
        style.background_color = bg;
        Text::new(text, position, style).draw(self).unwrap();
    }
}

#[async_trait]
impl<C: PioParallel<u8> + Send> Display<u8> for Ili9486<C> {
    type Color = Rgb565;

    async fn set_active_area(&mut self, area: Rectangle) {
        let start = area.top_left;
        if let Some(end) = area.bottom_right() {
            self.column_address_set(start.x as u16, end.x as u16).await;
            self.page_address_set(start.y as u16, end.y as u16).await;
        }
    }

    async fn set_pixel_format(&mut self, pixel: PixelFormat) {
        self.pio_interface
            .write_command(Command::InterfacePixelFormat, &[pixel as u8])
            .await;
    }

    async fn sleep_out(&mut self) {
        self.pio_interface
            .write_command(Command::SleepOut, &[])
            .await;
    }

    async fn display_on(&mut self) {
        self.pio_interface
            .write_command(Command::DisplayOn, &[])
            .await;
    }

    async fn idle_mode_off(&mut self) {
        self.pio_interface
            .write_command(Command::IdleModeOff, &[])
            .await;
    }

    async fn inversion_off(&mut self) {
        self.pio_interface
            .write_command(Command::DisplayInversionOff, &[])
            .await;
    }

    async fn norma_display_mode(&mut self) {
        self.pio_interface
            .write_command(Command::NormalDisplayMode, &[])
            .await;
    }

    async fn memory_access_control(
        &mut self,
        row_order: Order,
        column_order: Order,
        rc_exchange: Order,
        vert_refresh: Order,
        hor_refresh: Order,
        color: Order,
    ) {
        let mut data = 0u8;

        data |= match hor_refresh {
            Order::Forward => 0,
            Order::Reverse => 1 << 2,
        };

        data |= match color {
            Order::Forward => 0,
            Order::Reverse => 1 << 3,
        };

        data |= match vert_refresh {
            Order::Forward => 0,
            Order::Reverse => 1 << 4,
        };

        data |= match rc_exchange {
            Order::Forward => 0,
            Order::Reverse => 1 << 5,
        };

        data |= match column_order {
            Order::Forward => 0,
            Order::Reverse => 1 << 6,
        };

        data |= match row_order {
            Order::Forward => 0,
            Order::Reverse => 1 << 7,
        };

        self.pio_interface
            .write_command(Command::MemoryAccessControl, &[data])
            .await;
    }

    async fn draw_data(&mut self, area: Rectangle, data: &[u8]) {
        self.set_active_area(area).await;
        self.pio_interface
            .write_command(Command::MemoryWrite, data)
            .await;
    }

    async fn draw_solid(&mut self, origin: Point, color: Self::Color) {
        let color = Self::color_to_data(color);
        let data = &[color; 32 * 32];
        let mut v: Vec<[u8; 2], { 32 * 32 }> = Vec::new();
        v.extend_from_slice(data).unwrap();

        self.draw_tile(origin, v.flatten()).await;
    }

    async fn draw_solid_area(&mut self, area: Rectangle, color: Self::Color) {
        let area = self.bounding_box().intersection(&area);
        if let Some(bottom_right) = area.bottom_right() {
            for x in (area.top_left.x..bottom_right.x).step_by(32) {
                for y in (area.top_left.y..bottom_right.y).step_by(32) {
                    self.draw_solid(Point::new(x, y), color).await;
                }
            }
        }
    }

    async fn draw_tile(&mut self, origin: Point, data: &[u8]) {
        let area = Rectangle::new(origin, Size::new(32, 32));
        self.set_active_area(area).await;
        self.pio_interface
            .write_command(Command::MemoryWrite, data)
            .await;
    }

    async fn tearing_effect_line_on(&mut self) {
        self.pio_interface
            .write_command(Command::TearingEffectLineOn, &[0b1])
            .await;
    }

    async fn column_address_set(&mut self, start: u16, end: u16) {
        let data = [
            (start >> 8) as u8,
            (start & 0xff) as u8,
            (end >> 8) as u8,
            (end & 0xff) as u8,
        ];
        self.pio_interface
            .write_command(Command::ColumnAddressSet, &data)
            .await;
    }

    async fn page_address_set(&mut self, start: u16, end: u16) {
        let data = [
            (start >> 8) as u8,
            (start & 0xff) as u8,
            (end >> 8) as u8,
            (end & 0xff) as u8,
        ];
        self.pio_interface
            .write_command(Command::PageAddressSet, &data)
            .await;
    }

    fn tga_to_data(data: &[u8]) -> Vec<u8, { 32 * 32 * 2 }> {
        let tga: Tga<Self::Color> = Tga::from_slice(data).unwrap();
        let pixels: Vec<_, { 32 * 32 * 2 }> = tga
            .pixels()
            .map(|p| Self::color_to_data(p.1))
            .flatten()
            .collect();

        pixels
    }
}

#[async_trait]
impl<C: PioParallel<u8> + Send> GameDisplay for Ili9486<C> {}

#[derive(Clone, Copy, Debug)]
pub enum Command {
    Nop = 0x00,
    SoftReset = 0x01,
    ReadDisplayId = 0x04,
    ReadErrors = 0x05,
    ReadDisplayStatus = 0x09,
    ReadDisplayPowerMode = 0x0a,
    ReadDisplayMADCTL = 0x0b,
    ReadDisplayPixelFormat = 0x0c,
    ReadDisplayImageMode = 0x0d,
    ReadDisplaySignalMode = 0x0e,
    ReadDisplaySelfDiagResult = 0x0f,
    SleepIn = 0x10,
    SleepOut = 0x11,
    PartialModeOn = 0x12,
    NormalDisplayMode = 0x13,
    DisplayInversionOff = 0x20,
    DisplayInversionOn = 0x21,
    DisplayOff = 0x28,
    DisplayOn = 0x29,
    ColumnAddressSet = 0x2a,
    PageAddressSet = 0x2b,
    MemoryWrite = 0x2c,
    MemoryRead = 0x2e,
    PartialArea = 0x30,
    VerticalScrollingDefinition = 0x33,
    TearingEffectLineOff = 0x34,
    TearingEffectLineOn = 0x35,
    MemoryAccessControl = 0x36,
    VerticalScrollingStartAddress = 0x37,
    IdleModeOff = 0x38,
    IdleModeOn = 0x39,
    InterfacePixelFormat = 0x3a,
    MemoryWriteContinue = 0x3c,
    MemoryReadContinue = 0x3e,
    WriteTearScanLine = 0x44,
    ReadTearScanLine = 0x45,
    WriteDisplayBrightnessValue = 0x51,
    ReadDisplayBrigthnessValue = 0x52,
    WriteCTRLDisplayValue = 0x53,
    ReadCTRLDisplayValue = 0x54,
    WriteCABrigthnessControl = 0x55,
    ReadCABrigthnessControl = 0x56,
    WriteCABCMinBrigthness = 0x5e,
    ReadCABCMinBrigthness = 0x5f,
    ReadFirstChecksum = 0xaa,
    ReadContinueChecksum = 0xab,
    ReadID1 = 0xda,
    ReadID2 = 0xdb,
    ReadID3 = 0xdc,
    InterfaceModeControl = 0xb0,
    FrameRateControlNormal = 0xb1,
    FrameRateControlIdle = 0xb2,
    FrameRateControlPartial = 0xb3,
    DisplayInversionControl = 0xb4,
    BlankingPorchControl = 0xb5,
    DisplayFunctionControl = 0xb6,
    EntryModeSet = 0xb7,
    PowerControl1 = 0xc0,
    PowerControl2 = 0xc1,
    PowerControl3 = 0xc2,
    PowerControl4 = 0xc3,
    PowerControl5 = 0xc4,
    VCOMControl = 0xc5,
    CABCControl9 = 0xc6,
    CABCControl1 = 0xc8,
    CABCControl2 = 0xc9,
    CABCControl3 = 0xca,
    CABCControl4 = 0xcb,
    CABCControl5 = 0xcc,
    CABCControl6 = 0xcd,
    CABCControl7 = 0xce,
    CABCControl8 = 0xcf,
    NVMemoryWrite = 0xd0,
    NVMemoryProtectionKey = 0xd1,
    NVMemoryStatusRead = 0xd2,
    ReadID4 = 0xd3,
    PGAMCTRL = 0xe0,
    NGAMCTRL = 0xe1,
    DigitalGammaControl1 = 0xe2,
    DigitalGammaControl2 = 0xe3,
    SPIReadCommandSetting = 0xfb,
}
