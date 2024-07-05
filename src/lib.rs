#![no_std]

use core::cmp::min;
use core::default::Default;
use core::ops::Deref;
use core::str;

use embassy_sync::{
    blocking_mutex::raw::CriticalSectionRawMutex, mutex::Mutex,
};
use embassy_time::{Duration, Ticker};

pub mod graphics;
pub mod hal;

// pub mod serialdrv;

use graphics::Graphic;

use hal::DotMatrixLed;

pub static DISPLAYS: Displays<'static> = Displays::new();
pub static DATA: Mutex<CriticalSectionRawMutex, Data> =
    Mutex::new(Data::new());

pub struct DotMatrixLedMutex<'a>(
    pub Mutex<CriticalSectionRawMutex, Option<DotMatrixLed<'a>>>,
);

impl<'a> Default for DotMatrixLedMutex<'a> {
    fn default() -> Self {
        Self::new()
    }
}

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

    pub async fn draw(&self, g: &Graphic) {
        if let Some(d) = self.0.lock().await.as_mut() {
            d.graphic = *g;
        }
    }

    pub async fn flash(
        &self,
        panorama: &graphics::Panorama,
        ignore_lock: bool,
    ) {
        let mut ticker = Ticker::every(Self::FLASH_DURATION);

        for i in 0..panorama.len {
            while (!ignore_lock) && self.overridden().await {
                ticker.next().await;
            }

            let graphic = panorama.graphics[i];
            self.draw(graphic).await;
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
            for (row_index, row) in canvas.iter_mut().enumerate() {
                let n = counter - row_index;
                if n < row.len() {
                    row[n] = 1;
                }
            }

            counter += 1;
            counter %= 16;
            self.draw(&canvas).await;
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
        let mut ticker = Ticker::every(Duration::from_millis(30));

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
                self.draw(&canvas).await;
                ticker.next().await;
            }
        }
    }
}

#[derive(Default)]
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
        let p = pad(message.as_bytes());
        let message = null_term_string(&p).unwrap_or("");
        embassy_futures::join::join4(
            self[0].panorama2(message, prio),
            self[1].panorama2(&message[1..], prio),
            self[2].panorama2(&message[2..], prio),
            self[3].panorama2(&message[3..], prio),
        )
        .await;
    }

    pub async fn set_override(&self, v: bool) {
        for d in &self.0 {
            d.set_override(v).await;
        }
    }

    pub async fn alert(&self) {
        let alert = graphics::Panorama {
            graphics: {
                let mut g = [&graphics::EMPTY; 8];
                g[1] = &graphics::FULL;
                g[3] = &graphics::FULL;
                g[5] = &graphics::FULL;
                g[7] = &graphics::FULL;
                g
            },
            len: 8,
        };
        embassy_futures::join::join4(
            self[0].flash(&alert, true),
            self[1].flash(&alert, true),
            self[2].flash(&alert, true),
            self[3].flash(&alert, true),
        )
        .await;
    }
}

#[derive(Debug)]
pub enum Error {
    Utf8,
    // DataLength,
}

type ClockString = [u8; 16];
type WeatherString = [u8; 16];

pub struct Data {
    pub clock: Option<ClockString>,
    pub weather: Option<WeatherString>,
}

impl Data {
    #[allow(unused)]
    const fn new() -> Self {
        Self {
            clock: None,
            weather: None,
        }
    }
}

pub fn copy_str_bytes<const LEN: usize>(
    buf: &[u8],
) -> Result<[u8; LEN], Error> {
    if str::from_utf8(buf).is_err() {
        return Err(Error::Utf8);
    }

    let mut out = [0u8; LEN];
    let copy_len = min(buf.len(), out.len());
    out[..copy_len].copy_from_slice(&buf[..copy_len]);

    Ok(out)
}

/// pads string with spaces, so it can be scrolled on displays
/// four spaces are prepended, and one space is appended
/// the string will be truncated if it is more than 59 (64 - 5) chars
pub fn pad(string: &[u8]) -> [u8; 64] {
    let mut out = [0x0; 64];
    let mut cursor = 0;

    for _i in 0..4 {
        out[cursor] = 0x20; // スペース文字
        cursor += 1;
    }

    use core::cmp::min;
    let msg_part_len = min(string.len(), out.len() - 5);

    for &character in string.iter().take(msg_part_len) {
        if character == 0 {
            break; // doing this bc calling from clock() and weather(), string contains nulls and if i don't break here then the final 0x20 under will be pushed back and eventually cut by null_term_string. these funcs need to be merged somehow...
        }
        out[cursor] = character;
        cursor += 1;
    }

    // after last char from string
    out[cursor] = 0x20;
    // cursor += 1;

    out
}

pub fn null_term_string(data: &[u8]) -> Result<&str, Error> {
    let mut null_index = 0;

    for (index, &byte) in data.iter().enumerate() {
        if byte == 0 {
            null_index = index;
            break;
        }
    }

    str::from_utf8(&data[..null_index]).map_err(|_| Error::Utf8)
}
