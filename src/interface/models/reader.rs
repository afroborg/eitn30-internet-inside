use crate::config::BUFFER_SIZE;
use std::io::Read;
use tun2::platform::posix::Reader as PosixReader;

pub struct TunReader {
    reader: PosixReader,
    // TODO: 1596 instead of 4096. And check why we even have +96 instead of some other number
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
            Ok(size) => {
                if size == 0 {
                    return &[];
                }

                &self.buf[..size]
            }
            Err(e) => {
                println!("Error reading from tun device: {e}");
                &[]
            }
        }
    }
}
