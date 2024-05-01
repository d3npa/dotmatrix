use embassy_rp::bind_interrupts;
use embassy_rp::peripherals::USB;
use embassy_rp::usb::{Driver, Instance, InterruptHandler};
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::mutex::Mutex;
use embassy_usb::class::cdc_acm::{CdcAcmClass, State};
use embassy_usb::driver::EndpointError;
use embassy_usb::Config;

pub static USB_DRIVER: Mutex<CriticalSectionRawMutex, Option<Driver<USB>>> =
    Mutex::new(None);

use crate::copy_str_bytes;
use crate::graphics::{self, Graphic};
use crate::{DATA, DISPLAYS};

use core::str;

// used in setting up usb-serial
bind_interrupts!(pub struct Irqs {
    USBCTRL_IRQ => InterruptHandler<USB>;
});

#[embassy_executor::task]
pub async fn setup_serial() {
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
                for d in &*DISPLAYS {
                    d.set_override(true).await;
                }
                DISPLAYS[0].flash(panorama, true).await;
                DISPLAYS.panorama(a, true).await;
                for d in &*DISPLAYS {
                    d.set_override(false).await;
                }
            }
            "1" => {
                //store clock
                let clock_data = match copy_str_bytes(a.as_bytes()) {
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
                let weather_data = match copy_str_bytes(a.as_bytes()) {
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
