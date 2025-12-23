use crate::hardware::{Display, Memory, SystemHardware};

use core::fmt::Write;

use embedded_graphics::{
    mono_font::{MonoTextStyleBuilder, ascii::FONT_10X20},
    pixelcolor::BinaryColor,
    prelude::*,
    text::*,
};
use heapless::String;

pub struct InfoMode<'a> {
    display: Display<'a>,
    memory: Memory,
}

impl<'a> InfoMode<'a> {
    pub fn new(hw: &mut SystemHardware<'a>) -> Option<Self> {
        let display = if let Some(display) = hw.display() {
            display
        } else {
            log::error!("InfoMode: Failed to initialize. Display missing");
            return None;
        };

        let memory = hw.memory();

        log::info!("InfoMode: Initialized successfully");
        Some(Self { display, memory })
    }

    pub async fn run(&mut self) {
        log::info!("Info Mode Started");

        self.display.clear(BinaryColor::Off).unwrap();

        let text_style = MonoTextStyleBuilder::new()
            .font(&FONT_10X20)
            .text_color(BinaryColor::On)
            .build();

        let last_measurement = self.memory.get_last_measurement();
        let config = self.memory.get_config();

        let mut text_buffer: String<64> = String::new();
        match last_measurement {
            Some(m) => {
                log::info!("Found measurement {} mm", m.distance_mm);

                let percent = config.distance_to_percent(m.distance_mm);
                let liters = config.distance_to_liters(m.distance_mm);

                if write!(&mut text_buffer, "Level: {}%\n{}L", percent, liters).is_err() {
                    log::error!("String buffer overflow");
                }
            }
            None => {
                log::info!("No history found");

                if write!(&mut text_buffer, "No data yet\nWaiting...").is_err() {
                    log::error!("String buffer overflow");
                }
            }
        }

        Text::with_baseline(
            text_buffer.as_str(),
            Point::zero(),
            text_style,
            Baseline::Top,
        )
        .draw(&mut *self.display)
        .unwrap();

        self.display.flush().unwrap();

        log::info!("Display updated. Holding for 5s...");
        embassy_time::Timer::after_secs(5).await;

        log::info!("Info Mode finished");
    }
}
