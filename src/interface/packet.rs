use packet::ip::Packet;

/// Check if a buffer contains a valid IP packet
///
/// # Arguments
///
/// * `buf` - A buffer containing an IP packet
///
/// # Returns
///
/// A boolean indicating if the buffer contains a valid IP packet
pub fn is_valid(buf: &[u8]) -> bool {
    match Packet::new(buf) {
        Ok(packet) => {
            // Find what the length of the packet should be
            let packet_length = match packet {
                Packet::V4(packet) => packet.length() as usize,
                Packet::V6(_) => 40 + u16::from_be_bytes([buf[4], buf[5]]) as usize,
            };

            // Return if the buffer is too short
            if buf.len() < packet_length {
                return false;
            }

            true
        }
        Err(_) => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_valid_invalid() {
        let buf = vec![0; 60];

        assert_eq!(is_valid(&buf), false);
    }

    #[test]
    fn test_is_valid_valid() {
        let mut buf = vec![0; 60];
        buf[0] = 0x45;
        buf[1] = 0x00;
        buf[2] = 0x00;
        buf[3] = 0x3c;
        buf[4] = 0x00;
        buf[5] = 0x00;
        buf[6] = 0x00;
        buf[7] = 0x00;
        buf[8] = 0x40;
        buf[9] = 0x01;
        buf[10] = 0x00;
        buf[11] = 0x00;
        buf[12] = 0x7f;
        buf[13] = 0x00;
        buf[14] = 0x00;
        buf[15] = 0x01;
        buf[16] = 0x7f;
        buf[17] = 0x00;
        buf[18] = 0x00;
        buf[19] = 0x01;

        assert_eq!(is_valid(&buf), true);
    }
}
