#![no_std]
extern crate alloc;

use crate::config::Config;
use crate::error::Error;
use crate::peripherals::Peripherals;
use crate::runtime::Runtime;
use alloc::vec;
use esp_hal::peripherals::WIFI;
use esp_hal::rtc_cntl::sleep::{LowPower, RtcSleepConfig};
use esp_hal::time::{Duration, Instant};
use log::{error, info};

mod config;
mod error;
mod network;
pub mod peripherals;
pub mod runtime;
pub mod sensor;

/// Client's main logic runs here
pub async fn run(runtime: &Runtime, peripherals: Peripherals) -> ! {
    info!("run - start");
    let config = Config::new();

    match try_run(runtime, peripherals.wifi, &config).await {
        Ok(()) => {
            info!("run - success");
        }
        Err(error) => {
            error!("run - error: {}", error);
        }
    }
    info!("run - going to sleep");
    let mut low_power = LowPower::new(peripherals.low_power);
    let sleep_duration = Duration::from_secs(10);
    low_power.set_wakeup_deadline(Instant::now() + sleep_duration);
    low_power.sleep_deep(RtcSleepConfig::deep());
}

pub async fn try_run(
    runtime: &Runtime,
    wifi_peripheral: WIFI<'static>,
    config: &Config,
) -> Result<(), Error> {
    // TODO turn on activity led
    // TODO run data acquisition from sensor
    // TODO post data to remote server
    network::new()
        .setup(&config.wifi, wifi_peripheral, runtime.spawner)
        .await?
        .post_data(&config.http, vec![0x31, 0x32, 0x33, 0x34])
        .await?;
    // TODO turn off activity led
    // TODO enforce timeout
    Ok(())
}
