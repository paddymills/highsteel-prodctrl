
use clap::Parser;
// use prodctrl::fs::cnf::ProdFileProcessor;

#[derive(Debug, Parser)]
#[clap(author, version, about)]
struct Args {
    #[clap(short, long, help="Run without producing output or moving files")]
    dry_run: bool
}

fn main() {
    let args = Args::parse();
    println!("{:?}", args);
}
