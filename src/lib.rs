#![no_std]

use core::ops::Deref;

use embassy_sync::{
    blocking_mutex::raw::CriticalSectionRawMutex, mutex::Mutex,
};
use embassy_time::{Duration, Ticker};

pub mod graphics;
pub mod hal;

use graphics::Graphic;

use hal::DotMatrixLed;

pub static DISPLAYS: Displays<'static> = Displays::new();

pub struct DotMatrixLedMutex<'a>(
    pub Mutex<CriticalSectionRawMutex, Option<DotMatrixLed<'a>>>,
);

impl<'a> DotMatrixLedMutex<'a> {
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
            d.render();
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
            if iters == 1 {
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

pub struct Displays<'a>([DotMatrixLedMutex<'a>; 4]);

impl<'a> Deref for Displays<'a> {
    type Target = [DotMatrixLedMutex<'a>; 4];
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a> Displays<'a> {
    pub const fn new() -> Self {
        Self([
            DotMatrixLedMutex::new(),
            DotMatrixLedMutex::new(),
            DotMatrixLedMutex::new(),
            DotMatrixLedMutex::new(),
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
