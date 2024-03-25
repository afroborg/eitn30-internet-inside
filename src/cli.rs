use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Address of the device, should be a number between 0 and 9
    #[arg(short, long, default_value_t = 0)]
    pub address: u8,

    /// Channel of the device, should be a number between 0 and 255
    #[arg(short, long, default_value_t = 108)]
    pub channel: u8,
}
