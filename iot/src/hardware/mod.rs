mod button;
mod display;
mod sensor;

pub use button::Button;
pub use display::Display;
pub use sensor::Sensor;

use embassy_time::Timer;
use esp_hal::{
    gpio::AnyPin,
    i2c::master::AnyI2c,
    peripherals::LPWR,
    rtc_cntl::{
        Rtc,
        sleep::{Ext0WakeupSource, TimerWakeupSource, WakeupLevel},
    },
};

pub struct SystemHardware<'a> {
    display_i2c: Option<AnyI2c<'a>>,
    display_sda: Option<AnyPin<'a>>,
    display_scl: Option<AnyPin<'a>>,

    button_pin: Option<AnyPin<'a>>,

    sensor_trig_pin: Option<AnyPin<'a>>,
    sensor_echo_pin: Option<AnyPin<'a>>,

    rtc: Rtc<'a>,
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
    ) -> Self {
        Self {
            display_i2c: Some(display_i2c),
            display_sda: Some(display_sda),
            display_scl: Some(display_scl),
            button_pin: Some(button_pin),
            sensor_trig_pin: Some(trig_pin),
            sensor_echo_pin: Some(echo_pin),
            rtc: Rtc::new(lpwr),
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

    pub async fn deep_sleep(&mut self, duration: embassy_time::Duration) -> ! {
        log::info!("Entering deep sleep for {}ms", duration.as_millis());

        // HACK(pencelheimer): delay for logs to appear in the console
        Timer::after_millis(100).await;

        let timer_source = TimerWakeupSource::new(duration.into());

        let button_pin_raw = if let Some(pin) = self.button_pin.as_mut() {
            pin.reborrow()
        } else {
            panic!("Can't sleep! No pin available")
        };

        let ext_source = Ext0WakeupSource::new(button_pin_raw, WakeupLevel::Low);

        self.rtc.sleep_deep(&[&timer_source, &ext_source]);
    }
}
