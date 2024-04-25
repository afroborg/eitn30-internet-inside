use std::process::Command;

/// Apply the default route for the tun interface
///
/// # Arguments
///
/// * `tun_interface_name` - The name of the tun interface
/// * `address` - The last octet of the IP address to route to
///
/// # Panics
///
/// This function will panic if it fails to add or remove the default route
/// or if it fails to remove the default route when the program is interrupted
pub fn apply(tun_interface_name: &str, address: u8) {
    let interface = tun_interface_name.to_string();

    set_route_default(true, address, interface.clone());

    // Remove the default route when the program is interrupted
    ctrlc::set_handler(move || {
        set_route_default(false, address, interface.clone());

        std::process::exit(0);
    })
    .expect("Error setting Ctrl-C handler");
}

/// Set the default route for the tun interface
///
/// # Arguments
///
/// * `enable` - A boolean indicating if the default route should be added or removed
/// * `address` - The last octet of the IP address to route to
/// * `tun_interface_name` - The name of the tun interface
///
/// # Panics
///
/// This function will panic if it fails to add or remove the default route
fn set_route_default(enable: bool, address: u8, tun_interface_name: String) {
    let action = if enable { "add" } else { "del" };
    let ip = format!("10.0.0.{}", address);

    Command::new("ip")
        .arg("route")
        .arg(action)
        .arg("default")
        .arg("via")
        .arg(ip)
        .arg("dev")
        .arg(tun_interface_name)
        .output()
        .unwrap_or_else(|_| panic!("Failed to {} default route", action));
}
