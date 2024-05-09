use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Address of the receiver channel, should be a number between 0 and 9
    #[arg(short, long, default_value_t = 0, value_parser=clap::value_parser!(u8).range(0..255))]
    pub receiver_address: u8,

    /// Address of the transmitter channel, should be a number between 0 and 9
    #[arg(short, long, default_value_t = 0, value_parser=clap::value_parser!(u8).range(0..255))]
    pub transmitter_address: u8,

    /// Frequency of the receiver channel, should be a number between 0 and 125
    #[arg(short, long, default_value_t = 108, value_parser=clap::value_parser!(u8).range(0..125))]
    pub receiver_channel: u8,

    /// Frequency of the receiver channel, should be a number between 0 and 125
    #[arg(short, long, default_value_t = 108, value_parser=clap::value_parser!(u8).range(0..125))]
    pub transmitter_channel: u8,

    /// Base station interface forwarding
    #[arg(short, long, default_value = None)]
    pub forward: Option<Vec<String>>,

    /// Delay between each transmission
    #[arg(short, long, default_value_t = 20, value_parser=clap::value_parser!(u64))]
    pub delay: u64,

    /// GPIO pin for the transmitter
    #[arg(short, long, default_value_t = 7, value_parser=clap::value_parser!(u64))]
    pub transmitter_gpio: u64,

    /// GPIO pin for the receiver
    #[arg(short, long, default_value_t = 17, value_parser=clap::value_parser!(u64))]
    pub receiver_gpio: u64,

    /// Sets longge as the default interface
    #[arg(short, long, default_value_t = false, action = clap::ArgAction::Set)]
    pub longge_default: bool,
}
