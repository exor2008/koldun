use crate::ili9486::Command;
use alloc::boxed::Box;
use async_trait::async_trait;
use embassy_rp::dma::{AnyChannel, Channel};
use embassy_rp::gpio::Level;
use embassy_rp::pio::{
    Common, Config, Direction, FifoJoin, Instance, PioPin, ShiftConfig, ShiftDirection,
    StateMachine,
};
use embassy_rp::{into_ref, Peripheral, PeripheralRef};
use fixed::types::U24F8;
use heapless::Vec;
extern crate alloc;

#[async_trait]
pub trait PioParallel<DataFormat> {
    async fn write_command(&mut self, command: Command, words: &[DataFormat]);
}

pub struct PioParallel8<'a, P: Instance, const N: usize> {
    dma: PeripheralRef<'a, AnyChannel>,
    sm: StateMachine<'a, P, N>,
}

impl<'a, P: Instance, const N: usize> PioParallel8<'a, P, N> {
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
        cs: impl PioPin,
        dc: impl PioPin,
        rd: impl PioPin,
        wr: impl PioPin,
    ) -> PioParallel8<'a, P, N> {
        into_ref!(dma);

        let db0 = pio.make_pio_pin(db0);
        let db1 = pio.make_pio_pin(db1);
        let db2 = pio.make_pio_pin(db2);
        let db3 = pio.make_pio_pin(db3);
        let db4 = pio.make_pio_pin(db4);
        let db5 = pio.make_pio_pin(db5);
        let db6 = pio.make_pio_pin(db6);
        let db7 = pio.make_pio_pin(db7);

        let cs = pio.make_pio_pin(cs);
        let dc = pio.make_pio_pin(dc);
        let rd = pio.make_pio_pin(rd);
        let wr = pio.make_pio_pin(wr);

        let prg_command = pio_proc::pio_asm!(
            r#"
            .side_set 4 opt
            .wrap_target
                nop                 side 0b0100 ; wr - ON   rd - OFF dc - COMMAND   cs - ON
                out pins    8 
                jmp !osre   data    side 0b1100 ; wr - OFF  rd - OFF dc - COMMAND   cs - ON
                jmp end
            data:
                nop                 side 0b0110 ; wr - ON   rd - OFF dc - DATA      cs - ON
                out pins    8
                jmp !osre   data    side 0b1110 ; wr - OFF  rd - OFF dc - DATA      cs - ON
            end:
                nop                 side 0b1111 ; wr - OFF  rd - OFF dc - DATA      cs - OFF
            .wrap
            "#,
        );

        sm.set_pin_dirs(
            Direction::Out,
            &[
                &db0, &db1, &db2, &db3, &db4, &db5, &db6, &db7, &cs, &dc, &rd, &wr,
            ],
        );
        sm.set_pins(Level::High, &[&cs, &rd, &wr]);
        sm.set_pins(Level::Low, &[&dc]);

        let mut cfg = Config::default();
        cfg.use_program(
            &pio.load_program(&prg_command.program),
            &[&cs, &dc, &rd, &wr],
        );
        cfg.set_out_pins(&[&db0, &db1, &db2, &db3, &db4, &db5, &db6, &db7]);

        cfg.shift_out = ShiftConfig {
            auto_fill: true,
            direction: ShiftDirection::Left,
            threshold: 8,
        };

        cfg.fifo_join = FifoJoin::TxOnly;
        cfg.clock_divider = U24F8::from_num(3.0);
        sm.set_config(&cfg);
        sm.set_enable(true);

        PioParallel8 {
            dma: dma.map_into(),
            sm,
        }
    }
}

#[async_trait]
impl<'a, P: Instance + Send, const N: usize> PioParallel<u8> for PioParallel8<'a, P, N> {
    async fn write_command(&mut self, command: Command, words: &[u8]) {
        let mut data: Vec<u8, { 32 * 32 * 2 + 1 }> = Vec::new();
        data.extend_from_slice(&[command as u8]).unwrap();
        data.extend_from_slice(words).unwrap();

        self.sm
            .tx()
            .dma_push(self.dma.reborrow(), data.as_slice())
            .await;
    }
}
