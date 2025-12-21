mod button;
mod display;
mod memory;
mod sensor;
mod wifi;

pub use button::Button;
pub use display::Display;
use embassy_executor::Spawner;
pub use memory::{Config, Measurement, Memory};
pub use sensor::Sensor;
pub use wifi::{NetworkStack, WifiHandle, WifiMode};

use embassy_time::Timer;
use esp_hal::{
    gpio::AnyPin,
    i2c::master::AnyI2c,
    peripherals::{LPWR, WIFI},
    rng::Rng,
    rtc_cntl::{
        Rtc,
        sleep::{Ext0WakeupSource, RtcSleepConfig, TimerWakeupSource, WakeupLevel},
    },
};

pub struct SystemHardware<'a> {
    display_i2c: Option<AnyI2c<'a>>,
    display_sda: Option<AnyPin<'a>>,
    display_scl: Option<AnyPin<'a>>,

    button_pin: Option<AnyPin<'a>>,

    sensor_trig_pin: Option<AnyPin<'a>>,
    sensor_echo_pin: Option<AnyPin<'a>>,

    memory: Option<Memory>,

    rtc: Rtc<'a>,

    spawner: Spawner,
    wifi_peripheral: Option<WIFI<'static>>,
    rng: Option<Rng>,
}

impl<'a> SystemHardware<'a> {
    pub fn new(
        display_i2c: AnyI2c<'a>,
        display_sda: AnyPin<'a>,
        display_scl: AnyPin<'a>,
        button_pin: AnyPin<'a>,
        trig_pin: AnyPin<'a>,
        echo_pin: AnyPin<'a>,
        lpwr: LPWR<'a>,
        spawner: Spawner,
        wifi_peripheral: WIFI<'static>,
    ) -> Self {
        let memory = Memory::init();
        Self {
            display_i2c: Some(display_i2c),
            display_sda: Some(display_sda),
            display_scl: Some(display_scl),
            button_pin: Some(button_pin),
            sensor_trig_pin: Some(trig_pin),
            sensor_echo_pin: Some(echo_pin),
            memory: Some(memory),
            rtc: Rtc::new(lpwr),
            spawner,
            wifi_peripheral: Some(wifi_peripheral),
            rng: Some(Rng::new()),
        }
    }

    pub fn display(&mut self) -> Option<Display<'a>> {
        if let (Some(i2c), Some(sda), Some(scl)) = (
            self.display_i2c.take(),
            self.display_sda.take(),
            self.display_scl.take(),
        ) {
            Display::new(i2c, sda, scl)
        } else {
            None
        }
    }

    pub fn button(&mut self) -> Option<Button<'_>> {
        if let Some(button_pin) = self.button_pin.as_mut() {
            Some(Button::new(button_pin.reborrow()))
        } else {
            None
        }
    }

    pub fn sensor(&mut self) -> Option<Sensor<'_>> {
        if let Some(trig_pin) = self.sensor_trig_pin.as_mut()
            && let Some(echo_pin) = self.sensor_echo_pin.as_mut()
        {
            let trig = trig_pin.reborrow();
            let echo = echo_pin.reborrow();

            Some(Sensor::new(trig, echo))
        } else {
            None
        }
    }

    pub fn memory(&mut self) -> Option<Memory> {
        self.memory.take()
    }

    pub async fn deep_sleep(&mut self, duration: embassy_time::Duration) -> ! {
        log::info!("Entering deep sleep for {}ms", duration.as_millis());

        // HACK(pencelheimer): delay for logs to appear in the console
        Timer::after_millis(100).await;

        let mut cfg = RtcSleepConfig::deep();
        cfg.set_rtc_slowmem_pd_en(false);

        let timer_source = TimerWakeupSource::new(duration.into());

        let ext_source = Ext0WakeupSource::new(
            self.button_pin.as_mut().unwrap().reborrow(),
            WakeupLevel::Low,
        );

        self.rtc.sleep(&cfg, &[&timer_source, &ext_source]);

        loop {}
    }

    fn init_wifi(&mut self, mode: WifiMode) -> Option<WifiHandle> {
        let wifi = self.wifi_peripheral.take().expect("WiFi already consumed");
        let rng = self.rng.take().expect("RNG already consumed");

        Some(wifi::init(self.spawner, rng, wifi, mode))
    }

    pub fn wifi_client(&mut self) -> Option<WifiHandle> {
        self.init_wifi(WifiMode::Client)
    }

    pub fn wifi_server(&mut self) -> Option<WifiHandle> {
        self.init_wifi(WifiMode::AccessPoint)
    }
}
