
#[macro_use] extern crate log;

use clap::Parser;
use prodctrl::{
    logging,
    fs::cnf::ProdFileProcessor
};

#[derive(Debug, Parser)]
#[clap(author, version, about = "Confirmation files for SAP processing")]
struct Args {
    #[clap(flatten)]
    verbose: clap_verbosity_flag::Verbosity,

    #[clap(short, long, help="Run without producing output or moving files")]
    dry_run: bool
}

#[tokio::main]
async fn main() -> Result<(), prodctrl::Error> {
    logging::init_logger("Sap Confirmation Files");

    let args = Args::parse();
    debug!("{:?}", args);

    let processor = ProdFileProcessor::new(args.dry_run);
    processor.process_files()?;
    
    Ok(())
}
