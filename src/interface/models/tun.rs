use super::reader::TunReader;
use super::writer::TunWriter;

pub fn new(interface_name: &str, address: u8) -> (TunReader, TunWriter) {
    let mut tun_config = tun2::Configuration::default();

    tun_config
        .address((10, 0, 0, address))
        .netmask((255, 255, 255, 0))
        .tun_name(interface_name)
        .mtu(1500)
        .up();

    let device = tun2::create(&tun_config).unwrap();

    let (reader, writer) = device.split();

    (TunReader::new(reader), TunWriter::new(writer))
}
