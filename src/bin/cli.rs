
use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about)]
struct Args {
    #[clap(short, long, value_parser)]
    job: String,

    #[clap(short, long, value_parser, default_value_t=1)]
    shipment: u8,
}

fn main() {
    let args = Args::parse();

    println!("{:?}", args);
}
