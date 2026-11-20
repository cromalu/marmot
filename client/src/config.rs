use alloc::string::{String, ToString};
use esp_hal::time::Duration;
use reqwless::request::Method;

pub struct Config {
    pub sleep_duration: Duration,
    pub wifi: WifiConfig,
    pub http: HttpConfig,
}
pub struct WifiConfig {
    pub ssid: String,
    pub psk: String,
}

pub struct HttpConfig {
    pub domain: String,
    pub end_point: String,
    pub port: u32,
    pub method: Method,
}


impl Config {
    pub fn new() -> Config {
        Config {
            sleep_duration: Duration::from_secs(10),
            wifi: WifiConfig {
                ssid: String::new(),
                psk: String::new(),
            },
            http: HttpConfig {
                domain: String::new(),
                end_point: String::new(),
                port: 443,
                method: Method::POST,
            },

        }
    }
}