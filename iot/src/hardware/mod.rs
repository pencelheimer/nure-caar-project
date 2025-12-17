mod button;
mod display;

pub use button::BoardButton as Button;
pub use display::Display;

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

    rtc: Rtc<'a>,
}

impl<'a> SystemHardware<'a> {
    pub fn new(
        display_i2c: AnyI2c<'a>,
        display_sda: AnyPin<'a>,
        display_scl: AnyPin<'a>,
        button_pin: AnyPin<'a>,
        lpwr: LPWR<'a>,
    ) -> Self {
        Self {
            display_i2c: Some(display_i2c),
            display_sda: Some(display_sda),
            display_scl: Some(display_scl),
            button_pin: Some(button_pin),
            rtc: Rtc::new(lpwr),
        }
    }

    pub fn enable_display(&mut self) -> Option<Display<'a>> {
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
