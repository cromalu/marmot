#![no_std]

use embassy_executor::Spawner;

/// Client's main logic runs here
pub fn run(_spawner: Spawner) {
    log::info!("run - start");
    // TODO turn on activity led
    // TODO run data acquisition from sensor
    // TODO post data to remote server
    // TODO turn off activity led
    log::info!("run - end");
}
