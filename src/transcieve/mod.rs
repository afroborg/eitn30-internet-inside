use std::thread::sleep;
use std::time::Duration;

use models::Transciever;
use nrf24l01::{DataRate, OperatingMode, PALevel, RXConfig, TXConfig, NRF24L01};

mod models;

const TX: Transciever = Transciever {
    ce_pin: 17,
    spi: 0,
    channel: 108,
};
const RX: Transciever = Transciever {
    ce_pin: 27,
    spi: 1,
    channel: 108,
};

pub fn transmit() {
    println!("Transmitting");
    let config = TXConfig {
        // data_rate: DataRate::R2Mbps,
        channel: TX.channel,
        pa_level: PALevel::Low,
        pipe0_address: *b"abc",
        max_retries: 3,
        retry_delay: 2,
        ..Default::default()
    };

    let mut device = NRF24L01::new(TX.ce_pin, TX.spi, 0).unwrap();
    let message = b"sendtest";
    device.configure(&OperatingMode::TX(config)).unwrap();
    device.flush_output().unwrap();

    loop {
        println!("Loop!!");
        device.push(0, message).unwrap();
        match device.send() {
            Ok(retries) => println!("Message sent, {} retries needed", retries),

            // TODO: What does Err mean here? It is that the transmitter is not connected to the RP?
            Err(err) => {
                println!("Destination unreachable: {:?}", err);
                device.flush_output().unwrap()
            }
        };
        sleep(Duration::from_millis(5000));
    }
}

pub fn receive() {
    // TODO: Check data rate
    let config = RXConfig {
        data_rate: DataRate::R2Mbps,
        channel: RX.channel,
        pa_level: PALevel::Low,
        pipe0_address: *b"abc",
        ..Default::default()
    };

    let mut device = NRF24L01::new(RX.ce_pin, RX.spi, 0).unwrap();
    device.configure(&OperatingMode::RX(config)).unwrap();

    device.listen().unwrap();

    loop {
        sleep(Duration::from_millis(500));
        if device.data_available().unwrap() {
            device
                .read_all(|packet| {
                    println!("Received {:?} bytes", packet.len());
                    println!("Payload {:?}", packet);
                })
                .unwrap();
        }
    }
}