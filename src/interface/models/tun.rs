use super::reader::TunReader;
use super::writer::TunWriter;

pub fn new(interface_name: &str, address: u8) -> (TunReader, TunWriter) {
    let mut tun_config = tun::Configuration::default();

    tun_config
        .address((10, 0, 0, address))
        .netmask((255, 255, 255, 0))
        .name(interface_name)
        .mtu(65535)
        .up();

    let device = tun::create(&tun_config).unwrap();
    let (reader, writer) = device.split();

    (TunReader::new(reader), TunWriter::new(writer))
}
