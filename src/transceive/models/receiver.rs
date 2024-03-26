use crate::ADDRESS_WIDTH;
use nrf24l01::NRF24L01;
// use std::future;

use super::transceiver::Transceiver;

pub struct Receiver {
    device: NRF24L01,
}

impl Receiver {
    pub fn new(ce_pin: u64, spi: u8, channel: u8, address: [u8; ADDRESS_WIDTH]) -> Self {
        let device = Transceiver::new(ce_pin, spi).set_receiver(channel, address);

        Self { device }
    }

    pub fn receive(&mut self, buf: &mut [u8; 4096], end: usize) -> Result<usize, String> {
        match self.device.data_available() {
            Ok(true) => {
                let mut e = end;

                self.device
                    .read_all(|packet| {
                        let start = e;
                        e += packet.len();
                        buf[start..e].copy_from_slice(&packet);
                    })
                    .unwrap();

                Ok(e)
            }
            _ => Err("No data available".to_string()),
        }
    }
}
