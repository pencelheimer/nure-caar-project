use core::mem::MaybeUninit;
use core::ptr;
use embedded_storage::{ReadStorage, Storage};
use esp_hal::ram;
use heapless::{String, Vec};

use serde::{Deserialize, Serialize};

const MAGIC_NUMBER: u32 = 0x80085;
const HISTORY_SIZE: usize = 20;

const FLASH_ADDR: u32 = 0x9000;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Config {
    pub tank_height_mm: f64,
    pub tank_volume_l: f64,
    pub sensor_offset_mm: f64,
    pub wifi_ssid: String<32>,
    pub wifi_pass: String<64>,
    pub server_url: String<64>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            tank_height_mm: 2000.0,
            tank_volume_l: 200.0,
            sensor_offset_mm: 0.0,
            wifi_ssid: String::try_from("csnz").unwrap(),
            wifi_pass: String::try_from("csnz01087CSNZ").unwrap(),
            server_url: String::try_from("http://192.168.0.244:3000/devices/measurements").unwrap(),
        }
    }
}

impl Config {
    pub fn distance_to_percent(&self, dist_mm: f64) -> u8 {
        if dist_mm <= self.sensor_offset_mm {
            return 100;
        }
        if dist_mm >= self.tank_height_mm {
            return 0;
        }
        let water_depth = self.tank_height_mm - dist_mm;
        let max_depth = self.tank_height_mm - self.sensor_offset_mm;
        if max_depth == 0.0 {
            return 0;
        }
        ((water_depth as u32 * 100) / max_depth as u32) as u8
    }

    pub fn distance_to_liters(&self, dist_mm: f64) -> f64 {
        let percent = self.distance_to_percent(dist_mm);
        (self.tank_volume_l * percent as f64) / 100.0
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Measurement {
    pub distance_mm: f64,
    pub timestamp: u64,
    pub synced: bool,
}

#[repr(C)]
#[derive(Debug)]
struct RtcState {
    magic: u32,
    config: Config,
    measurements: Vec<Measurement, HISTORY_SIZE>,
}

impl Default for RtcState {
    fn default() -> Self {
        Self {
            magic: MAGIC_NUMBER,
            config: Config::default(),
            measurements: Vec::new(),
        }
    }
}

#[ram(unstable(rtc_slow))]
static mut RTC_STORAGE: MaybeUninit<RtcState> = MaybeUninit::uninit();

pub struct Memory {
    _marker: (),
}

impl Memory {
    pub fn init() -> Self {
        unsafe {
            let state_ptr = (*(&raw mut RTC_STORAGE)).as_mut_ptr();
            let magic_ptr = core::ptr::addr_of!((*state_ptr).magic);

            if magic_ptr.read_volatile() == MAGIC_NUMBER {
                log::info!("Memory: Wakeup. RTC is valid.");
            } else {
                log::info!("Memory: Cold boot. Initializing...");

                let loaded_config = Self::load_config_from_flash().unwrap_or_else(|_| {
                    log::warn!("Flash config invalid or empty. Using Defaults.");
                    let def = Config::default();
                    let _ = Self::save_config_to_flash(&def);
                    def
                });

                let new_state = RtcState {
                    magic: MAGIC_NUMBER,
                    config: loaded_config,
                    measurements: Vec::new(),
                };

                ptr::write(state_ptr, new_state);
            }
        }
        Self { _marker: () }
    }

    fn load_config_from_flash() -> Result<Config, ()> {
        let mut flash = esp_storage::FlashStorage::new();
        let mut buffer = [0u8; 256];

        if flash.read(FLASH_ADDR, &mut buffer).is_err() {
            return Err(());
        }

        postcard::from_bytes::<Config>(&buffer).map_err(|_| ())
    }

    fn save_config_to_flash(cfg: &Config) -> Result<(), ()> {
        let mut flash = esp_storage::FlashStorage::new();
        let mut buffer = [0u8; 256];

        if let Ok(slice) = postcard::to_slice(cfg, &mut buffer) {
            if flash.write(FLASH_ADDR, slice).is_err() {
                log::error!("Failed to write flash");
                return Err(());
            }
            Ok(())
        } else {
            Err(())
        }
    }

    pub fn get_config(&self) -> Config {
        unsafe { (*(*(&raw const RTC_STORAGE)).as_ptr()).config.clone() }
    }

    pub fn set_config(&mut self, cfg: Config) {
        unsafe {
            (*(*(&raw mut RTC_STORAGE)).as_mut_ptr()).config = cfg.clone();

            if Self::save_config_to_flash(&cfg).is_ok() {
                log::info!("Config saved to Flash");
            } else {
                log::error!("Config save failed!");
            }
        }
    }

    pub fn add_measurement(&mut self, m: Measurement) {
        unsafe {
            let state = (*(&raw mut RTC_STORAGE)).as_mut_ptr();
            if (*state).measurements.push(m).is_err() {
                (*state).measurements.remove(0);
                let _ = (*state).measurements.push(m);
            }
        }
    }

    pub fn get_last_measurement(&self) -> Option<Measurement> {
        unsafe {
            let state = (*(&raw const RTC_STORAGE)).as_ptr();
            (*state).measurements.last().cloned()
        }
    }

    pub fn get_history(&self) -> &[Measurement] {
        unsafe {
            let state = (*(&raw const RTC_STORAGE)).as_ptr();
            (*state).measurements.as_slice()
        }
    }

    pub fn clear_history_keeping_last(&mut self) {
        unsafe {
            let state = (*(&raw mut RTC_STORAGE)).as_mut_ptr();

            let last = self.get_last_measurement();

            (*state).measurements.clear();

            if let Some(last) = last {
                self.add_measurement(last);
            }
        }
    }
}
