#![no_std]
#![no_main]

use core::panic::PanicInfo;
use core::str;

use cyw43::NetDriver;
use dotmatrix::graphics;
use dotmatrix::hal::{DotMatrixLed, Line, ShiftRegister};

use defmt_rtt as _;
use embassy_executor::Spawner;
use embassy_net::tcp::TcpSocket;
use embassy_rp::gpio::AnyPin;
use embassy_time::{Duration, Ticker, Timer};

use cyw43_pio::PioSpi;
use embassy_net::{ConfigV6, Stack, StackResources};
use embassy_rp::gpio::Level;
use embassy_rp::gpio::Output;
use embassy_rp::peripherals::{DMA_CH0, PIN_23, PIN_25, PIO0};

use embassy_rp::pio::Pio;
use static_cell::StaticCell;

use dotmatrix::tcpserver;
use dotmatrix::DATA;
use dotmatrix::DISPLAYS;

use embassy_rp::bind_interrupts;
use embassy_rp::pio::InterruptHandler;

include!("../credentials.rs");

#[panic_handler]
fn panic(_: &PanicInfo) -> ! {
    loop {}
}

bind_interrupts!(struct Irqs2 {
    PIO0_IRQ_0 => InterruptHandler<PIO0>;
});

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
        DISPLAYS.panorama("AKIHABARA", false).await;
        for d in &*DISPLAYS {
            d.pulse().await;
        }
        clock().await;
        weather().await;
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

#[embassy_executor::task]
async fn wifi_task(
    runner: cyw43::Runner<
        'static,
        Output<'static, PIN_23>,
        PioSpi<'static, PIN_25, PIO0, 0, DMA_CH0>,
    >,
) -> ! {
    runner.run().await
}

#[embassy_executor::task]
async fn net_task(stack: &'static Stack<cyw43::NetDriver<'static>>) -> ! {
    stack.run().await
}

