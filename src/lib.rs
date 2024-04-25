#![no_std]

use core::ops::Deref;

use embassy_rp::gpio::{AnyPin, Level, Output};
use embassy_sync::{
    blocking_mutex::raw::CriticalSectionRawMutex, mutex::Mutex,
};
use embassy_time::{Duration, Ticker};

pub mod graphics;
use graphics::Graphic;

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

pub trait DotMatrixOutput {
    fn output(&mut self, g: Graphic);
    fn clear(&mut self);
}

pub struct GpioOutput<'a> {
    pub rows: [Line<'a>; 8],
    pub cols: [Line<'a>; 8],
}

impl<'a> DotMatrixOutput for GpioOutput<'a> {
    fn output(&mut self, g: Graphic) {
        for r in 0..self.rows.len() {
            for c in 0..self.cols.len() {
                if g[r][c] == 1 {
                    self.rows[r].enable();
                    self.cols[c].enable();
                }
            }
            self.clear();
        }
    }

    fn clear(&mut self) {
        for r in &mut self.rows {
            r.disable();
        }
        for c in &mut self.cols {
            c.disable();
        }
    }
}

pub struct ShiftRegisterOutput<'a> {
    pub ser: Line<'a>,
    pub oe: Line<'a>,
    pub rclk: Line<'a>,
    pub srclk: Line<'a>,
    pub srclr: Line<'a>,
}

impl<'a> ShiftRegisterOutput<'a> {
    // 1s represent disabled columns (cathodes)
    const EMPTY_SIGNAL: u16 = 0b1101011000101100;
    fn tick(&mut self) {
        self.srclk.enable();
        self.srclk.disable();
    }

    fn latch(&mut self) {
        self.rclk.enable();
        self.rclk.disable();
    }

    fn clear(&mut self) {
        self.write_short(Self::EMPTY_SIGNAL);
    }

