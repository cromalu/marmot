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
pub async fn run(runtime: &Runtime, peripherals: Peripherals) -> Result<(), Error> {
    log::info!("run - start");
    let config = Config::new();
    // TODO turn on activity led
    // TODO run data acquisition from sensor
    // TODO post data to remote server
    let network = network::new();
    let _init_network = network.setup(&config.wifi, peripherals.wifi, runtime.spawner).await?;
    // TODO turn off activity led
    // TODO enforce timeout
    log::info!("run - end");
    Ok(())
}
