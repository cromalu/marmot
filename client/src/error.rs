use embassy_executor::SpawnError;
use esp_hal::rmt::ConfigError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Wifi error")]
    WifiError(#[from] WifiError),
    #[error("Spawn error")]
    SpawnError(#[from] SpawnError),
    #[error("Http error")]
    HttpError(#[from] HttpError),
    #[error("Sensor error")]
    SensorError(#[from] SensorError),
}

#[derive(Error, Debug)]
pub enum WifiError {
    #[error("Wifi setup error")]
    WifiSetupError(#[from] esp_radio::wifi::WifiError),
    #[error("Wifi connection error")]
    WifiConnectionError(#[from] esp_radio::wifi::ConnectionError),
}

#[derive(Error, Debug)]
pub enum HttpError {
    #[error("Request error")]
    RequestError(#[from] reqwless::Error),
    #[error("Status error")]
    StatusError(u16),
}

#[derive(Error, Debug)]
pub enum SensorError {
    #[error("Rmt config error")]
    RmtConfigError(#[from] ConfigError),
    #[error("One wire error")]
    OneWireError(#[from] esp_hal_rmt_onewire::Error),
    #[error("Acquisition timeout error")]
    AcquisitionTimeoutError()
}
