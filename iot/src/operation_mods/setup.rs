use core::net::Ipv4Addr;

use crate::hardware::{Button, Config, Display, Memory, SystemHardware, WifiHandle};
use embassy_executor::Spawner;
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::mutex::Mutex;
use embassy_time::Duration;
use embedded_graphics::{
    mono_font::{MonoTextStyleBuilder, ascii::FONT_6X10},
    pixelcolor::BinaryColor,
    prelude::*,
    text::{Baseline, Text},
};
use esp_hal_dhcp_server::{simple_leaser::SimpleDhcpLeaser, structs::DhcpServerConfig};
use esp_radio::wifi::{AccessPointConfig, ModeConfig};
use heapless::String;
use picoserve::{
    AppBuilder, //
    AppRouter,
    extract::{Json, State},
    make_static,
    response::File,
    routing::{self, get, get_service}
};

const INDEX_HTML: &str = include_str!("../index.html");

type MemoryMutex = Mutex<CriticalSectionRawMutex, Memory>;

#[derive(Clone, Copy)]
struct SharedMemory(&'static MemoryMutex);

struct AppState {
    shared_memory: SharedMemory,
}

impl picoserve::extract::FromRef<AppState> for SharedMemory {
    fn from_ref(state: &AppState) -> Self {
        state.shared_memory
    }
}

struct AppProps {
    state: AppState,
}

async fn get_config(
    State(SharedMemory(memory)): State<SharedMemory>,
) -> impl picoserve::response::IntoResponse {
    let config = {
        let mem = memory.lock().await;
        mem.get_config()
    };
    Json(config)
}

async fn update_config(
    State(SharedMemory(memory)): State<SharedMemory>,
    Json(config): Json<Config>,
) -> impl picoserve::response::IntoResponse {
    {
        let mut mem = memory.lock().await;
        mem.set_config(config);
    }
    "Config Saved"
}

impl AppBuilder for AppProps {
    type PathRouter = impl routing::PathRouter;

    fn build_app(self) -> picoserve::Router<Self::PathRouter> {
        let Self { state } = self;

        picoserve::Router::new()
            .route("/", get_service(File::html(INDEX_HTML)))
            .route("/config", get(get_config).post(update_config))
            .with_state(state)
    }
}

const WEB_TASK_POOL_SIZE: usize = 2;

#[embassy_executor::task(pool_size = WEB_TASK_POOL_SIZE)]
async fn web_task(
    id: usize,
    stack: embassy_net::Stack<'static>,
    app: &'static AppRouter<AppProps>,
    config: &'static picoserve::Config<Duration>,
) -> ! {
    let mut tcp_rx = [0u8; 1024];
    let mut tcp_tx = [0u8; 1024];
    let mut http_buf = [0u8; 2048];

    picoserve::Server::new(app, config, &mut http_buf)
        .listen_and_serve(id, stack, 80, &mut tcp_rx, &mut tcp_tx)
        .await
        .into_never()
}

pub struct SetupMode<'a> {
    wifi_server: WifiHandle,
    display: Display<'a>,
    spawner: &'a Spawner,
    memory: Memory,
    button: Button<'a>,
}

impl<'a> SetupMode<'a> {
    pub fn new<'b>(hw: &'a mut SystemHardware<'b>) -> Option<Self> {
        let wifi_server = hw.wifi_server().or_else(|| {
            log::error!("SetupMode: WiFi AP init failed");
            None
        })?;

        let display = hw.display().or_else(|| {
            log::error!("SetupMode: Display init failed");
            None
        })?;

        let memory = hw.memory();

        let button = hw.button().or_else(|| {
            log::error!("SetupMode: Button missing");
            None
        })?;

        log::info!("SetupMode: Initialized");

