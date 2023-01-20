use clap::Parser;

#[derive(Parser)]
#[clap(version, about, long_about = None)]
pub struct Options {
    pub config_directory: String,
    #[clap(short, long, default_value = "0.0.0.0:8080")]
    pub bind: String,
}

pub fn parse() -> Options {
    Options::parse()
}
