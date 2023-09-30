use crate::ili9431::Command;
use embassy_rp::dma::{AnyChannel, Channel};
use embassy_rp::gpio::Level;
use embassy_rp::pio::{
    Common, Config, Direction, FifoJoin, Instance, PioPin, ShiftConfig, ShiftDirection,
    StateMachine,
};
use embassy_rp::{into_ref, Peripheral, PeripheralRef};
use fixed::types::U24F8;
use heapless::Vec;

pub trait PioParallel<DataFormat> {
    async fn write_command(&mut self, command: Command, words: &[DataFormat]);
}

pub struct PioParallel16<'a, P: Instance, const N: usize> {
    dma: PeripheralRef<'a, AnyChannel>,
    sm: StateMachine<'a, P, N>,
}

impl<'a, P: Instance, const N: usize> PioParallel16<'a, P, N> {
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
    ) -> PioParallel16<'a, P, N> {
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
        cfg.clock_divider = U24F8::from_num(2.0);
        sm.set_config(&cfg);
        sm.set_enable(true);

        PioParallel16 {
            dma: dma.map_into(),
            sm,
        }
    }
}

impl<'a, P: Instance, const N: usize> PioParallel<u16> for PioParallel16<'a, P, N> {
    async fn write_command(&mut self, command: Command, words: &[u16]) {
        let mut data: Vec<u16, { 32 * 32 + 1 }> = Vec::new();
        data.extend_from_slice(&[command as u16]).unwrap();
        data.extend_from_slice(words).unwrap();

        self.sm
            .tx()
            .dma_push(self.dma.reborrow(), data.as_slice())
            .await;
    }
}
