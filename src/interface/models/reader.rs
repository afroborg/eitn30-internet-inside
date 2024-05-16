use crate::config::BUFFER_SIZE;
use std::io::Read;
use tun::platform::posix::Reader as PosixReader;
use tun2 as tun;

pub struct TunReader {
    reader: PosixReader,
    buf: [u8; BUFFER_SIZE],
}

impl TunReader {
    pub fn new(reader: PosixReader) -> Self {
        Self {
            reader,
            buf: [0u8; BUFFER_SIZE],
        }
    }

    pub fn read(&mut self) -> &[u8] {
        match self.reader.read(&mut self.buf) {
            Ok(size) => &self.buf[..size],
            Err(e) => {
                println!("Error reading from tun device: {e}");
                &[]
            }
        }
    }
}
