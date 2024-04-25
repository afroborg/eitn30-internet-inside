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
