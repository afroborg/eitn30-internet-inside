use super::reader::TunReader;
use super::writer::TunWriter;

pub fn new(interface_name: &str, address: u8) -> (TunReader, TunWriter) {
    let mut tun_config = tun::Configuration::default();

    tun_config
        .address((10, 0, 0, address))
        .netmask((255, 255, 255, 0))
        // .mtu(32) // The payload in our interface is 32 bytes
        .name(interface_name)
        .up();

    let device = tun::create(&tun_config).unwrap();
    let (reader, writer) = device.split();

    (TunReader::new(reader), TunWriter::new(writer))
}
