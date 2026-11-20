use esp_hal::peripherals::{LPWR, WIFI};

pub struct Peripherals {
    pub wifi: WIFI<'static>,
    pub low_power: LPWR<'static>,
}

impl Peripherals {
    pub fn new(wifi: WIFI<'static>, low_power: LPWR<'static>) -> Self {
        Self { wifi, low_power }
    }
}
