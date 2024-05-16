use crate::config::ADDRESS_WIDTH;
use nrf24l01::NRF24L01;

use super::transceiver::Transceiver;

pub struct Transmitter {
    device: NRF24L01,
}

impl Transmitter {
    /// Create a new Transmitter
    ///
    /// # Arguments
    ///
    /// * `ce_pin` - The CE pin for the device
    /// * `spi` - The SPI bus for the device
    /// * `channel` - The channel for the device
    pub fn new(
        ce_pin: u64,
        spi: u8,
        channel: u8,
        address: [u8; ADDRESS_WIDTH],
        auto_ack: bool,
    ) -> Self {
        let device = Transceiver::new(ce_pin, spi, auto_ack).set_transmitter(channel, address);

        Self { device }
    }

    /// Push a message to the device
    ///
    /// # Arguments
    ///
    /// * `message` - The message to push to the device
    ///
    /// # Returns
    ///
    /// A Result indicating if the message was successfully pushed to the device
    /// or an error message if the message could not be pushed
    pub fn push(&mut self, message: &[u8]) -> Result<(), String> {
        self.device
            .push(0, message)
            .map_err(|err| format!("{err:?}"))
    }

    /// Transmit a message to the device
    ///
    /// # Arguments
    ///  
    /// * `retries` - The number of times to retry transmitting the message
    ///
    /// # Returns
    ///
    /// A Result indicating if the message was successfully transmitted to the device
    /// or an error message if the message could not be transmitted
    pub fn transmit(&mut self, retries: u8) -> Result<u8, String> {
        match self.device.send() {
            Ok(retries) => Ok(retries),
            Err(err) => {
                if retries > 0 {
                    println!("Failed to transmit, {retries} retries remaining");
                    return self.transmit(retries - 1);
                }

                self.device
                    .flush_output()
                    .map_err(|err| format!("Failed to flush output: {err:?}"))?;

                Err(format!("Destination unreachable: {err:?}"))
            }
        }
    }
}
