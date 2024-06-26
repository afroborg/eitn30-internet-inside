use crate::config::{ADDRESS_WIDTH, DATA_RATE, PACKET_MAX_RETRIES, PACKET_RETRY_DELAY, PA_LEVEL};
use nrf24l01::{OperatingMode, RXConfig, TXConfig, NRF24L01};

pub struct Transceiver {
    ce_pin: u64,
    spi: u8,
    auto_ack: bool,
}

impl Transceiver {
    pub fn new(ce_pin: u64, spi: u8, auto_ack: bool) -> Self {
        Self {
            ce_pin,
            spi,
            auto_ack,
        }
    }

    /// Set the device as a receiver
    ///
    /// # Arguments
    ///
    /// * `channel` - The channel for the device
    /// * `address` - The address for the device
    ///
    /// # Returns
    ///
    /// The device set as a receiver
    pub fn set_receiver(&mut self, channel: u8, address: [u8; ADDRESS_WIDTH]) -> NRF24L01 {
        let config = RXConfig {
            data_rate: DATA_RATE,
            channel,
            pa_level: PA_LEVEL,
            pipe0_address: address,
            ..Default::default()
        };

        let mut device = NRF24L01::new(self.ce_pin, self.spi, 0, self.auto_ack).unwrap();

        device.configure(&OperatingMode::RX(config)).unwrap();
        device.flush_input().unwrap();
        device.listen().unwrap();

        device
    }

    /// Set the device as a transmitter
    ///
    /// # Arguments
    ///
    /// * `channel` - The channel for the device
    /// * `address` - The address for the device
    ///
    /// # Returns
    ///
    /// The device set as a transmitter
    pub fn set_transmitter(&mut self, channel: u8, address: [u8; ADDRESS_WIDTH]) -> NRF24L01 {
        let config = TXConfig {
            data_rate: DATA_RATE,
            channel,
            pa_level: PA_LEVEL,
            pipe0_address: address,
            retry_delay: PACKET_RETRY_DELAY,
            max_retries: PACKET_MAX_RETRIES,
        };

        let mut device = NRF24L01::new(self.ce_pin, self.spi, 0, self.auto_ack).unwrap();

        device.configure(&OperatingMode::TX(config)).unwrap();
        device.flush_output().unwrap();

        device
    }
}
