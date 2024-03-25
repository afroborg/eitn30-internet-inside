use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Address of the receiver channel, should be a number between 0 and 9
    #[arg(short, long, default_value_t = 0, value_parser=clap::value_parser!(u8).range(0..9))]
    pub receiver_address: u8,

    /// Address of the transmitter channel, should be a number between 0 and 9
    #[arg(short, long, default_value_t = 0, value_parser=clap::value_parser!(u8).range(0..9))]
    pub transmitter_address: u8,

    /// Frequency of the receiver channel, should be a number between 0 and 125
    #[arg(short, long, default_value_t = 108, value_parser=clap::value_parser!(u8).range(0..125))]
    pub receiver_channel: u8,

    /// Frequency of the receiver channel, should be a number between 0 and 125
    #[arg(short, long, default_value_t = 108, value_parser=clap::value_parser!(u8).range(0..125))]
    pub transmitter_channel: u8,
}
