use cyw43::{Control, NetDriver};
use cyw43_pio::PioSpi;
use embassy_executor::Spawner;
use embassy_net::{ConfigV6, Stack, StackResources};
use embassy_rp::{
    gpio::Output,
    peripherals::{DMA_CH0, PIN_23, PIN_25, PIO0},
};
use embassy_time::Timer;
use static_cell::StaticCell;

pub mod tcpserver;

include!("../../credentials.rs");

static STACK: StaticCell<Stack<NetDriver<'static>>> = StaticCell::new();
static RESOURCES: StaticCell<StackResources<2>> = StaticCell::new();

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

pub async fn configure_network(
    spawner: &Spawner,
    pwr: Output<'static, PIN_23>,
    spi: PioSpi<'static, PIN_25, PIO0, 0, DMA_CH0>,
) {
    let (net_dev, ctrl) = init_wifi(&spawner, pwr, spi).await;
    let stack = init_ip(&spawner, net_dev).await;

    while !stack.is_config_up() {
        Timer::after_millis(100).await;
    }

    tcpserver::listen(stack, ctrl).await;
}

pub async fn init_wifi(
    spawner: &Spawner,
    pwr: Output<'static, PIN_23>,
    spi: PioSpi<'static, PIN_25, PIO0, 0, DMA_CH0>,
) -> (NetDriver<'static>, Control<'static>) {
    /* init wifi */

    let fw = include_bytes!("../../cyw43-firmware/43439A0.bin");
    let clm = include_bytes!("../../cyw43-firmware/43439A0_clm.bin");

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
