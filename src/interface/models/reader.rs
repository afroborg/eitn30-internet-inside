use std::io::Read;
use tun::platform::posix::Reader as PosixReader;

pub struct TunReader {
    reader: PosixReader,
    buf: [u8; 4096],
}

impl TunReader {
    pub fn new(reader: PosixReader) -> Self {
        Self {
            reader,
            buf: [0u8; 4096],
        }
    }

    pub fn read(&mut self) -> &[u8] {
        match self.reader.read(&mut self.buf) {
            Ok(size) => {
                if size == 0 {
                    return &[];
                }

                return &self.buf[..size];
            }
            Err(e) => {
                eprintln!("Error reading from tun device: {}", e);
                return &[];
            }
        }
    }
}
