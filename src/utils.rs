use crate::config::ADDRESS_WIDTH;

/// Change the last byte of an address
///
/// # Arguments
///
/// * `address` - The address to change
/// * `value` - The value to change the last byte to
///
/// # Returns
///
/// The new address with the last byte changed
pub const fn change_last_byte(address: &[u8; ADDRESS_WIDTH], value: u8) -> [u8; ADDRESS_WIDTH] {
    let mut new_address = *address;
    new_address[ADDRESS_WIDTH - 1] = value;
    new_address
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_change_last_byte() {
        let expected = *b"ad1";
        let address = b"ad0";
        let value = 49;

        assert_eq!(change_last_byte(&address, value), expected);
    }

    #[test]
    fn test_change_last_byte_2() {
        let expected = *b"ad2";
        let address = b"ad0";
        let value = 50;

        assert_eq!(change_last_byte(&address, value), expected);
    }
}
