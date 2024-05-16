use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Address of the receiver channel, should be a number between 0 and 9
    #[arg(long, default_value_t = 0, value_parser=clap::value_parser!(u8).range(0..255))]
    pub receiver_address: u8,

    /// Address of the transmitter channel, should be a number between 0 and 9
    #[arg(long, default_value_t = 0, value_parser=clap::value_parser!(u8).range(0..255))]
    pub transmitter_address: u8,

    /// Frequency of the receiver channel, should be a number between 0 and 125
    #[arg(long, default_value_t = 108, value_parser=clap::value_parser!(u8).range(0..125))]
    pub receiver_channel: u8,

    /// Frequency of the receiver channel, should be a number between 0 and 125
    #[arg(long, default_value_t = 108, value_parser=clap::value_parser!(u8).range(0..125))]
    pub transmitter_channel: u8,

    /// Base station interface forwarding
    #[arg(long, default_value = None)]
    pub forward: Option<Vec<String>>,

    /// GPIO pin for the transmitter
    #[arg(long, default_value_t = 7, value_parser=clap::value_parser!(u64))]
    pub transmitter_gpio: u64,

    /// GPIO pin for the receiver
    #[arg(long, default_value_t = 17, value_parser=clap::value_parser!(u64))]
    pub receiver_gpio: u64,

    /// Sets longge as the default interface
    #[arg(long, default_value_t = false, action = clap::ArgAction::Set)]
    pub longge_default: bool,

    /// Sets the auto-acknowledgement
    #[arg(long, default_value_t = true, action = clap::ArgAction::Set)]
    pub auto_ack: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mobile() {
        let args = Args::parse_from(&[
            "test",
            "--receiver-address",
            "0",
            "--transmitter-address",
            "1",
            "--receiver-channel",
            "116",
            "--transmitter-channel",
            "108",
            "--transmitter-gpio",
            "7",
            "--receiver-gpio",
            "17",
            "--longge-default",
            "true",
        ]);

        assert_eq!(args.receiver_address, 0);
        assert_eq!(args.transmitter_address, 1);
        assert_eq!(args.receiver_channel, 116);
        assert_eq!(args.transmitter_channel, 108);
        assert_eq!(args.forward, None);
        assert_eq!(args.transmitter_gpio, 7);
        assert_eq!(args.receiver_gpio, 17);
        assert_eq!(args.longge_default, true);
        assert_eq!(args.auto_ack, true);
    }

    #[test]
    fn test_base() {
        let args = Args::parse_from(&[
            "test",
            "--receiver-address",
            "1",
            "--transmitter-address",
            "0",
            "--receiver-channel",
            "108",
            "--transmitter-channel",
            "116",
            "--transmitter-gpio",
            "17",
            "--receiver-gpio",
            "27",
            "--forward",
            "eth0",
            "--forward",
            "wlan0",
            "--auto-ack",
            "false",
        ]);

        assert_eq!(args.receiver_address, 1);
        assert_eq!(args.transmitter_address, 0);
        assert_eq!(args.receiver_channel, 108);
        assert_eq!(args.transmitter_channel, 116);
        assert_eq!(
            args.forward,
            Some(vec!["eth0".to_string(), "wlan0".to_string()])
        );
        assert_eq!(args.transmitter_gpio, 17);
        assert_eq!(args.receiver_gpio, 27);
        assert_eq!(args.longge_default, false);
        assert_eq!(args.auto_ack, false);
    }
}
