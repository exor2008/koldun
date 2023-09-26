use embassy_rp::dma::{AnyChannel, Channel};
use embassy_rp::gpio::Level;
use embassy_rp::pio::Common;
use embassy_rp::pio::{
    Config, Direction, FifoJoin, Instance, PioPin, ShiftConfig, ShiftDirection, StateMachine,
};

use embassy_rp::{into_ref, Peripheral, PeripheralRef};
use fixed::types::U24F8;
use heapless::Vec;

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

pub struct ILI9431<'a, P: Instance, const N: usize> {
    dma: PeripheralRef<'a, AnyChannel>,
    sm: StateMachine<'a, P, N>,
}

impl<'a, P: Instance, const N: usize> ILI9431<'a, P, N> {
    pub fn new(
        pio: &mut Common<'a, P>,
        mut sm: StateMachine<'a, P, N>,
        dma: impl Peripheral<P = impl Channel> + 'a,
        db0: impl PioPin,
        db1: impl PioPin,
        db2: impl PioPin,
        db3: impl PioPin,
        db4: impl PioPin,
        db5: impl PioPin,
        db6: impl PioPin,
        db7: impl PioPin,
        db8: impl PioPin,
        db9: impl PioPin,
        db10: impl PioPin,
        db11: impl PioPin,
        db12: impl PioPin,
        db13: impl PioPin,
        db14: impl PioPin,
        db15: impl PioPin,
        cs: impl PioPin,
        dc: impl PioPin,
        rd: impl PioPin,
        wr: impl PioPin,
    ) -> ILI9431<'a, P, N> {
        into_ref!(dma);

        let db0 = pio.make_pio_pin(db0);
        let db1 = pio.make_pio_pin(db1);
        let db2 = pio.make_pio_pin(db2);
        let db3 = pio.make_pio_pin(db3);
        let db4 = pio.make_pio_pin(db4);
        let db5 = pio.make_pio_pin(db5);
        let db6 = pio.make_pio_pin(db6);
        let db7 = pio.make_pio_pin(db7);
        let db8 = pio.make_pio_pin(db8);
        let db9 = pio.make_pio_pin(db9);
        let db10 = pio.make_pio_pin(db10);
        let db11 = pio.make_pio_pin(db11);
        let db12 = pio.make_pio_pin(db12);
        let db13 = pio.make_pio_pin(db13);
        let db14 = pio.make_pio_pin(db14);
        let db15 = pio.make_pio_pin(db15);
        let cs = pio.make_pio_pin(cs);
        let dc = pio.make_pio_pin(dc);
        let rd = pio.make_pio_pin(rd);
        let wr = pio.make_pio_pin(wr);

        let prg_command = pio_proc::pio_asm!(
            r#"
            .side_set 4 opt
            .wrap_target
                nop                 side 0b0100 ; wr - ON   rd - OFF dc - COMMAND   cs - ON
                out pins    16 
                jmp !osre   data    side 0b1100 ; wr - OFF  rd - OFF dc - COMMAND   cs - ON
                jmp end
            data:
                nop                 side 0b0110 ; wr - ON   rd - OFF dc - DATA      cs - ON
                out pins    16 
                jmp !osre   data    side 0b1110 ; wr - OFF  rd - OFF dc - DATA      cs - ON
            end:
                nop                 side 0b1111 ; wr - OFF  rd - OFF dc - DATA      cs - OFF
            .wrap
            "#,
        );

        sm.set_pin_dirs(
            Direction::Out,
            &[
                &db0, &db1, &db2, &db3, &db4, &db5, &db6, &db7, &db8, &db9, &db10, &db11, &db12,
                &db13, &db14, &db15, &dc, &rd, &wr,
            ],
        );
        sm.set_pins(Level::High, &[&cs, &rd, &wr]);
        sm.set_pins(Level::Low, &[&dc]);

        let mut cfg = Config::default();
        cfg.use_program(
            &pio.load_program(&prg_command.program),
            &[&cs, &dc, &rd, &wr],
        );
        cfg.set_out_pins(&[
            &db0, &db1, &db2, &db3, &db4, &db5, &db6, &db7, &db8, &db9, &db10, &db11, &db12, &db13,
            &db14, &db15,
        ]);

        cfg.shift_out = ShiftConfig {
            auto_fill: true,
            direction: ShiftDirection::Left,
            threshold: 16,
        };

        cfg.fifo_join = FifoJoin::TxOnly;
        cfg.clock_divider = U24F8::from_num(1.5);
        sm.set_config(&cfg);
        sm.set_enable(true);

        ILI9431 {
            dma: dma.map_into(),
            sm,
        }
    }

    pub async fn write_command(&mut self, command: Command, words: &[u16]) {
        let mut a: Vec<u16, { 32 * 32 + 1 }> = Vec::new();
        a.extend_from_slice(&[command as u16]).unwrap();
        a.extend_from_slice(words).unwrap();

        self.sm
            .tx()
            .dma_push(self.dma.reborrow(), a.as_slice())
            .await;
    }
}
