use crate::error::LedError;
use esp_hal::Async;
use esp_hal::gpio::AnyPin;
use esp_hal::peripherals::{GPIO, RMT};
use esp_hal::rmt::Rmt;
use esp_hal::time::Rate;
use esp_hal_smartled::{RmtSmartLeds, buffer_size, color_order};
use smart_leds::colors::{BLACK, GREEN, RED, WHITE};
use smart_leds::{RGB8, SmartLedsWriteAsync, White, brightness};

pub struct Led<'a> {
    led: RmtSmartLeds<'a, { buffer_size::<RGB8>(1) }, Async, RGB8, color_order::Grb>,
}

#[derive(Debug, Clone, Copy)]
pub enum Status {
    Working,
    Success,
    Error,
}
impl Led<'_> {
    pub fn new(rmt: RMT<'static>, gpio: AnyPin<'static>) -> Result<Self, LedError> {
        let rmt = Rmt::new(rmt, Rate::from_mhz(80))?.into_async();
        Ok(Self {
            led: RmtSmartLeds::<{ buffer_size::<RGB8>(1) }, _, RGB8, color_order::Grb>::new(
                esp_hal_smartled::WS2812_TIMING,
                rmt.channel0,
                gpio,
                Rate::from_mhz(80),
            )
            .map_err(|_| LedError::SmartLedError())?,
        })
    }

    pub async fn show_status(&mut self, status: Status) -> Result<(), LedError> {
        match status {
            Status::Working => self.set_color(WHITE).await,
            Status::Success => self.set_color(GREEN).await,
            Status::Error => self.set_color(RED).await,
        }
    }

    async fn set_color<'a>(&mut self, color: RGB8) -> Result<(), LedError> {
        self.led
            .write(brightness([color].into_iter(), 10))
            .await
            .map_err(|_| LedError::SmartLedError())
    }
}
