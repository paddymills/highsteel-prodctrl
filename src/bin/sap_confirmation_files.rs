
use clap::Parser;
// use prodctrl::fs::cnf::ProdFileProcessor;

#[derive(Debug, Parser)]
#[clap(author, version, about = "Confirmation files for SAP processing")]
struct Args {
    #[clap(short, long, help="Run without producing output or moving files")]
    dry_run: bool
}

#[tokio::main]
async fn main() -> Result<(), prodctrl::Error> {
    let args = Args::parse();
    println!("{:?}", args);

    // let processor = ProdFileProcessor::new();
    // processor.process_files()?;
    
    Ok(())
}
