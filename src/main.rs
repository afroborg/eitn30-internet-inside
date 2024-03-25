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
                // Ok(retries) => println!("Transmitted in {} retries", retries),
                Ok(_) => (),
                Err(e) => println!("Error: {}", e),
            };

            sleep(Duration::from_millis(DEFAULT_DELAY));
        });

        sleep(Duration::from_millis(DEFAULT_DELAY));
    }
}

fn rx_main(rx: &mut Receiver, tun_writer: &mut TunWriter) {
    let mut buf = [0u8; 4096];
    let mut end = 0;

    loop {
        sleep(Duration::from_millis(DEFAULT_DELAY));

        if let Some(data) = rx.receive() {
            let start = end;
            end += data.len();
            buf[start..end].copy_from_slice(&data);

            match packet::ip::Packet::new(&buf[..end]) {
                Ok(packet) => {
                    let packet_length = match &packet {
                        packet::ip::Packet::V4(packet) => packet.length() as usize,
                        packet::ip::Packet::V6(_) => {
                            40 + u16::from_be_bytes([buf[4], buf[5]]) as usize
                        }
                    };

                    if end >= packet_length {
                        println!("Packet: {:?} with length {}", &packet, packet_length);
                        tun_writer.write(&buf[..end].to_vec());
                        buf = [0u8; 4096];
                        end = 0;
                    }
                }
                _ => (),
            }
        };
    }
}
