use std::io::Write;
use tun::platform::posix::Writer as PosixWriter;

pub struct TunWriter {
    writer: PosixWriter,
}

impl TunWriter {
    pub fn new(writer: PosixWriter) -> Self {
        Self { writer }
    }

    pub fn write(&mut self, data: &[u8]) {
        match self.writer.write(data) {
            Ok(_size) => {
                // println!("Wrote {} bytes to tun device", size)
            }
            Err(e) => {
                eprintln!("Error writing to tun device: {e}");
            }
        }
    }
}
