use core::ops::{
    Deref, //
    DerefMut,
};
use esp_hal::{
    Blocking, //
    gpio::AnyPin,
    i2c::master::{AnyI2c, Config, I2c},
};
use ssd1306::{I2CDisplayInterface, Ssd1306, mode::BufferedGraphicsMode, prelude::*};

type InnerDisplay<'i2c> = Ssd1306<
    I2CInterface<I2c<'i2c, Blocking>>,
    DisplaySize128x64,
    BufferedGraphicsMode<DisplaySize128x64>,
>;

pub struct Display<'i2c>(InnerDisplay<'i2c>);

impl<'a> Display<'a> {
    pub fn new(i2c: AnyI2c<'a>, sda: AnyPin<'a>, scl: AnyPin<'a>) -> Option<Self> {
        let i2c = I2c::new(i2c, Config::default())
            .ok()?
            .with_sda(sda)
            .with_scl(scl);

        let interface = I2CDisplayInterface::new(i2c);
        let mut display = Ssd1306::new(interface, DisplaySize128x64, DisplayRotation::Rotate0)
            .into_buffered_graphics_mode();

        display.init().ok()?;

        Some(Self(display))
    }
}

impl<'a> Deref for Display<'a> {
    type Target = InnerDisplay<'a>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a> DerefMut for Display<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<'a> Drop for Display<'a> {
    fn drop(&mut self) {
        let _ = self.set_display_on(false);
    }
}
