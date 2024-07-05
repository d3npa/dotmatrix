use crate::graphics::Graphic;
use embassy_rp::gpio::{AnyPin, Level, Output};

pub enum Line<'a> {
    Anode(Output<'a, AnyPin>),
    Cathode(Output<'a, AnyPin>),
}

impl<'a> Line<'a> {
    pub fn new_anode(pin: AnyPin) -> Self {
        Self::Anode(Output::new(pin, Level::Low))
    }

    pub fn new_cathode(pin: AnyPin) -> Self {
        Self::Cathode(Output::new(pin, Level::High))
    }

    pub fn enable(&mut self) {
        match self {
            Line::Anode(out) => out.set_high(),
            Line::Cathode(out) => out.set_low(),
        }
    }

    pub fn disable(&mut self) {
        match self {
            Line::Anode(out) => out.set_low(),
            Line::Cathode(out) => out.set_high(),
        }
    }
}

// TODO: rewrite. this should be a generic ShiftRegister
pub struct ShiftRegister<'a> {
    pub ser: Line<'a>,
    // pub oe: Line<'a>,
    pub rclk: Line<'a>,
    pub srclk: Line<'a>,
    pub srclr: Line<'a>,
}

impl<'a> ShiftRegister<'a> {
    // 1s represent disabled columns (cathodes)
    // TODO: this shouldn't be in the generic driver...
    // pub const EMPTY_SIGNAL: u16 = 0b1101011000101100;

    pub fn tick(&mut self) {
        self.srclk.enable();
        self.srclk.disable();
    }

    pub fn latch(&mut self) {
        self.rclk.enable();
        self.rclk.disable();
    }

    pub fn clear(&mut self) {
        self.srclr.enable();
        self.srclr.disable();
        // self.write_short(Self::EMPTY_SIGNAL);
    }

    #[allow(clippy::bool_comparison)]
    pub fn write_short(&mut self, data: u16) {
        for bit in (0..16).map(|i| data & (1 << i) != 0) {
            if bit == true {
                self.ser.enable();
            } else {
                self.ser.disable();
            }
            self.tick();
        }
        self.latch();
    }
}

pub struct DotMatrixLed<'a> {
    pub sr: ShiftRegister<'a>,
    pub graphic: Graphic,
    pub overridden: bool,
}

impl<'a> DotMatrixLed<'a> {
    pub const EMPTY_SIGNAL: u16 = 0b1101011000101100;

    pub fn clear(&mut self) {
        self.sr.write_short(Self::EMPTY_SIGNAL);
    }

    pub fn render(&mut self) {
        let row_map = [9, 14, 8, 12, 1, 7, 2, 5];
        let col_map = [13, 3, 4, 10, 6, 11, 15, 16];
        for (row_index, row) in self.graphic.iter().enumerate() {
            let mut signal = Self::EMPTY_SIGNAL;
            for (col_index, &pixel) in row.iter().enumerate() {
                if pixel == 1 {
                    signal |= 1 << (row_map[row_index] - 1);
                    signal &= !(1u16 << (col_map[col_index] - 1));
                }
            }
            self.sr.write_short(signal);
        }
        self.clear();
    }
}
