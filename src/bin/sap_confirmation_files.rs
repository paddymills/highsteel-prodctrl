
#[macro_use] extern crate log;

use clap::Parser;
use simplelog::{Config, WriteLogger};
use std::fs::File;
use prodctrl::fs::{
    cnf::ProdFileProcessor,
    timestamped_file
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
    let args = Args::parse();

    WriteLogger::init(
        args.verbose.log_level_filter(),
        Config::default(),
        File::create(
            timestamped_file("log/cnf", "log")
        ).expect("failed to create log")
    ).expect("Failed to init logger");

    debug!("{:?}", args);

    let processor = ProdFileProcessor::new(args.dry_run);
    processor.process_files()?;
    
    Ok(())
}
