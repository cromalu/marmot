use esp_hal::gpio::AnyPin;
use esp_hal::peripherals::{LPWR, RMT, WIFI};

pub struct Peripherals {
    pub wifi: WIFI<'static>,
    pub low_power: LPWR<'static>,
    pub rmt: RMT<'static>,
    pub gpio: AnyPin<'static>
}

impl Peripherals {
    pub fn new(
        wifi: WIFI<'static>,
        low_power: LPWR<'static>,
        rmt: RMT<'static>,
        gpio: AnyPin<'static>,
    ) -> Self {
        Self {
            wifi,
            low_power,
            rmt,
            gpio,
        }
    }
}
