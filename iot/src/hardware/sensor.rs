#![allow(unused)]

use embassy_time::{
    Duration, //
    Instant,
    Timer,
    with_timeout,
};
use esp_hal::gpio::{AnyPin, Input, InputConfig, Level, Output, OutputConfig, Pull};
use log::{
    error, //
    warn,
};

pub struct Sensor<'a> {
    trig: Output<'a>,
    echo: Input<'a>,
}

impl<'a> Sensor<'a> {
    pub fn new(trig_pin: AnyPin<'a>, echo_pin: AnyPin<'a>) -> Self {
        let trig_config = OutputConfig::default();
        let echo_config = InputConfig::default().with_pull(Pull::Down);

        Self {
            trig: Output::new(trig_pin, Level::Low, trig_config),
            echo: Input::new(echo_pin, echo_config),
        }
    }

    pub async fn measure(&mut self) -> Option<f64> {
        let distance_cm = 100.0;

        // self.trig.set_low();
        // Timer::after_micros(10).await;
        //
        // self.trig.set_high();
        // Timer::after_micros(10).await;
        // self.trig.set_low();
        //
        // let wait_high = with_timeout(Duration::from_millis(150), self.echo.wait_for_high()).await;
        // if wait_high.is_err() {
        //     error!("Sensor Error: Echo never went HIGH (check wiring/power)");
        //     return None;
        // }
        //
        // let start_time = Instant::now();
        //
        // let wait_low = with_timeout(Duration::from_millis(150), self.echo.wait_for_low()).await;
        // if wait_low.is_err() {
        //     error!("Sensor Error: Echo never went LOW (object too far?)");
        //     return None;
        // }
        //
        // let duration = Instant::now().duration_since(start_time);
        //
        // let distance_cm = duration.as_micros() as f64 / 58.0;
        //
        // if distance_cm < 20.0 {
        //     warn!("Blind zone detected (<20cm): {:.2}", distance_cm);
        // }

        Some(distance_cm)
    }
}