#[embassy_executor::task]
async fn blinky(mut led: Output<'static, AnyPin>) -> ! {
    let delay = Duration::from_secs(1);
    loop {
        led.set_high();
        Timer::after(delay).await;
        led.set_low();
        Timer::after(delay).await;
    }
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_rp::init(Default::default());

    {
        let display0 = DotMatrixLed {
            sr: ShiftRegister {
                ser: Line::new_anode(AnyPin::from(p.PIN_2)),
                // oe: Line::new_cathode(AnyPin::from(p.PIN_22)), // 適当値;未使用
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
                // oe: Line::new_cathode(AnyPin::from(p.PIN_23)), // 適当値;未使用
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
                // oe: Line::new_cathode(AnyPin::from(p.PIN_16)), // 適当値;未使用
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
                // oe: Line::new_cathode(AnyPin::from(p.PIN_17)), // 適当値;未使用
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

    let _ = spawner.spawn(render_displays());
    let _ = spawner.spawn(animate());
    let led = Output::new(AnyPin::from(p.PIN_14), Level::Low);
    let _ = spawner.spawn(blinky(led));

    {
        // network code

        let pwr = Output::new(p.PIN_23, Level::Low);
        let cs = Output::new(p.PIN_25, Level::High);
        let mut pio = Pio::new(p.PIO0, Irqs2);
        let spi = PioSpi::new(
            &mut pio.common,
            pio.sm0,
            pio.irq0,
            cs,
            p.PIN_24,
            p.PIN_29,
            p.DMA_CH0,
        );

        configure_network(&spawner, pwr, spi).await;
    }
}

use cyw43::Control;
use embedded_io_async::Write;

pub async fn init_wifi(
    spawner: &Spawner,
    pwr: Output<'static, PIN_23>,
    spi: PioSpi<'static, PIN_25, PIO0, 0, DMA_CH0>,
) -> (NetDriver<'static>, Control<'static>) {
    /* init wifi */

    let fw = include_bytes!("../cyw43-firmware/43439A0.bin");
    let clm = include_bytes!("../cyw43-firmware/43439A0_clm.bin");

    static STATE: StaticCell<cyw43::State> = StaticCell::new();
    let state = STATE.init(cyw43::State::new());
    let (dev, mut ctrl, runner) = cyw43::new(state, pwr, spi, fw).await;

    let _ = spawner.spawn(wifi_task(runner));

    ctrl.init(clm).await;
    ctrl.set_power_management(cyw43::PowerManagementMode::PowerSave)
        .await;

    ctrl.gpio_set(0, true).await;
    Timer::after_secs(1).await;
    ctrl.gpio_set(0, false).await;

    if ctrl.join_wpa2(WIFI_NETWORK, WIFI_PASSWORD).await.is_err() {
        /* require reboot when failed to join wifi. network code won't run
        but other tasks, which were already spawned, will continue */
        todo!(
            "exit async task without disrupting others. maybe using Result?"
        );
    }

    ctrl.gpio_set(0, true).await;
    Timer::after_secs(1).await;
    ctrl.gpio_set(0, false).await;

    (dev, ctrl)
}

static STACK: StaticCell<Stack<NetDriver<'static>>> = StaticCell::new();
static RESOURCES: StaticCell<StackResources<2>> = StaticCell::new();

pub async fn init_ip(
    spawner: &Spawner,
    net_dev: NetDriver<'static>,
) -> &'static Stack<NetDriver<'static>> {
    /* init ip stack */
    use embassy_net::{
        Ipv4Address, Ipv4Cidr, Ipv6Address, Ipv6Cidr, StaticConfigV4,
    };

    use heapless::Vec;

    let ipv4_config = StaticConfigV4 {
        address: Ipv4Cidr::new(Ipv4Address::new(192, 168, 1, 5), 24),
        dns_servers: Vec::new(),
        gateway: Some(Ipv4Address::new(192, 168, 1, 1)),
    };

    let mut dns_servers = Vec::<Ipv6Address, 3>::new();
    dns_servers
        .push(Ipv6Address::new(
            0xfd00, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0001,
        ))
        .unwrap();

    let ipv6_config = embassy_net::StaticConfigV6 {
        address: Ipv6Cidr::new(
            Ipv6Address::new(0xfd00, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0005),
            64,
        ),
        gateway: Some(Ipv6Address::new(
            0xfd00, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0001,
        )),
        dns_servers,
    };

    let mut config = embassy_net::Config::ipv4_static(ipv4_config);
    config.ipv6 = ConfigV6::Static(ipv6_config);

    let seed = 0x0123_4567_89ab_cdef;

    // Init network stack

    let stack = &*STACK.init(Stack::new(
        net_dev,
        config,
        RESOURCES.init(StackResources::<2>::new()),
        seed,
    ));

    let _ = spawner.spawn(net_task(stack));
    stack
}

pub async fn configure_network(
    spawner: &Spawner,
    pwr: Output<'static, PIN_23>,
    spi: PioSpi<'static, PIN_25, PIO0, 0, DMA_CH0>,
) {
    let (net_dev, mut ctrl) = init_wifi(&spawner, pwr, spi).await;
    let stack = init_ip(&spawner, net_dev).await;

    while !stack.is_config_up() {
        Timer::after_millis(100).await;
    }

    let mut rx_buffer = [0; 4096];
    let mut tx_buffer = [0; 4096];
    let mut buf = [0; 4096];

    loop {
        let mut socket = TcpSocket::new(stack, &mut rx_buffer, &mut tx_buffer);
        socket.set_timeout(Some(Duration::from_secs(10)));

        ctrl.gpio_set(0, false).await;

        if socket.accept(1234).await.is_err() {
            continue;
        }

        ctrl.gpio_set(0, true).await;

        if let Err(_e) = socket.write_all(b"[*] welcome~\n").await {
            break;
        }

        loop {
            let n = match socket.read(&mut buf).await {
                Ok(0) => break, // eof
                Ok(n) => n,
                Err(_e) => break,
            };

            buf[n] = 0;

            if let Ok(string) = dotmatrix::null_term_string(&buf) {
                if string.is_empty() {
                    continue;
                }

                let status = tcpserver::handle_command(string.trim()).await;
                if socket.write_all(&status.mesg).await.is_err() {
                    break;
                }
            }
        }
    }
}
