
#[macro_use] extern crate log;

use clap::Parser;
use simplelog::{Config, WriteLogger};
use std::fs::File;
use prodctrl::fs::{
    cnf::ProdFileProcessor,
    timestamped_file, is_empty_file
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

    let log_file = timestamped_file("log/cnf", "log");

    WriteLogger::init(
        args.verbose.log_level_filter(),
        Config::default(),
        File::create(log_file).expect("failed to create log")
    ).expect("Failed to init logger");

    debug!("{:?}", args);

    let processor = ProdFileProcessor::new(args.dry_run);
    processor.process_files()?;

    // remove log file if nothing logged
    if is_empty_file(log_file) {
        let _ = fs::remove_file(log_file);
    }
    
    Ok(())
}
