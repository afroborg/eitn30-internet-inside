use clap::Parser;
use std::thread;
use std::thread::sleep;
use std::time::Duration;
use transceive::{Receiver, Transmitter};

mod cli;
mod transceive;

fn main() {
    println!("Hello, world!");

    let args = cli::Args::parse();

    let mut address = *b"chan0";
    address[4] = args.address;

    let channel = args.channel;

    let mut tx = Transmitter::new(7, 0, channel, address);
    let mut rx = Receiver::new(17, 1, channel, address);

    let tx_thread = thread::spawn(move || loop {
        match tx.transmit(b"Hello there") {
            Ok(retries) => println!("Transmitted in {} retries", retries),
            Err(e) => println!("Error: {}", e),
        };

        sleep(Duration::from_millis(5000));
    });

    let rx_thread = thread::spawn(move || loop {
        sleep(Duration::from_millis(500));

        if let Some(data) = rx.receive() {
            println!("Received: {:?}", String::from_utf8(data).unwrap())
        };
    });

    tx_thread.join().unwrap();
    rx_thread.join().unwrap();

    println!("Goodbye, world!");
}
