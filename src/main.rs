use clap::Parser;
use config::*;
use interface::{tun, TunReader, TunWriter};
use std::sync::mpsc::{channel, Receiver as ChannelReceiver, Sender as ChannelSender};
use std::thread;
use transceive::{Receiver, Transmitter};

mod cli;
mod config;
mod interface;
mod transceive;
mod utils;

fn main() {
    let args = cli::Args::parse();

    let address = *b"ad0"; // In hexadecimal: 61 64 30

    let receiver_address = utils::change_last_byte(&address, args.receiver_address);
    let transmitter_address = utils::change_last_byte(&address, args.transmitter_address);

    let (tun_reader, tun_writer) = tun::new(TUN_INTERFACE_NAME, args.transmitter_address);

    // Handle base station mode
    if let Some(forward) = args.forward {
        println!("Running in base station mode");
        interface::forward::apply(TUN_INTERFACE_NAME, &forward);
    } else {
        println!("Running in mobile mode");

        if args.longge_default {
            println!("Setting {} to default interface", TUN_INTERFACE_NAME);
            interface::routing::apply(TUN_INTERFACE_NAME, args.transmitter_address);
        }
    }

    let mut tx = Transmitter::new(
        args.transmitter_gpio,
        TRANSMITTER_SPI_CHANNEL,
        args.transmitter_channel,
        transmitter_address,
    );

    let mut rx = Receiver::new(
        args.receiver_gpio,
        RECEIVER_SPI_CHANNEL,
        args.receiver_channel,
        receiver_address,
    );

    let tx_thread = thread::spawn(move || tx_main(&mut tx, tun_reader));
    let rx_thread = thread::spawn(move || rx_main(&mut rx, tun_writer));

    tx_thread.join().expect("Transmitter thread panicked");
    rx_thread.join().expect("Receiver thread panicked");
}

fn tx_main(tx: &mut Transmitter, mut tun_reader: TunReader) -> ! {
    println!("Transmitter thread started");

    let (reader_queue, tx_queue) = channel::<Vec<u8>>();

    thread::spawn(move || reader_main(&reader_queue, &mut tun_reader));

    loop {
        let data = tx_queue.recv().unwrap();

        data.chunks(PACKET_SIZE * QUEUE_SIZE).for_each(|queue| {
            queue.chunks(PACKET_SIZE).for_each(|pkt| {
                tx.push(pkt).unwrap();
            });

            if let Err(err) = tx.transmit(10) {
                println!("Error: {err}");
            };
        });
    }
}

fn reader_main(queue: &ChannelSender<Vec<u8>>, tun_reader: &mut TunReader) -> ! {
    println!("Reader thread started");

    loop {
        let data = tun_reader.read();

        if data.is_empty() {
            continue;
        }

        queue.send(data.to_vec()).unwrap();
    }
}

fn rx_main(rx: &mut Receiver, mut tun_writer: TunWriter) -> ! {
    println!("Receiver thread started");

    let mut buf = [0u8; BUFFER_SIZE];
    let mut end = 0;

    let (rx_queue, writer_queue) = channel::<Vec<u8>>();

    thread::spawn(move || writer_main(&writer_queue, &mut tun_writer));

    loop {
        if (end + PACKET_SIZE * QUEUE_SIZE) >= BUFFER_SIZE {
            end = 0;
        }

        if let Ok(new_end) = rx.receive(&mut buf, end) {
            end = new_end;

            let data = &buf[..end];

            if end <= 6 || !interface::packet::is_valid(data) {
                continue;
            }

            rx_queue.send(data.to_vec()).unwrap();

            end = 0;
        };
    }
}

fn writer_main(queue: &ChannelReceiver<Vec<u8>>, tun_writer: &mut TunWriter) -> ! {
    println!("Writer thread started");

    loop {
        let data = queue.recv().unwrap();
        tun_writer.write(&data);
    }
}

#[cfg(test)]
mod test {}
