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

            // Print the packet and its length if it is valid
            // println!("Packet {:?} length: {}", &packet, packet_length);
            true
        }
        Err(_) => false,
    }
}
