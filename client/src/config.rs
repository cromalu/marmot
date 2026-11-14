use alloc::string::String;
pub struct Config {
    pub wifi: WifiConfig,
}
pub struct WifiConfig {
    pub ssid: String,
    pub psk: String,
}

impl Config {
    pub fn new() -> Config {
        Config {
            wifi: WifiConfig {
                ssid: String::new(),
                psk: String::new(),
            },
        }
    }
}
