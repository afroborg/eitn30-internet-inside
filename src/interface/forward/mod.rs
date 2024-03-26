mod iptable;

use std::process::Command;

use iptable::IpTableEntry;

pub fn apply(tun_interface_name: &str, forwards: &[String]) {
    println!("Applying IP forwarding rules");

    set_ip_forward(true);

    let rules = forwards
        .iter()
        .flat_map(|forward| {
            vec![
                IpTableEntry::new("filter", "FORWARD")
                    .in_iterface(tun_interface_name)
                    .out_interface(&forward)
                    .jump("ACCEPT")
                    .apply(),
                IpTableEntry::new("filter", "FORWARD")
                    .in_iterface(&forward)
                    .out_interface(tun_interface_name)
                    .matching("state")
                    .state("RELATED,ESTABLISHED")
                    .jump("ACCEPT")
                    .apply(),
                IpTableEntry::new("nat", "POSTROUTING")
                    .out_interface(&forward)
                    .jump("MASQUERADE")
                    .apply(),
            ]
        })
        .collect::<Vec<_>>();

    ctrlc::set_handler(move || {
        set_ip_forward(false);

        rules.iter().for_each(|rule| {
            rule.drop();
        });

        std::process::exit(0);
    })
    .expect("Error setting Ctrl-C handler");
}

fn set_ip_forward(enable: bool) {
    Command::new("sh")
        .arg("-c")
        .arg(format!(
            "echo {} > /proc/sys/net/ipv4/ip_forward",
            if enable { "1" } else { "0" }
        ))
        .output()
        .expect(&format!(
            "Failed to {} IP forwarding",
            if enable { "enable" } else { "disable" }
        ));
}
