
#[macro_use] extern crate log;

use clap::{Parser, Subcommand};
use simplelog::{Config, SimpleLogger};
use prodctrl::prelude::*;

// TODO: completions https://docs.rs/clap_complete/latest/clap_complete/index.html
#[derive(Debug, Parser)]
#[clap(author, version, about)]
#[clap(propagate_version = true)]
struct Cli {
    /// Subcommand to run
    #[clap(subcommand)]
    command: Commands,

    /// verbosity level
    #[clap(flatten)]
    verbose: clap_verbosity_flag::Verbosity,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// interact with configs
    Cfg {
        /// generate config files
        #[clap(short, long)]
        generate: bool
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

    match args.command {
        Commands::Cfg { generate: true } => {
            prodctrl::config::DbConfig::generate()
        }
        _ => ()
    }

    Ok(())
}
