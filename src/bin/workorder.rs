
#[macro_use] extern crate log;

use clap::{Parser, Subcommand};
use simplelog::{Config, SimpleLogger};

use prodctrl::prelude::*;
use prodctrl::JobShipment;

// TODO: completions https://docs.rs/clap_complete/latest/clap_complete/index.html

/// Work order management system
#[derive(Debug, Parser)]
#[clap(name = "Workorder")]
#[clap(author, version)]
struct Cli {
    /// Subcommand to run
    #[clap(subcommand)]
    command: Option<Commands>,
    
    /// Job number (with structure letter) and shipment
    jobship: JobShipment,

    /// verbosity level
    #[clap(flatten)]
    verbose: clap_verbosity_flag::Verbosity,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// update work order
    Update,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Cli::parse();

    SimpleLogger::init(
        args.verbose.log_level_filter(),
        Config::default()
    ).expect("Failed to init logger");

    debug!("{:?}", args);

    Ok(())
}
