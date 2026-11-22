#![no_std]
extern crate alloc;

use crate::config::Config;
use crate::error::Error;
use crate::led::{Led, Status};
use crate::peripherals::Peripherals;
use crate::runtime::Runtime;
use alloc::vec;
use embassy_time::{Duration, Instant, Timer};
use esp_hal::gpio::Level;
use esp_hal::peripherals::WIFI;
use esp_hal::rtc_cntl::sleep::{LowPower, RtcSleepConfig};
use log::{error, info};

mod config;
mod error;
mod led;
mod network;
pub mod peripherals;
pub mod runtime;
pub mod sensor;

/// Client's main logic runs here
pub async fn run(runtime: &Runtime, peripherals: Peripherals) -> ! {
    info!("run - start");

    let mut led = Led::new(peripherals.rmt, peripherals.gpio).unwrap();
    led.show_status(Status::Working).await.unwrap();

    let config = Config::new();

    match try_run(runtime, peripherals.wifi, &config).await {
        Ok(()) => {
            info!("run - success");
            led.show_status(Status::Success).await.unwrap();
            Timer::after(Duration::from_secs(2)).await
        }
        Err(error) => {
            error!("run - error: {}", error);
            led.show_status(Status::Error).await.unwrap();
            Timer::after(Duration::from_secs(2)).await
        }
    }

    info!("run - going to sleep");
    let mut low_power = LowPower::new(peripherals.low_power);
    let sleep_duration = esp_hal::time::Duration::from_secs(10);
    low_power.set_wakeup_deadline(esp_hal::time::Instant::now() + sleep_duration);
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
