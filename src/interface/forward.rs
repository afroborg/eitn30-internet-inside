use std::{fs, process::Command};

use super::iptable::IpTableEntry;

/// Apply IP forwarding rules
///
/// # Arguments
///
/// * `tun_interface_name` - The name of the TUN interface
/// * `forwards` - A list of interface names to forward packets to
///
/// # Panics
///
/// This function will panic if it fails to apply the IP forwarding rules
/// or if it fails to remove the IP forwarding rules when the program is
/// interrupted
pub fn apply(tun_interface_name: &str, forwards: &[String]) {
    println!("Applying IP forwarding rules");

    set_ip_forward(true);

    let rules = forwards
        .iter()
        .flat_map(|forward| {
            vec![
                // Allow forwarding from tun interface to forward interface(s)
                // for packets that are NEW (i.e. requests)
                IpTableEntry::new("filter", "FORWARD")
                    .in_iterface(tun_interface_name)
                    .out_interface(&forward)
                    .jump("ACCEPT")
                    .apply(),
                // Allow forwarding from forward interface(s) to tun interface
                // for packets that are RELATED or ESTABLISHED (i.e. responses)
                IpTableEntry::new("filter", "FORWARD")
                    .in_iterface(&forward)
                    .out_interface(tun_interface_name)
                    .matching("state")
                    .state("RELATED,ESTABLISHED")
                    .jump("ACCEPT")
                    .apply(),
                // Masquerade packets from forward interface(s)
                // allows the packets to be routed back to the tun interface
                // and then to the original source
                IpTableEntry::new("nat", "POSTROUTING")
                    .out_interface(&forward)
                    .jump("MASQUERADE")
                    .apply(),
            ]
        })
        .collect::<Vec<_>>();

    // Remove IP forwarding rules when the program is interrupted
    ctrlc::set_handler(move || {
        set_ip_forward(false);

        rules.iter().for_each(|rule| {
            rule.drop();
        });

        std::process::exit(0);
    })
    .expect("Error setting Ctrl-C handler");
}

/// Enable or disable IP forwarding
///
/// # Arguments
///
/// * `enable` - A boolean indicating if IP forwarding should be enabled
///
/// # Panics
///
/// This function will panic if it fails to enable or disable IP forwarding
fn set_ip_forward(enable: bool) {
    println!(
        "{} IP forwarding",
        if enable { "Enabling" } else { "Disabling" }
    );

    fs::write(
        "/proc/sys/net/ipv4/ip_forward",
        if enable { "1" } else { "0" },
    )
    .expect(&format!(
        "Failed to {} IP forwarding",
        if enable { "enable" } else { "disable" }
    ));
}
