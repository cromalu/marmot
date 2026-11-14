use esp_hal::peripherals::WIFI;

pub struct Peripherals {
    pub wifi: WIFI<'static>,
}

impl Peripherals {
    pub fn new(wifi: WIFI<'static>) -> Self {
        Self { wifi }
    }
}
