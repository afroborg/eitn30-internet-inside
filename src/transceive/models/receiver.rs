use nrf24l01::NRF24L01;
// use std::future;

use super::transceiver::Transceiver;

pub struct Receiver {
    device: NRF24L01,
}

impl Receiver {
    pub fn new(ce_pin: u64, spi: u8, channel: u8, address: [u8; 5]) -> Self {
        let device = Transceiver::new(ce_pin, spi).set_receiver(channel, address);

        Self { device }
    }

    pub fn receive(&mut self) -> Option<Vec<u8>> {
        match self.device.data_available() {
            Ok(true) => {
                let mut vec = vec![];
                self.device
                    .read_all(|packet| {
                        vec.extend_from_slice(packet);
                    })
                    .unwrap();

                Some(vec)
            }
            _ => None,
        }
    }
}
