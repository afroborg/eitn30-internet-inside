use std::io::Write;
use tun::platform::posix::Writer as PosixWriter;
use tun2 as tun;

pub struct TunWriter {
    writer: PosixWriter,
}

impl TunWriter {
    pub fn new(writer: PosixWriter) -> Self {
        Self { writer }
    }

    pub fn write(&mut self, data: &[u8]) {
        match self.writer.write(data) {
            Ok(size) => {
                if size == 0 {
                    eprintln!("Wrote 0 bytes to tun device");
                }
            }
            Err(e) => {
                eprintln!("Error writing to tun device: {e}");
            }
        }
    }
}
