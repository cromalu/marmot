#![no_std]
extern crate alloc;

use crate::config::Config;
use crate::error::Error;
use crate::peripherals::Peripherals;
use crate::runtime::Runtime;
mod config;
mod error;
mod network;
pub mod peripherals;
pub mod runtime;

/// Client's main logic runs here
pub async fn run(_runtime: &Runtime, peripherals: Peripherals) -> Result<(), Error> {
    log::info!("run - start");
    let config = Config::new();
    // TODO turn on activity led
    // TODO run data acquisition from sensor
    // TODO post data to remote server
    network::connect_wifi(&config.wifi, peripherals.wifi).await?;
    // TODO turn off activity led
    log::info!("run - end");
    Ok(())
}
