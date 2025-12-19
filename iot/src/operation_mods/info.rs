use crate::hardware::{
    SystemHardware,
    Display,
};
use embedded_graphics::{
    mono_font::{MonoTextStyleBuilder, ascii::FONT_10X20},
    pixelcolor::BinaryColor,
    prelude::*,
    text::*,
};

pub struct InfoMode<'a> {
    display: Display<'a>,
}

impl<'a> InfoMode<'a> {
    pub fn new(hw: &mut SystemHardware<'a>) -> Option<Self> {
        if let Some(display) = hw.display() {
            Some(Self { display })
        } else {
            log::error!("Can't initialize display for info mode");
            None
        }
    }

    pub async fn run(&mut self) {
        self.display.clear(BinaryColor::Off).unwrap();

        let text_style = MonoTextStyleBuilder::new()
            .font(&FONT_10X20)
            .text_color(BinaryColor::On)
            .build();

        Text::with_baseline("Hello world!", Point::zero(), text_style, Baseline::Top)
            .draw(&mut *self.display)
            .unwrap();

        self.display.flush().unwrap();

        embassy_time::Timer::after_secs(5).await;
    }
}
