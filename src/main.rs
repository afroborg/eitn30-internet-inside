use clap::Parser;
use interface::{tun, TunReader, TunWriter};
use std::thread;
use std::thread::sleep;
use std::time::Duration;
use transceive::{Receiver, Transmitter};

mod cli;
mod interface;
mod transceive;
mod utils;

const TRANSMITTER_GPIO: u64 = 7;
const TRANSMITTER_SPI_CHANNEL: u8 = 0;
const RECEIVER_GPIO: u64 = 17;
const RECEIVER_SPI_CHANNEL: u8 = 1;
const TUN_INTERFACE_NAME: &str = "longge";
const ADDRESS_WIDTH: usize = 3;

fn main() {
    let args = cli::Args::parse();

    let address = *b"ad0";

    let receiver_address = utils::change_last_byte(&address, args.receiver_address);
    let transmitter_address = utils::change_last_byte(&address, args.transmitter_address);

    let (mut tun_reader, mut tun_writer) = tun::new(TUN_INTERFACE_NAME, args.transmitter_address);

    // Handle base station mode
    if let Some(forward) = args.forward {
        println!("Running in base station mode");
        interface::forward::apply(TUN_INTERFACE_NAME, &forward);
    } else {
        println!("Running in mobile mode");
        interface::routing::apply(TUN_INTERFACE_NAME, args.transmitter_address);
    }

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

    let tx_thread = thread::spawn(move || tx_main(&mut tx, &mut tun_reader, args.delay));
    let rx_thread = thread::spawn(move || rx_main(&mut rx, &mut tun_writer, args.delay));

    tx_thread.join().unwrap();
    rx_thread.join().unwrap();
}

fn tx_main(tx: &mut Transmitter, tun_reader: &mut TunReader, delay: u64) {
    println!("Transmitter thread started");

    loop {
        let data = tun_reader.read();

        if data.is_empty() {
            sleep(Duration::from_micros(delay));
            continue;
        }

        data.chunks(32).for_each(|chunk| {
            if let Err(e) = tx.push(&chunk).and(tx.transmit(10)) {
                println!("Error: {}", e);
            };

            sleep(Duration::from_micros(delay));
        });
    }
}

fn rx_main(rx: &mut Receiver, tun_writer: &mut TunWriter, delay: u64) {
    println!("Receiver thread started");

    let mut buf = [0u8; 4096];
    let mut end = 0;

    loop {
        // TODO: look into this
        if end + 96 >= 4096 {
            end = 0;
        }

        if let Ok(new_end) = rx.receive(&mut buf, end) {
            end = new_end;

            if end <= 6 || !interface::packet::is_valid(&buf[..end]) {
                continue;
            }

            tun_writer.write(&buf[..end].to_vec());
            end = 0;
        };

        sleep(Duration::from_micros(delay / 2));
    }
}
