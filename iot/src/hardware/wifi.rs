use embassy_executor::Spawner;
use embassy_net::{Config, Ipv4Address, Ipv4Cidr, Runner, Stack, StackResources, StaticConfigV4};
use esp_hal::peripherals::WIFI;
use esp_hal::rng::Rng;
use esp_radio::wifi::{WifiController, WifiDevice};
use static_cell::StaticCell;

#[derive(Clone, Copy, PartialEq)]
pub enum WifiMode {
    Client,
    AccessPoint,
}

pub type NetworkStack = Stack<'static>;

pub struct WifiHandle {
    pub controller: WifiController<'static>,
    pub stack: Stack<'static>,
}

macro_rules! mk_static {
    ($t:ty,$val:expr) => {{
        static STATIC_CELL: StaticCell<$t> = StaticCell::new();
        #[deny(unused_attributes)]
        let x = STATIC_CELL.uninit().write(($val));
        x
    }};
}

#[embassy_executor::task]
async fn net_task(mut runner: Runner<'static, WifiDevice<'static>>) {
    runner.run().await
}

pub fn init(spawner: Spawner, rng: Rng, wifi: WIFI<'static>, mode: WifiMode) -> WifiHandle {
    let radio_init = &*mk_static!(
        esp_radio::Controller<'static>,
        esp_radio::init().expect("Failed to initialize Wi-Fi controller")
    );

    let (wifi_controller, interfaces) = esp_radio::wifi::new(radio_init, wifi, Default::default())
        .expect("Failed to create wifi interfaces");

    let (interface, config, seed_offset) = match mode {
        WifiMode::Client => {
            let interface = interfaces.sta;
            let config = Config::dhcpv4(Default::default());
            (interface, config, 0)
        }
        WifiMode::AccessPoint => {
            let interface = interfaces.ap;

            let config = Config::ipv4_static(StaticConfigV4 {
                address: Ipv4Cidr::new(Ipv4Address::new(192, 168, 4, 1), 24),
                gateway: Some(Ipv4Address::new(192, 168, 4, 1)),
                dns_servers: Default::default(),
            });
            (interface, config, 1)
        }
    };

    let net_seed = (rng.random() as u64) | ((rng.random() as u64) << 32);

    let (stack, runner) = embassy_net::new(
        interface,
        config,
        mk_static!(StackResources<8>, StackResources::<8>::new()),
        net_seed + seed_offset,
    );

    spawner.must_spawn(net_task(runner));

    WifiHandle {
        controller: wifi_controller,
        stack,
    }
}
