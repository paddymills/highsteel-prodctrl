
#[macro_use] extern crate log;

use clap::{Parser, Subcommand};
use simplelog::{Config, SimpleLogger};

use prodctrl::prelude::*;
use prodctrl::JobShipment;

/// Work order management system
#[derive(Debug, Parser)]
#[clap(name = "Workorder")]
#[clap(author, version)]
struct Cli {
    /// Subcommand to run
    #[clap(subcommand)]
    command: Option<Commands>,

    /// verbosity level
    #[clap(flatten)]
    verbose: clap_verbosity_flag::Verbosity,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// update work order
    Update {
        /// Job number (with structure letter) and shipment
        jobship: JobShipment,
    },

    /// check for jobs to update from SAP load files
    CheckUpdate,
}

impl Commands {
    fn handle_command(self) {
        match self {
            Self::Update { jobship } => println!("Updating {}...", jobship),
            Self::CheckUpdate => println!("Checking for updates..."),
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Cli::parse();

    SimpleLogger::init(
        args.verbose.log_level_filter(),
        Config::default()
    ).expect("Failed to init logger");

    debug!("{:?}", args);

    if let Some(cmd) = args.command {
        cmd.handle_command();
    }

    Ok(())
}
