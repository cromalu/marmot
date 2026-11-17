use crate::config::WifiConfig;
use crate::error::{Error, WifiError};
use embassy_executor::Spawner;
use embassy_net::{Runner, Stack};
use esp_hal::peripherals::WIFI;
use esp_hal::rng::Rng;
use esp_radio::wifi::sta::StationConfig;
use esp_radio::wifi::{AuthenticationMethodConfig, Config, Interface, Password, WifiController};
use log::info;
use static_cell::StaticCell;

pub fn new() -> UninitializedNetwork {
    UninitializedNetwork {}
}

pub struct UninitializedNetwork {}

pub struct InitializedNetwork<'a> {
    controller: WifiController<'a>,
    stack: Stack<'a>,
}

impl UninitializedNetwork {
    pub async fn setup<'a>(
        self,
        wifi_config: &WifiConfig,
        wifi_peripheral: WIFI<'static>,
        spawner: Spawner,
    ) -> Result<InitializedNetwork<'a>, Error> {
        let controller = connect_wifi(wifi_config, wifi_peripheral).await?;
        let stack = start_ip_stack(spawner).await?;
        Ok(InitializedNetwork { stack, controller })
    }
}

async fn connect_wifi(
    wifi_config: &WifiConfig,
    wifi_peripheral: WIFI<'static>,
) -> Result<WifiController<'static>, WifiError> {
    info!("network - connecting wifi...");
    let mut wifi_controller = WifiController::new(wifi_peripheral, Default::default())?;
    let station_config = Config::Station(
        StationConfig::default()
            .with_ssid(wifi_config.ssid.as_str().try_into()?)
            .with_authentication(AuthenticationMethodConfig::Wpa2Personal(Password::from(
                wifi_config.psk.as_str().try_into()?,
            ))),
    );
    wifi_controller.set_config(&station_config)?;
    wifi_controller.connect_async().await?;
    info!("network - wifi connected");
    Ok(wifi_controller)
}

async fn start_ip_stack<'a>(spawner: Spawner) -> Result<Stack<'a>, Error> {
    info!("network - starting ip stack...");
    let config = embassy_net::Config::dhcpv4(Default::default());
    let interface = Interface::station();
    let rng = Rng::new();
    let seed = (rng.random() as u64) << 32 | rng.random() as u64;

    // Init network stack
    // NOTE: DHCP and DNS need one socket slot if enabled. This is why we're
    // provisioning space for 3 sockets here: one for DHCP, one for DNS, and one application code (e.g. TCP).
    static RESOURCES: StaticCell<embassy_net::StackResources<3>> = StaticCell::new();
    let (stack, runner) = embassy_net::new(
        interface,
        config,
        RESOURCES.init(embassy_net::StackResources::new()),
        seed,
    );
    // Launch network task that runs `runner.run().await`
    spawner.spawn(net_task(runner)?);
    // Wait for DHCP config
    stack.wait_config_up().await;
    info!("network - ip stack started");
    if let Some(config) = stack.config_v4() {
        info!("network - got ip: {}", config.address);
    }
    Ok(stack)
}

#[embassy_executor::task]
async fn net_task(mut runner: Runner<'static, Interface>) {
    runner.run().await
}
