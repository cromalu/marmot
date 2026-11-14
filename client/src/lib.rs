#![no_std]

use crate::runtime::Runtime;
pub mod runtime;

/// Client's main logic runs here
pub fn run(runtime: &Runtime) {
    log::info!("run - start");
    // TODO turn on activity led
    // TODO run data acquisition from sensor
    // TODO post data to remote server
    // TODO turn off activity led
    log::info!("run - end");
}
