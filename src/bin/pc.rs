
#[macro_use] extern crate log;

use clap::{Parser, Subcommand};
use simplelog::{Config, SimpleLogger};
use std::path::PathBuf;

use prodctrl::prelude::*;

// TODO: completions https://docs.rs/clap_complete/latest/clap_complete/index.html

/// Production Control management system
#[derive(Debug, Parser)]
#[clap(name = "Production Control")]
#[clap(author, version)]
struct Cli {
    /// subcommand to run
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
    },

    /// push files to client machines
    Push {
        files: Vec<PathBuf>,

        #[clap(short, long)]
        all: Option<bool>
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
