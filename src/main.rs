use clap::Parser;
use interface::{Tun, TunReader, TunWriter};
use std::thread;
use std::thread::sleep;
use std::time::Duration;
use transceive::{Receiver, Transmitter};

mod cli;
mod interface;
mod transceive;

const TRANSMITTER_GPIO: u64 = 7;
const TRANSMITTER_SPI_CHANNEL: u8 = 0;
const RECEIVER_GPIO: u64 = 17;
const RECEIVER_SPI_CHANNEL: u8 = 1;

const DEFAULT_DELAY: u64 = 100;

fn main() {
    println!("Hello, world!");

    let args = cli::Args::parse();

    let address = *b"addr0";

    let receiver_address = change_last_byte(&address, args.receiver_address);
    let transmitter_address = change_last_byte(&address, args.transmitter_address);

    let (mut tun_reader, mut tun_writer) = Tun::new(args.transmitter_address);

    let mut tx = Transmitter::new(
        TRANSMITTER_GPIO,
        TRANSMITTER_SPI_CHANNEL,
        args.transmitter_channel,
        transmitter_address,
    );

    let mut rx = Receiver::new(
        RECEIVER_GPIO,
        RECEIVER_SPI_CHANNEL,
        args.receiver_channel,
        receiver_address,
    );

    let tx_thread = thread::spawn(move || tx_main(&mut tx, &mut tun_reader));
    let rx_thread = thread::spawn(move || rx_main(&mut rx, &mut tun_writer));

    tx_thread.join().unwrap();
    rx_thread.join().unwrap();

    println!("Goodbye, world!");
}

fn change_last_byte(address: &[u8; 5], value: u8) -> [u8; 5] {
    let mut new_address = address.clone();
    new_address[4] = value;
    new_address
}

fn tx_main(tx: &mut Transmitter, tun_reader: &mut TunReader) {
    loop {
        let data = tun_reader.read();

        if data.is_empty() {
            sleep(Duration::from_millis(DEFAULT_DELAY));
            continue;
        }

        data.chunks(32).for_each(|chunk| {
            match tx.transmit(&chunk) {
                Ok(retries) => println!("Transmitted in {} retries", retries),
                Err(e) => println!("Error: {}", e),
            };

            sleep(Duration::from_millis(DEFAULT_DELAY));
        });

        sleep(Duration::from_millis(DEFAULT_DELAY));
    }
}

fn rx_main(rx: &mut Receiver, tun_writer: &mut TunWriter) {
    loop {
        sleep(Duration::from_millis(DEFAULT_DELAY));

        if let Some(data) = rx.receive() {
            println!("Received: {:?}", data)
            // tun_writer.write(&data);
        };
    }
}
