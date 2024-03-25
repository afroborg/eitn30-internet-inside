use super::reader::TunReader;
use super::writer::TunWriter;

pub struct Tun {}

impl Tun {
    pub fn new(address: u8) -> (TunReader, TunWriter) {
        let mut tun_config = tun::Configuration::default();

        tun_config
            .address((10, 0, 0, address))
            // .destination((10, 0, 0, 2)) // TODO: Check if we need destination address as well
            .netmask((255, 255, 255, 0))
            // .mtu(32) // The payload in our interface is 32 bytes
            .up();

        let device = tun::create(&tun_config).unwrap();
        let (reader, writer) = device.split();

        (TunReader::new(reader), TunWriter::new(writer))
    }
}