    fn write_short(&mut self, data: u16) {
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

impl<'a> DotMatrixOutput for ShiftRegisterOutput<'a> {
    fn output(&mut self, g: Graphic) {
        let row_map = [9, 14, 8, 12, 1, 7, 2, 5];
        let col_map = [13, 3, 4, 10, 6, 11, 15, 16];
        for r in 0..g.len() {
            let mut signal = Self::EMPTY_SIGNAL;
            for c in 0..g[r].len() {
                if g[r][c] == 1 {
                    signal |= 1 << (row_map[r] - 1);
                    signal &= !(1u16 << (col_map[c] - 1));
                }
            }
            self.write_short(signal);
        }
        self.clear();
    }

    fn clear(&mut self) {
        ShiftRegisterOutput::clear(self);
    }
}

/// represents a single 8x8 dot-matrix led display
pub struct DotMatrixDisplay<O>
where
    O: DotMatrixOutput,
{
    pub output_driver: O,
    pub graphic: Graphic,
    pub overridden: bool,
}

// pub type Display<'a> = DotMatrixDisplay<'a>;
pub type Display<O> = DotMatrixDisplay<O>;

// impl<'a> DotMatrixDisplay<'a> {
impl<O> DotMatrixDisplay<O>
where
    O: DotMatrixOutput,
{
    pub async fn render(&mut self) {
        self.output_driver.output(self.graphic);
    }

    pub fn clear(&mut self) {
        self.output_driver.clear();
    }

    /// TODO: deprecate. setting graphic directly is fine.
    pub fn draw(&mut self, graphic: Graphic) {
        self.graphic = graphic;
    }

    /*
     * 　flash, panoramaなどのアニメーションここで書きたかったけど、
     * メインプログラムでDotMatrixDisplayのアクセスをMutexで管理していて、
     * flash().awaitを呼び出す前にそのMutexをlock()しているから、
     * 別タスクで実行しているrender()はlock()取れなくなって、
     * 一切表示されなくなってしまう。
     *
     * 　だから一旦は、アニメーションをMutexラッパーで実装することにした。
     *
     * 　確か、std/tokioだとawaitの間ロックを解除する種類のMutexがあった気が
     * するけど、ここで似たようなものはあるかどうか。いずれ調べるべきでしょう。
     *
     */
    // pub async fn flash()
    // pub async fn panorama()
    // pub async fn pulse()
}

/// TODO: better research types of mutexes
/// TODO: must there really be a single static Display, needing Option?
pub struct DotMatrixDisplayMutex<O: DotMatrixOutput>(
    pub Mutex<CriticalSectionRawMutex, Option<DotMatrixDisplay<O>>>,
);

impl<O> DotMatrixDisplayMutex<O>
where
    O: DotMatrixOutput,
{
    const FLASH_DURATION: Duration = Duration::from_millis(100);
    const OVERRIDE_CHECK_INTERVAL: Duration = Duration::from_micros(100);

    pub const fn new() -> Self {
        Self(Mutex::new(None))
    }

    pub async fn overridden(&self) -> bool {
        match self.0.lock().await.as_ref() {
            Some(d) => d.overridden,
            None => false,
        }
    }

    pub async fn set_override(&self, state: bool) {
        let mut ticker = Ticker::every(Self::OVERRIDE_CHECK_INTERVAL);
        while self.overridden().await && state {
            ticker.next().await;
        }
        if let Some(d) = self.0.lock().await.as_mut() {
            d.overridden = state;
        }
    }

    /// TODO: deprecate. the term "lock/ed" is confusing in a Mutex context
    pub async fn locked(&self) -> bool {
        self.overridden().await
    }

    /// TODO: deprecate
    pub async fn lock(&self) {
        self.set_override(true).await;
    }

    /// TODO: deprecate
    pub async fn unlock(&self) {
        self.set_override(false).await;
    }

    pub async fn render(&self) {
        if let Some(d) = self.0.lock().await.as_mut() {
            d.render().await;
        }
    }

    pub async fn draw(&self, g: Graphic) {
        if let Some(d) = self.0.lock().await.as_mut() {
            d.graphic = g;
        }
    }

    pub async fn flash(
        &self,
        panorama: graphics::Panorama,
        ignore_lock: bool,
    ) {
        let mut ticker = Ticker::every(Self::FLASH_DURATION);

        for i in 0..panorama.len {
            while (!ignore_lock) && self.overridden().await {
                ticker.next().await;
            }

            let graphic = panorama.graphics[i];
            self.draw(*graphic).await;
            ticker.next().await;
        }
    }

    pub async fn pulse(&self) {
        let mut ticker = Ticker::every(Duration::from_millis(20));
        let mut counter: usize = 0;
        let mut iters = 0;

        loop {
            while self.locked().await {
                ticker.next().await;
            }
            // moving diagonal stripe
            let mut canvas = graphics::EMPTY;
            for r in 0..canvas.len() {
                let n = counter - r;
                if n < canvas[r].len() {
                    canvas[r][n] = 1;
                }
            }

            counter += 1;
            counter %= 16;
            self.draw(canvas).await;
            ticker.next().await;
            if counter == 0 {
                iters += 1;
            }
            if iters == 2 {
                break;
            }
        }
    }

    pub async fn panorama2(&self, message: &str, prio: bool) {
        // let panorama_cols = message.len() * 8;
        let mut ticker = Ticker::every(Duration::from_millis(40));

        for pair in message.chars().zip(message.chars().skip(1)) {
            let mut cursor = 0;
            let (a, b) = {
                let (a, b) = pair;
                (graphics::from_char(a), graphics::from_char(b))
            };

            // 8, bc i only care about the first char and pieces of the 2nd
            while cursor < 8 {
                while (!prio) && self.overridden().await {
                    ticker.next().await;
                }

                let mut canvas = graphics::EMPTY;
                for r in 0..canvas.len() {
                    for canvas_c in 0..canvas[r].len() {
                        let panorama_c = canvas_c + cursor;
                        let frame_c = panorama_c % 8;
                        let frame = {
                            // a or b?
                            if panorama_c < 8 {
                                a
                            } else {
                                b
                            }
                        };
                        canvas[r][canvas_c] = frame[r][frame_c];
                    }
                }

                cursor += 1;
                self.draw(canvas).await;
                ticker.next().await;
            }
        }
    }
}

pub struct Displays<O: DotMatrixOutput>([DotMatrixDisplayMutex<O>; 4]);

impl<O: DotMatrixOutput> Deref for Displays<O> {
    type Target = [DotMatrixDisplayMutex<O>; 4];
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<O: DotMatrixOutput> Displays<O> {
    pub const fn new() -> Self {
        Self([
            DotMatrixDisplayMutex::new(),
            DotMatrixDisplayMutex::new(),
            DotMatrixDisplayMutex::new(),
            DotMatrixDisplayMutex::new(),
        ])
    }

    pub async fn panorama(&self, message: &str, prio: bool) {
        let d0 = async {
            self[0].panorama2(&message, prio).await;
        };

        let d1 = async {
            self[1].panorama2(&message[1..], prio).await;
        };

        let d2 = async {
            self[2].panorama2(&message[2..], prio).await;
        };

        let d3 = async {
            self[3].panorama2(&message[3..], prio).await;
        };

        embassy_futures::join::join4(d0, d1, d2, d3).await;
    }
}
