use crate::config::WifiConfig;
use crate::error::WifiError;
use esp_hal::peripherals::WIFI;
use esp_radio::wifi::sta::StationConfig;
use esp_radio::wifi::{AuthenticationMethodConfig, Config, Password};
use log::info;

pub async fn connect_wifi(
    wifi_config: &WifiConfig,
    wifi_peripheral: WIFI<'static>,
) -> Result<(), WifiError> {
    info!("network - connecting wifi...");
    let mut wifi_controller =
        esp_radio::wifi::WifiController::new(wifi_peripheral, Default::default())?;
    let station_config = Config::Station(
        StationConfig::default()
            .with_ssid(wifi_config.ssid.as_str().try_into()?)
            .with_authentication(AuthenticationMethodConfig::Wpa2Personal(Password::from(
                wifi_config.psk.as_str().try_into()?,
            ))),
    );
    wifi_controller.set_config(&station_config)?;
    wifi_controller.set_power_saving(esp_radio::wifi::PowerSaveMode::Maximum)?;
    wifi_controller.connect_async().await?;
    info!("network - wifi connected");
    Ok(())
}
