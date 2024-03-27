use std::process::Command;

pub fn apply(tun_interface_name: &str, address: u8) {
    let interface = tun_interface_name.to_string();

    set_route_default(true, address, interface.clone());

    ctrlc::set_handler(move || {
        set_route_default(false, address, interface.clone());

        std::process::exit(0);
    })
    .expect("Error setting Ctrl-C handler");
}

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
        .expect(&format!("Failed to {} default route", action));
}
