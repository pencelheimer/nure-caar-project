use embassy_time::{
    Duration, //
    Timer,
};
use esp_hal::gpio::{
    AnyPin, //
    Input,
    InputConfig,
};

pub struct BoardButton<'a> {
    input: Input<'a>,
}

impl<'a> BoardButton<'a> {
    pub fn new(pin: AnyPin<'a>) -> Self {
        Self {
            input: Input::new(pin, InputConfig::default()),
        }
    }

    pub async fn is_long_press(&mut self) -> bool {
        let check_interval = Duration::from_millis(100);
        let long_press_iterations = 5;

        for _ in 0..long_press_iterations {
            if self.input.is_high() {
                return false;
            }
            Timer::after(check_interval).await;
        }

        true
    }
}
