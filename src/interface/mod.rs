fn test() {
    let mut tun_config = tun::Configuration::default();

    tun_config
        .address((10, 0, 0, 1))
        .netmask((255, 255, 255, 0))
        .up();

    let mut dev = tun::create(&tun_config).unwrap();

    let mut buff = [0u8; 1504];

    loop {
        let nbytes = dev.read(&mut buff).unwrap();
        println!("{:?}", &buff[..nbytes]);
    }
}
