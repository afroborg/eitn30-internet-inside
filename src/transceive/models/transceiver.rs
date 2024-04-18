use crate::ADDRESS_WIDTH;
use nrf24l01::{DataRate, OperatingMode, PALevel, RXConfig, TXConfig, NRF24L01};

const DATA_RATE: DataRate = DataRate::R2Mbps;
const PA_LEVEL: PALevel = PALevel::Min;

pub struct Transceiver {
    ce_pin: u64,
    spi: u8,
}

impl Transceiver {
    pub fn new(ce_pin: u64, spi: u8) -> Self {
        Self { ce_pin, spi }
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

        let mut device = NRF24L01::new(self.ce_pin, self.spi, 0).unwrap();

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
            retry_delay: 2,
            max_retries: 15,
            ..Default::default()
        };

        let mut device = NRF24L01::new(self.ce_pin, self.spi, 0).unwrap();

        device.configure(&OperatingMode::TX(config)).unwrap();
        device.flush_output().unwrap();

        device
    }
}
