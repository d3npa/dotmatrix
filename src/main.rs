#![no_std]
#![no_main]

use core::panic::PanicInfo;
use core::str;
use defmt_rtt as _;
use embassy_executor::Spawner;
use embassy_rp::gpio::{AnyPin, Level, Output};
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::mutex::Mutex;
use embassy_time::{Duration, Ticker, Timer};

use embassy_rp::bind_interrupts;
use embassy_rp::peripherals::USB;
use embassy_rp::usb::{Driver, Instance, InterruptHandler};
use embassy_usb::class::cdc_acm::{CdcAcmClass, State};
use embassy_usb::driver::EndpointError;
use embassy_usb::Config;

use dotmatrix::graphics::{self, Graphic};
use dotmatrix::Line;
use dotmatrix::{Display, DotMatrixDisplayMutex};
use dotmatrix::{GpioOutput, ShiftRegisterOutput};

#[panic_handler]
fn panic(_: &PanicInfo) -> ! {
    loop {}
}

static DATA: Mutex<CriticalSectionRawMutex, Data> = Mutex::new(Data::new());

static USB_DRIVER: Mutex<CriticalSectionRawMutex, Option<Driver<USB>>> =
    Mutex::new(None);

static LED: Mutex<CriticalSectionRawMutex, Option<Output<'static, AnyPin>>> =
    Mutex::new(None);

static DISPLAY: DotMatrixDisplayMutex<ShiftRegisterOutput> =
    DotMatrixDisplayMutex::new();

// used in setting up usb-serial
bind_interrupts!(struct Irqs {
    USBCTRL_IRQ => InterruptHandler<USB>;
});

struct Data {
    clock: Option<ClockData>,
    weather: Option<WeatherData>,
}

#[derive(Debug)]
enum Error {
    Utf8,
    DataLength,
}

#[derive(Copy, Clone)]
struct ClockData([u8; 7]);
impl ClockData {
    fn try_from(bytes: &[u8]) -> Result<Self, Error> {
        if bytes.len() != 7 {
            return Err(Error::DataLength);
        }
        let mut clock_data = [0u8; 7];
        for i in 0..clock_data.len() {
            clock_data[i] = bytes[i];
        }
        match str::from_utf8(&clock_data[..]) {
            Ok(_) => Ok(Self(clock_data)),
            Err(_) => Err(Error::Utf8),
        }
    }
}

