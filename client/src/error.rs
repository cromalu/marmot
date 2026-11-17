use embassy_executor::SpawnError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Wifi error")]
    WifiError(#[from] WifiError),
    #[error("Spawn error")]
    SpawnError(#[from] SpawnError),
}

#[derive(Error, Debug)]
pub enum WifiError {
    #[error("Wifi setup error")]
    WifiSetupError(#[from] esp_radio::wifi::WifiError),
    #[error("Wifi connection error")]
    WifiConnectionError(#[from] esp_radio::wifi::ConnectionError),
}