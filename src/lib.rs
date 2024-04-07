#![no_std]

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

/// represents a single 8x8 dot-matrix led display
pub struct DotMatrixDisplay<'a> {
    pub rows: [Line<'a>; 8],
    pub cols: [Line<'a>; 8],
    pub graphic: Graphic,
    pub overridden: bool,
}

pub type Display<'a> = DotMatrixDisplay<'a>;

impl<'a> DotMatrixDisplay<'a> {
    pub async fn render(&mut self) {
        for r in 0..self.rows.len() {
            for c in 0..self.cols.len() {
                if self.graphic[r][c] == 1 {
                    self.rows[r].enable();
                    self.cols[c].enable();
                }
            }
            self.clear();
        }
    }

    pub fn clear(&mut self) {
        for r in &mut self.rows {
            r.disable();
        }
        for c in &mut self.cols {
            c.disable();
        }
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
pub struct DotMatrixDisplayMutex<'a>(
    pub Mutex<CriticalSectionRawMutex, Option<DotMatrixDisplay<'a>>>,
);

impl<'a> DotMatrixDisplayMutex<'a> {
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
