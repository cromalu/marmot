use alloc::string::String;
use reqwless::request::Method;

pub struct Config {
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
