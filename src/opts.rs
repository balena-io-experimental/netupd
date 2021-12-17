use clap::Parser;

#[derive(Parser)]
pub struct Opts {
    #[clap(short, long, default_value = "localhost:3000")]
    pub address: String,
}
