use std::thread;
mod transcieve;

fn main() {
    println!("Hello, world!");

    let tx_thread = thread::spawn(|| {
        transcieve::transmit();
    });

    let rx_thread = thread::spawn(|| {
        transcieve::receive();
    });

    tx_thread.join().unwrap();
    rx_thread.join().unwrap();

    println!("Goodbye, world!");
}
