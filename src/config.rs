use nrf24l01::{DataRate, PALevel};

// -- TUN Config -- //
pub const TUN_INTERFACE_NAME: &str = "longge";

// -- Transmitter config -- //
pub const TRANSMITTER_SPI_CHANNEL: u8 = 0;
pub const QUEUE_SIZE: usize = 3;

// -- Receiver config -- //
pub const RECEIVER_SPI_CHANNEL: u8 = 1;
pub const BUFFER_SIZE: usize = 4096;

// -- NRF24L01 packet config -- //
pub const DATA_RATE: DataRate = DataRate::R2Mbps;
pub const PA_LEVEL: PALevel = PALevel::Max;
pub const ADDRESS_WIDTH: usize = 3;
pub const PACKET_RETRY_DELAY: u8 = 2;
pub const PACKET_MAX_RETRIES: u8 = 15;
pub const PACKET_SIZE: usize = 32;
