#![no_std]
#![no_main]

use core::panic::PanicInfo;
use core::str;

use dotmatrix::graphics;
use dotmatrix::hal::{DotMatrixLed, Line, ShiftRegister};
use dotmatrix::serialdrv;

use defmt_rtt as _;
use embassy_executor::Spawner;
use embassy_rp::gpio::AnyPin;
use embassy_rp::usb::Driver;
use embassy_time::{Duration, Ticker, Timer};

#[panic_handler]
fn panic(_: &PanicInfo) -> ! {
    loop {}
}

use dotmatrix::serialdrv::USB_DRIVER;
use dotmatrix::DATA;
use dotmatrix::DISPLAYS;

// static USB_DRIVER: Mutex<CriticalSectionRawMutex, Option<Driver<USB>>> =
// Mutex::new(None);

// static LED: Mutex<CriticalSectionRawMutex, Option<Output<'static, AnyPin>>> =
// Mutex::new(None);

// static DISPLAY: DotMatrixDisplayMutex<ShiftRegisterOutput> =
//     DotMatrixDisplayMutex::new();

// static DISPLAYS: Displays<ShiftRegisterOutput> = Displays::new();

// used in setting up usb-serial
// bind_interrupts!(struct Irqs {
// USBCTRL_IRQ => InterruptHandler<USB>;
// });

async fn clock() {
    if let Some(clock) = DATA.lock().await.clock {
        let string = str::from_utf8(&clock[..]).unwrap();
        DISPLAYS.panorama(string, false).await;
        for d in &*DISPLAYS {
            d.pulse().await;
        }
    }
}

async fn weather() {
    if let Some(weather) = DATA.lock().await.weather {
        let string = str::from_utf8(&weather[..]).unwrap();
        DISPLAYS.panorama(string, false).await;
        for d in &*DISPLAYS {
            d.pulse().await;
        }
    }
}

#[embassy_executor::task]
async fn animate() {
    Timer::after_secs(3).await;
    loop {
        DISPLAYS.panorama("    AKIHABARA ", false).await;
        for d in &*DISPLAYS {
            d.pulse().await;
        }
        clock().await;
        weather().await;
    }
}

#[embassy_executor::task]
async fn render_displays() {
    let mut ticker = Ticker::every(Duration::from_micros(300));
    loop {
        for d in &*DISPLAYS {
            d.render().await;
        }
        ticker.next().await;
    }
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_rp::init(Default::default());

    {
        let display0 = DotMatrixLed {
            sr: ShiftRegister {
                ser: Line::new_anode(AnyPin::from(p.PIN_2)),
                oe: Line::new_cathode(AnyPin::from(p.PIN_22)), // 適当値;未使用
                rclk: Line::new_anode(AnyPin::from(p.PIN_3)),
                srclk: Line::new_anode(AnyPin::from(p.PIN_4)),
                srclr: Line::new_cathode(AnyPin::from(p.PIN_5)),
            },
            graphic: graphics::LETTER_A,
            overridden: false,
        };

        let display1 = DotMatrixLed {
            sr: ShiftRegister {
                ser: Line::new_anode(AnyPin::from(p.PIN_6)),
                oe: Line::new_cathode(AnyPin::from(p.PIN_23)), // 適当値;未使用
                rclk: Line::new_anode(AnyPin::from(p.PIN_7)),
                srclk: Line::new_anode(AnyPin::from(p.PIN_8)),
                srclr: Line::new_cathode(AnyPin::from(p.PIN_9)),
            },
            graphic: graphics::LETTER_B,
            overridden: false,
        };

        let display2 = DotMatrixLed {
            sr: ShiftRegister {
                ser: Line::new_anode(AnyPin::from(p.PIN_10)),
                oe: Line::new_cathode(AnyPin::from(p.PIN_16)), // 適当値;未使用
                rclk: Line::new_anode(AnyPin::from(p.PIN_11)),
                srclk: Line::new_anode(AnyPin::from(p.PIN_12)),
                srclr: Line::new_cathode(AnyPin::from(p.PIN_13)),
            },
            graphic: graphics::LETTER_C,
            overridden: false,
        };

        let display3 = DotMatrixLed {
            sr: ShiftRegister {
                ser: Line::new_anode(AnyPin::from(p.PIN_21)),
                oe: Line::new_cathode(AnyPin::from(p.PIN_17)), // 適当値;未使用
                rclk: Line::new_anode(AnyPin::from(p.PIN_20)),
                srclk: Line::new_anode(AnyPin::from(p.PIN_19)),
                srclr: Line::new_cathode(AnyPin::from(p.PIN_18)),
            },
            graphic: graphics::LETTER_D,
            overridden: false,
        };

        *(DISPLAYS[0].0.lock().await) = Some(display0);
        *(DISPLAYS[1].0.lock().await) = Some(display1);
        *(DISPLAYS[2].0.lock().await) = Some(display2);
        *(DISPLAYS[3].0.lock().await) = Some(display3);
    }

    {
        let driver = Driver::new(p.USB, serialdrv::Irqs);
        *(USB_DRIVER.lock().await) = Some(driver);
    }

    let _ = spawner.spawn(render_displays());
    let _ = spawner.spawn(animate());
    let _ = spawner.spawn(serialdrv::setup_serial());
}