        Some(Self {
            wifi_server,
            display,
            spawner: &hw.spawner,
            memory,
            button,
        })
    }

    async fn start_ap(&mut self) -> bool {
        log::info!("Configuring AP...");
        let controller = &mut self.wifi_server.controller;

        let ap_config = AccessPointConfig::default().with_ssid("ESP32-Setup".into());

        let config = ModeConfig::AccessPoint(ap_config);

        if let Err(e) = controller.set_config(&config) {
            log::error!("Failed to set AP config: {:?}", e);
            return false;
        }

        log::info!("Starting Wi-Fi Radio...");
        if let Err(e) = controller.start() {
            log::error!("Failed to start AP: {:?}", e);
            return false;
        }

        embassy_time::Timer::after_millis(100).await;

        log::info!("AP Started successfully!");
        true
    }

    pub async fn run(mut self) {
        log::info!("Setup Mode Started");

        if !self.start_ap().await {
            log::error!("Critical Error: Could not start AP. Retrying loop...");
        }

        log::info!("AP IP: 192.168.4.1");

        self.display.clear(BinaryColor::Off).unwrap();
        let text_style = MonoTextStyleBuilder::new()
            .font(&FONT_6X10)
            .text_color(BinaryColor::On)
            .build();

        let mut text_buffer: String<128> = String::new();
        use core::fmt::Write;
        let _ = write!(
            &mut text_buffer,
            "SETUP MODE\nSSID: ESP32-Setup\nIP: 192.168.4.1\nBrowser:\nhttp://192.168.4.1"
        );

        Text::with_baseline(
            text_buffer.as_str(),
            Point::zero(),
            text_style,
            Baseline::Top,
        )
        .draw(&mut *self.display)
        .unwrap();
        self.display.flush().unwrap();

        let shared_memory = SharedMemory(make_static!(MemoryMutex, Mutex::new(self.memory)));

        let app = make_static!(
            AppRouter<AppProps>,
            AppProps {
                state: AppState { shared_memory }
            }
            .build_app()
        );

        let config = make_static!(
            picoserve::Config<Duration>,
            picoserve::Config::new(picoserve::Timeouts {
                start_read_request: Some(Duration::from_secs(5)),
                persistent_start_read_request: Some(Duration::from_secs(1)),
                read_request: Some(Duration::from_secs(1)),
                write: Some(Duration::from_secs(1)),
            })
            .keep_connection_alive()
        );

        self.spawner.must_spawn(dhcp_server(self.wifi_server.stack));

        for id in 0..WEB_TASK_POOL_SIZE {
            self.spawner
                .must_spawn(web_task(id, self.wifi_server.stack, app, config));
        }

        log::info!("Web server started. Hold button to exit...");

        loop {
            if self.button.is_long_press().await {
                log::info!("Button held. Shutting down...");
                break;
            }
            embassy_time::Timer::after_millis(100).await;
        }

        self.display.clear(BinaryColor::Off).unwrap();
        Text::with_baseline(
            "Saved.\nTurning off...",
            Point::zero(),
            text_style,
            Baseline::Top,
        )
        .draw(&mut *self.display)
        .unwrap();
        self.display.flush().unwrap();

        if let Err(e) = self.wifi_server.controller.stop() {
            log::warn!("Failed to stop wifi: {:?}", e);
        }

        embassy_time::Timer::after_secs(2).await;
    }
}

#[embassy_executor::task]
async fn dhcp_server(stack: embassy_net::Stack<'static>) {
    let server_ip = Ipv4Addr::new(192, 168, 4, 1);
    let config = DhcpServerConfig {
        ip: server_ip,
        lease_time: Duration::from_secs(3600),
        gateways: &[],
        subnet: None,
        dns: &[server_ip],
        use_captive_portal: true,
    };

    let mut leaser = SimpleDhcpLeaser {
        start: Ipv4Addr::new(192, 168, 4, 50),
        end: Ipv4Addr::new(192, 168, 4, 200),
        leases: Default::default(),
    };

    esp_hal_dhcp_server::run_dhcp_server(stack, config, &mut leaser)
        .await
        .expect("Failed to start DHCP server");
}