#[derive(Copy, Clone)]
struct WeatherData([u8; 5]);
impl WeatherData {
    fn try_from(bytes: &[u8]) -> Result<Self, Error> {
        if bytes.len() != 5 {
            return Err(Error::DataLength);
        }
        let mut data = [0u8; 5];
        for i in 0..data.len() {
            data[i] = bytes[i];
        }
        match str::from_utf8(&data[..]) {
            Ok(_) => Ok(Self(data)),
            Err(_) => Err(Error::Utf8),
        }
    }
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

async fn clock() {
    if let Some(clock) = DATA.lock().await.clock {
        let string = str::from_utf8(&clock.0[..]).unwrap();
        // panorama2("  ", false).await;
        DISPLAY.panorama2(string, false).await;
    }
}

async fn weather() {
    if let Some(weather) = DATA.lock().await.weather {
        let string = str::from_utf8(&weather.0[..]).unwrap();
        // panorama2("  ", false).await;
        DISPLAY.panorama2(string, false).await;
    }
}

#[embassy_executor::task]
async fn animate() {
    let mut ticker = Ticker::every(Duration::from_micros(100));
    loop {
        DISPLAY.panorama2(" AKIHABARA ", false).await;
        DISPLAY.pulse().await;
        clock().await;
        DISPLAY.pulse().await;
        weather().await;
        DISPLAY.pulse().await;
        ticker.next().await;
    }
}

#[embassy_executor::task]
async fn render_display() {
    let mut ticker = Ticker::every(Duration::from_micros(100));
    loop {
        DISPLAY.render().await;
        ticker.next().await;
    }
}

async fn handle_commands<'d, T: Instance + 'd>(
    class: &mut CdcAcmClass<'d, Driver<'d, T>>,
) -> Result<(), EndpointError> {
    let mut buf = [0; 64];
    loop {
        let n = class.read_packet(&mut buf).await?;
        let data = &buf[..n];
        // find null
        let mut null_i = n;
        for i in 0..n {
            if data[i] == 0 {
                null_i = i;
            }
        }
        // string
        let string = match str::from_utf8(&data[..null_i]) {
            Ok(s) => s,
            Err(_) => continue,
        };

        let (c, a) = string.split_at(1);
        match c {
            "0" => {
                //echo message
                let mut alert: [&Graphic; 8] = [&graphics::EMPTY; 8];
                alert[1] = &graphics::FULL;
                alert[3] = &graphics::FULL;
                alert[5] = &graphics::FULL;
                alert[7] = &graphics::FULL;
                let panorama = graphics::Panorama {
                    graphics: alert,
                    len: 8,
                };
                DISPLAY.set_override(true).await;
                DISPLAY.flash(panorama, true).await;
                DISPLAY.panorama2(a, true).await;
                DISPLAY.set_override(false).await;
            }
            "1" => {
                //store clock
                let clock_data = match ClockData::try_from(a.as_bytes()) {
                    Ok(v) => v,
                    Err(_) => {
                        let _ = class.write_packet(b"ERROR").await;
                        continue;
                    }
                };
                let mut data = DATA.lock().await;
                data.clock = Some(clock_data);
            }
            "2" => {
                //store weather
                let weather_data = match WeatherData::try_from(a.as_bytes()) {
                    Ok(v) => v,
                    Err(_) => continue,
                };
                let mut data = DATA.lock().await;
                data.weather = Some(weather_data);
            }
            _ => {}
        }
    }
}

#[embassy_executor::task]
async fn setup_serial() {
    let mut config = Config::new(0xc0de, 0xcafe);
    config.manufacturer = Some("Rei");
    config.product = Some("Dot-Matrix");
    config.serial_number = Some("1");
    config.max_power = 100;
    config.max_packet_size_0 = 64;
    config.device_class = 0xEF;
    config.device_sub_class = 0x02;
    config.device_protocol = 0x01;
    config.composite_with_iads = true;
    let mut device_descriptor = [0; 256];
    let mut config_descriptor = [0; 256];
    let mut bos_descriptor = [0; 256];
    let mut control_buf = [0; 64];
    let mut state = State::new();
    let driver = {
        let mut d = USB_DRIVER.lock().await;
        d.take().unwrap()
    };
    let mut builder = embassy_usb::Builder::new(
        driver,
        config,
        &mut device_descriptor,
        &mut config_descriptor,
        &mut bos_descriptor,
        &mut [],
        &mut control_buf,
    );

    let mut class = CdcAcmClass::new(&mut builder, &mut state, 64);
    let mut usb = builder.build();

    let usb_fut = usb.run();

    let serial_loop = async {
        loop {
            class.wait_connection().await;
            let _ = handle_commands(&mut class).await;
        }
    };

    embassy_futures::join::join(usb_fut, serial_loop).await;
}

#[embassy_executor::task]
async fn blink() {
    loop {
        let mut led_unlocked = LED.lock().await;
        if let Some(led) = led_unlocked.as_mut() {
            led.set_high();
            Timer::after_secs(1).await;
            led.set_low();
            Timer::after_secs(1).await;
        }
    }
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_rp::init(Default::default());

    {
        let display = Display {
            // output_driver: GpioOutput {
            //     rows: [
            //         Line::new_anode(AnyPin::from(p.PIN_9)),
            //         Line::new_anode(AnyPin::from(p.PIN_14)),
            //         Line::new_anode(AnyPin::from(p.PIN_8)),
            //         Line::new_anode(AnyPin::from(p.PIN_12)),
            //         Line::new_anode(AnyPin::from(p.PIN_1)),
            //         Line::new_anode(AnyPin::from(p.PIN_7)),
            //         Line::new_anode(AnyPin::from(p.PIN_2)),
            //         Line::new_anode(AnyPin::from(p.PIN_5)),
            //     ],
            //     cols: [
            //         Line::new_cathode(AnyPin::from(p.PIN_13)),
            //         Line::new_cathode(AnyPin::from(p.PIN_3)),
            //         Line::new_cathode(AnyPin::from(p.PIN_4)),
            //         Line::new_cathode(AnyPin::from(p.PIN_10)),
            //         Line::new_cathode(AnyPin::from(p.PIN_6)),
            //         Line::new_cathode(AnyPin::from(p.PIN_11)),
            //         Line::new_cathode(AnyPin::from(p.PIN_15)),
            //         Line::new_cathode(AnyPin::from(p.PIN_16)),
            //     ],
            // },
            output_driver: ShiftRegisterOutput {
                ser: Line::new_anode(AnyPin::from(p.PIN_2)),
                oe: Line::new_cathode(AnyPin::from(p.PIN_22)), // 適当値;未使用
                rclk: Line::new_anode(AnyPin::from(p.PIN_3)),
                srclk: Line::new_anode(AnyPin::from(p.PIN_4)),
                srclr: Line::new_cathode(AnyPin::from(p.PIN_5)),
            },
            graphic: graphics::EMPTY,
            overridden: false,
        };
        *(DISPLAY.0.lock().await) = Some(display);
    }

    let mut power = Output::new(p.PIN_1, Level::Low);
    power.set_high();

    {
        let led = Output::new(AnyPin::from(p.PIN_14), Level::Low);
        *(LED.lock().await) = Some(led);
    }

    {
        let driver = Driver::new(p.USB, Irqs);
        *(USB_DRIVER.lock().await) = Some(driver);
    }

    let _ = spawner.spawn(blink());
    let _ = spawner.spawn(setup_serial());
    let _ = spawner.spawn(render_display());
    let _ = spawner.spawn(animate());
}
