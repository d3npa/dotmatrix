#![no_std]
#![no_main]

use core::cmp::min;
use core::panic::PanicInfo;
use core::str;

use dotmatrix::graphics;
use dotmatrix::hal::{DotMatrixLed, Line, ShiftRegister};

use defmt_rtt as _;
use embassy_executor::Spawner;
use embassy_rp::gpio::AnyPin;
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::mutex::Mutex;
use embassy_time::{Duration, Ticker, Timer};

#[panic_handler]
fn panic(_: &PanicInfo) -> ! {
    loop {}
}

use dotmatrix::DISPLAYS;
static DATA: Mutex<CriticalSectionRawMutex, Data> = Mutex::new(Data::new());

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

#[derive(Debug)]
enum Error {
    Utf8,
    // DataLength,
}

type ClockString = [u8; 7];
type WeatherString = [u8; 5];

struct Data {
    clock: Option<ClockString>,
    weather: Option<WeatherString>,
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

fn copy_str_bytes<const LEN: usize>(buf: &[u8]) -> Result<[u8; LEN], Error> {
    if let Err(_) = str::from_utf8(buf) {
        return Err(Error::Utf8);
    }

    let mut out = [0u8; LEN];
    let copy_len = min(buf.len(), out.len());
    for i in 0..copy_len {
        out[i] = buf[i];
    }

    Ok(out)
}

async fn clock() {
    if let Some(clock) = DATA.lock().await.clock {
        let string = str::from_utf8(&clock[..]).unwrap();
        // panorama2("  ", false).await;
        DISPLAYS.panorama(string, false).await;
    }
}

async fn weather() {
    if let Some(weather) = DATA.lock().await.weather {
        let string = str::from_utf8(&weather[..]).unwrap();
        // panorama2("  ", false).await;
        DISPLAYS.panorama(string, false).await;
    }
}

#[embassy_executor::task]
async fn animate() {
    Timer::after_secs(5).await;
    loop {
        DISPLAYS.panorama("    AKIHABARA ", false).await;
        for d in &*DISPLAYS {
            d.pulse().await;
        }
        // DISPLAYS[1].pulse().await;
        // clock().await;
        // DISPLAYS[1].pulse().await;
        // weather().await;
        // DISPLAYS[1].pulse().await;
    }
}

#[embassy_executor::task]
async fn render_displays() {
    let mut ticker = Ticker::every(Duration::from_micros(500));
    loop {
        for d in &*DISPLAYS {
            d.render().await;
        }
        ticker.next().await;
    }
}

// async fn handle_commands<'d, T: Instance + 'd>(
//     class: &mut CdcAcmClass<'d, Driver<'d, T>>,
// ) -> Result<(), EndpointError> {
//     let mut buf = [0; 64];
//     loop {
//         let n = class.read_packet(&mut buf).await?;
//         let data = &buf[..n];
//         // find null
//         let mut null_i = n;
//         for i in 0..n {
//             if data[i] == 0 {
//                 null_i = i;
//             }
//         }
//         // string
//         let string = match str::from_utf8(&data[..null_i]) {
//             Ok(s) => s,
//             Err(_) => continue,
//         };

//         let (c, a) = string.split_at(1);
//         match c {
//             "0" => {
//                 //echo message
//                 let mut alert: [&Graphic; 8] = [&graphics::EMPTY; 8];
//                 alert[1] = &graphics::FULL;
//                 alert[3] = &graphics::FULL;
//                 alert[5] = &graphics::FULL;
//                 alert[7] = &graphics::FULL;
//                 let panorama = graphics::Panorama {
//                     graphics: alert,
//                     len: 8,
//                 };
//                 DISPLAYS[0].set_override(true).await;
//                 DISPLAYS[1].set_override(true).await;
//                 DISPLAYS[1].flash(panorama, true).await;
//                 DISPLAYS.panorama(a, true).await;
//                 DISPLAYS[0].set_override(false).await;
//                 DISPLAYS[1].set_override(false).await;
//             }
//             "1" => {
//                 //store clock
//                 let clock_data = match copy_str_bytes(a.as_bytes()) {
//                     Ok(v) => v,
//                     Err(_) => {
//                         let _ = class.write_packet(b"ERROR").await;
//                         continue;
//                     }
//                 };
//                 let mut data = DATA.lock().await;
//                 data.clock = Some(clock_data);
//             }
//             "2" => {
//                 //store weather
//                 let weather_data = match copy_str_bytes(a.as_bytes()) {
//                     Ok(v) => v,
//                     Err(_) => continue,
//                 };
//                 let mut data = DATA.lock().await;
//                 data.weather = Some(weather_data);
//             }
//             _ => {}
//         }
//     }
// }

// #[embassy_executor::task]
// async fn setup_serial() {
//     let mut config = Config::new(0xc0de, 0xcafe);
//     config.manufacturer = Some("Rei");
//     config.product = Some("Dot-Matrix");
//     config.serial_number = Some("1");
//     config.max_power = 100;
//     config.max_packet_size_0 = 64;
//     config.device_class = 0xEF;
//     config.device_sub_class = 0x02;
//     config.device_protocol = 0x01;
//     config.composite_with_iads = true;
//     let mut device_descriptor = [0; 256];
//     let mut config_descriptor = [0; 256];
//     let mut bos_descriptor = [0; 256];
//     let mut control_buf = [0; 64];
//     let mut state = State::new();
//     let driver = {
//         let mut d = USB_DRIVER.lock().await;
//         d.take().unwrap()
//     };
//     let mut builder = embassy_usb::Builder::new(
//         driver,
//         config,
//         &mut device_descriptor,
//         &mut config_descriptor,
//         &mut bos_descriptor,
//         &mut [],
//         &mut control_buf,
//     );

//     let mut class = CdcAcmClass::new(&mut builder, &mut state, 64);
//     let mut usb = builder.build();

//     let usb_fut = usb.run();

//     let serial_loop = async {
//         loop {
//             class.wait_connection().await;
//             let _ = handle_commands(&mut class).await;
//         }
//     };

//     embassy_futures::join::join(usb_fut, serial_loop).await;
// }

// #[embassy_executor::task]
// async fn blink() {
//     loop {
//         let mut led_unlocked = LED.lock().await;
//         if let Some(led) = led_unlocked.as_mut() {
//             led.set_high();
//             Timer::after_secs(1).await;
//             led.set_low();
//             Timer::after_secs(1).await;
//         }
//     }
// }

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

    // {
    //     let led = Output::new(AnyPin::from(p.PIN_14), Level::Low);
    //     *(LED.lock().await) = Some(led);
    // }

    // {
    //     let driver = Driver::new(p.USB, Irqs);
    //     *(USB_DRIVER.lock().await) = Some(driver);
    // }

    // let _ = spawner.spawn(blink());
    // let _ = spawner.spawn(setup_serial()); // serial taking too much ram??
    let _ = spawner.spawn(render_displays());
    let _ = spawner.spawn(animate());
}
