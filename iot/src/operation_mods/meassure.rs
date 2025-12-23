use embassy_net::dns::DnsSocket;
use embassy_net::tcp::client::{TcpClient, TcpClientState};
use embassy_time::{Duration, with_timeout};
use esp_radio::wifi::ClientConfig;
use heapless::Vec;
use reqwless::client::HttpClient;
use reqwless::headers::ContentType;
use reqwless::request::{Method, RequestBuilder};
use serde::Serialize;
use thiserror_no_std::Error;

use crate::hardware::{Measurement, Memory, Sensor, SystemHardware, WifiHandle};

#[derive(Error)]
enum ModeError {
    #[error("Serde error")]
    Serde(#[from] serde_json_core::ser::Error),

    #[error("Reqwless error")]
    Reqwless(#[from] reqwless::Error),

    #[error("Timeout error")]
    Timeout(#[from] embassy_time::TimeoutError),
}

const BATCH_SIZE: usize = 10;

#[derive(Serialize)]
struct TelemetryPayload {
    value: f64,
    // timestamp: String<64>,
}

pub struct MeassureMode<'b> {
    wifi_client: WifiHandle,
    memory: Memory,
    sensor: Sensor<'b>,
}

impl<'b> MeassureMode<'b> {
    pub fn new<'a>(hw: &'b mut SystemHardware<'a>) -> Option<Self> {
        let wifi_client = hw.wifi_client().or_else(|| {
            log::error!("MeassureMode: Failed to initialize. Wifi missing");
            None
        })?;
        let memory = hw.memory();
        let sensor = hw.sensor().or_else(|| {
            log::error!("MeassureMode: Failed to initialize. Sensor missing");
            None
        })?;

        log::info!("MeassureMode: Initialized successfully");
        Some(Self {
            wifi_client,
            memory,
            sensor,
        })
    }

    pub async fn run(&mut self) {
        log::info!("Measure Mode Started");

        let current_distance = match self.measure_raw().await {
            Some(val) => val,
            None => return,
        };

        self.save_local(current_distance, 0, false);

        let config = self.memory.get_config();

        let wifi_connected = self
            .connect_wifi(config.wifi_ssid.as_str(), config.wifi_pass.as_str())
            .await;

        if !wifi_connected {
            log::info!("No Wi-Fi. Exiting.");
            self.stop_wifi();
            return;
        }

        let history = self.memory.get_history();

        if history.is_empty() {
            log::warn!("History empty (unexpected). Exiting.");
            self.stop_wifi();
            return;
        }

        log::info!("Preparing to send records...");

        let payloads: Vec<TelemetryPayload, BATCH_SIZE> = history
            .iter()
            .filter(|m| !m.synced)
            .take(BATCH_SIZE)
            .map(|m| {
                let liters = config.distance_to_liters(m.distance_mm);
                TelemetryPayload {
                    value: liters,
                    // timestamp: String::new(), // TODO: NTP?
                }
            })
            .collect();

        if payloads.is_empty() {
            log::info!("No new unsynced data to send.");
            self.stop_wifi();
            return;
        }

        let sent_success = self
            .send_telemetry_batch(
                config.server_url.as_str(),
                config.api_key.as_str(),
                &payloads,
            )
            .await;

        if sent_success.is_ok() {
            log::info!("Batch sent successfully. Cleaning up history.");
            self.memory.clear_history_keeping_last();
        } else {
            log::warn!("Failed to send batch. Keeping data for next time.");
        }

        self.stop_wifi();
    }

    async fn measure_raw(&mut self) -> Option<f64> {
        match self.sensor.measure().await {
            Some(val) => {
                log::info!("Raw measurement: {} mm", val);
                Some(val)
            }
            None => {
                log::error!("Sensor error");
                None
            }
        }
    }

    fn save_local(&mut self, distance_mm: f64, timestamp: u64, synced: bool) {
        let m = Measurement {
            distance_mm,
            timestamp,
            synced,
        };
        self.memory.add_measurement(m);
        log::info!("Saved measurement locally.");
    }

    fn stop_wifi(&mut self) {
        log::info!("Stopping Wi-Fi...");
        let _ = self.wifi_client.controller.stop();
    }

    async fn connect_wifi(&mut self, ssid: &str, password: &str) -> bool {
        let controller = &mut self.wifi_client.controller;
        let stack = self.wifi_client.stack;

        log::info!("Connecting to SSID: {}", ssid);

        let client_config = ClientConfig::default()
            .with_ssid(ssid.into())
            .with_password(password.into());

        let radio_config = esp_radio::wifi::ModeConfig::Client(client_config);

        if let Err(e) = controller.set_config(&radio_config) {
            log::error!("Failed to set wifi config: {:?}", e);
            return false;
        }

        if !controller.is_started().unwrap() {
            log::info!("Starting Wi-Fi radio...");

            if let Err(e) = controller.start() {
                log::error!("Error starting Wi-Fi radio: {:?}", e);
                return false;
            }
        }

        match controller.connect() {
            Ok(_) => {
                log::info!("Wi-Fi connected! Waiting for IP...");

                match with_timeout(Duration::from_secs(10), stack.wait_config_up()).await {
                    Ok(_) => {
                        if let Some(config) = stack.config_v4() {
                            log::info!("IP Address: {}", config.address);
                            true
                        } else {
                            log::error!("DHCP failed: Config is up but empty (unexpected)");
                            false
                        }
                    }
                    Err(_) => {
                        log::error!("DHCP Timeout: Could not get IP address in time");
                        false
                    }
                }
            }

            Err(e) => {
                log::warn!("Wi-Fi connection failed: {:?}", e);
                false
            }
        }
    }

    async fn send_telemetry_batch(
        &self,
        url: &str,
        api_key: &str,
        payloads: &[TelemetryPayload],
    ) -> Result<bool, ModeError> {
        let json_string = serde_json_core::to_string::<_, 2048>(payloads).map_err(|e| {
            log::error!("{e:?}");
            e
        })?;

        let stack = self.wifi_client.stack;
        let dns = DnsSocket::new(stack);

        let tcp_state = TcpClientState::<1, 4096, 4096>::new();
        let tcp = TcpClient::new(stack, &tcp_state);

        let mut http_buf = [0u8; 4096];

        // let mut tls_rx = [0u8; 16384];
        // let mut tls_tx = [0u8; 1024];
        // let mut client = if url.starts_with("https://") {
        //     let rng = Rng::new();
        //     let tls_seed = (rng.random() as u64) | ((rng.random() as u64) << 32);
        //     let tls = TlsConfig::new(
        //         tls_seed,
        //         &mut tls_rx,
        //         &mut tls_tx,
        //         reqwless::client::TlsVerify::None,
        //     );
        //
        //     HttpClient::new_with_tls(&tcp, &dns, tls)
        // } else {
        //     HttpClient::new(&tcp, &dns)
        // };

        let mut client = HttpClient::new(&tcp, &dns);

        log::info!("Sending batch of {} items to {}", payloads.len(), url);

        let raw_request = with_timeout(
            Duration::from_millis(2000),
            client.request(Method::POST, url),
        )
        .await
        .map_err(|e| {
            log::error!("{e:?}");
            e
        })?
        .map_err(|e| {
            log::error!("{e:?}");
            e
        })?;

        let headers: Vec<_, 1> = Vec::from_slice(&[("x-api-key", api_key)]).unwrap();
        let mut request = raw_request
            .content_type(ContentType::ApplicationJson)
            .body(json_string.as_bytes())
            .headers(&headers);

        let resp = with_timeout(Duration::from_millis(2000), request.send(&mut http_buf))
            .await
            .map_err(|e| {
                log::error!("{e:?}");
                e
            })?
            .map_err(|e| {
                log::error!("{e:?}");
                e
            })?;

        log::info!("Status: {:?}", resp.status);
        Ok::<bool, ModeError>(resp.status.is_successful())
    }
}
