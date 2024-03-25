use nrf24l01::NRF24L01;

use super::transceiver::Transceiver;

pub struct Transmitter {
    device: NRF24L01,
}

impl Transmitter {
    pub fn new(ce_pin: u64, spi: u8, channel: u8, address: [u8; 5]) -> Self {
        let device = Transceiver::new(ce_pin, spi).set_transmitter(channel, address);

        Self { device }
    }

    pub fn transmit(&mut self, message: &[u8]) -> Result<u8, String> {
        self.device.push(0, message).unwrap();

        match self.device.send() {
            Ok(retries) => Ok(retries),
            Err(err) => {
                self.device.flush_output().unwrap();
                Err(format!("Destination unreachable: {:?}", err))
            }
        }
    }
}
