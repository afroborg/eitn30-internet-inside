use std::io::Read;
use tun::platform::posix::Reader as PosixReader;

pub struct TunReader {
    reader: PosixReader,
}

impl TunReader {
    pub fn new(reader: PosixReader) -> Self {
        Self { reader }
    }

    pub fn read(&mut self) -> Vec<u8> {
        let mut buf = [0; 4096]; // TODO: Check what size the buffer should have

        match self.reader.read(&mut buf) {
            Ok(size) => {
                if size == 0 {
                    return vec![];
                }

                return buf[..size].to_vec();
            }
            Err(e) => {
                eprintln!("Error reading from tun device: {}", e);
                return vec![];
            }
        }
    }
}
