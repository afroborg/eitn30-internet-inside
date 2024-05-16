use crate::{config::ADDRESS_WIDTH, BUFFER_SIZE};
use nrf24l01::NRF24L01;

use super::transceiver::Transceiver;

pub struct Receiver {
    device: NRF24L01,
}

impl Receiver {
    pub fn new(
        ce_pin: u64,
        spi: u8,
        channel: u8,
        address: [u8; ADDRESS_WIDTH],
        auto_ack: bool,
    ) -> Self {
        let device = Transceiver::new(ce_pin, spi, auto_ack).set_receiver(channel, address);

        Self { device }
    }

    pub fn data_available(&self) -> bool {
        match self.device.data_available() {
            Ok(val) => val,
            Err(e) => {
                println!("Error checking for data: {e}");
                false
            }
        }
    }

    /// Receive data from the device
    ///
    /// # Arguments
    ///
    /// * `buf` - The buffer to receive the data into
    /// * `end` - The end of the buffer
    ///
    /// # Returns
    ///
    /// The number of bytes received
    /// or an error message if no data is available
    pub fn receive(&mut self, buf: &mut [u8; BUFFER_SIZE], end: usize) -> Result<usize, String> {
        let mut e = end;

        let mut overflow = false;

        self.device
            .read_all(|packet| {
                if overflow {
                    return;
                }

                let start = e;
                e += packet.len();

                if e > buf.len() {
                    overflow = true;
                    return;
                }

                buf[start..e].copy_from_slice(packet);
            })
            .unwrap();

        if overflow {
            return Err("Buffer overflow".to_string());
        }

        Ok(e)
    }
}
