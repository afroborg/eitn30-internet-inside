use nrf24l01::{DataRate, OperatingMode, PALevel, RXConfig, TXConfig, NRF24L01};

// TODO? Abstract out these constants
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

    pub fn set_receiver(&mut self, channel: u8, address: [u8; 5]) -> NRF24L01 {
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

    pub fn set_transmitter(&mut self, channel: u8, address: [u8; 5]) -> NRF24L01 {
        let config = TXConfig {
            data_rate: DATA_RATE,
            channel,
            pa_level: PA_LEVEL,
            pipe0_address: address,
            retry_delay: 10,
            max_retries: 15,
            ..Default::default()
        };

        let mut device = NRF24L01::new(self.ce_pin, self.spi, 0).unwrap();

        device.configure(&OperatingMode::TX(config)).unwrap();
        device.flush_output().unwrap();

        device
    }
}
