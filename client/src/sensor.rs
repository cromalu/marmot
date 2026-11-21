use crate::error::SensorError;
use embassy_time::{Duration, Instant, Timer};
use esp_hal::Async;
use esp_hal::gpio::AnyPin;
use esp_hal::peripherals::RMT;
use esp_hal::rmt::{Channel, Rmt, Rx, Tx};
use esp_hal::time::Rate;
use esp_hal::tsens::Temperature;
use esp_hal_rmt_onewire::{OneWire, OneWireConfigZST};
use fixed::types::I12F4;

/// reads temperature from ds18b20 sensor
async fn read_temperature(rmt: RMT<'static>, pin: AnyPin<'static>) -> Result<I12F4, SensorError> {
    // Wiring gpio pin on rmt driver and wrap rmt driver into one-wire driver
    let rmt = Rmt::new(rmt, Rate::from_mhz(80_u32))?.into_async();
    let mut ow: OneWire<'static, OneWireConfigZST<Channel<Async, Rx>, Channel<'_, Async, Tx>, Tx>> = OneWire::new(rmt.channel0, rmt.channel2, pin)?;

    // one wire protocol sequence should be:

    // 1. issue reset pulse
    ow.reset().await?;

    // 2. issue 'skip rom' command (CC) (considering there is a single sensor on the bus)
    ow.send_byte(0xCC).await?;

    // 3. issue 'convert temperature' command (44)
    ow.send_byte(0x44).await?;

    // 4. read time slot until response is one / or wait for 750 ms for max resolution
    let deadline = Instant::now() + Duration::from_millis(800);
    loop {
        let [done] = ow.exchange_bits([true]).await?;
        if done {
            break;
        }

        if Instant::now() >= deadline {
            // Handle timeout / disconnected sensor
            return Err(SensorError::AcquisitionTimeoutError());
        }

        Timer::after(Duration::from_millis(10)).await;
    }

    // 6. issue reset pulse
    ow.reset().await?;

    // 7. issue 'skip rom' command (CC) (considering there is a single sensor on the bus)
    ow.send_byte(0xCC).await?;

    // 8. issue  'read scratch pad' command (BE)
    //      9 bytes to be read (temperature written on byte 0 and byte 1)
    //      The data transfer starts with the least significant bit of byte 0 and continues through the scratchpad until the 9th byte (byte 8 – CRC) is read
    ow.send_byte(0xBE).await?;
    let mut scratchpad = [0u8; 9];
    for byte in &mut scratchpad {
        *byte = ow.exchange_byte(0xFF).await?
    }
    let temperature = I12F4::from_le_bytes([scratchpad[0], scratchpad[1]]);
    //TODO check crc

    // 9. issue reset pulse
    ow.reset().await?;
    Ok(temperature)
}
