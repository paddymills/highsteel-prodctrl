

#[macro_use] extern crate lazy_static;
#[macro_use] extern crate log;
#[macro_use] extern crate serde;

pub mod api;
pub mod paths;
pub mod processor;

use clap::Parser;
use simplelog::{Config, WriteLogger};
use std::fs::File;
use surrealdb::{
    Surreal,
    engine::remote::http::Https,
    opt::auth
};
use tokio::sync::mpsc;

use prodctrl::fs::{timestamped_file, is_empty_file};

use processor::ProdFileProcessor;

/// Confirmation files for SAP processing
#[derive(Debug, Parser)]
#[clap(name = "Sap Confirmation Files")]
#[clap(author, version)]
struct Args {
    /// Run without producing output or moving files
    #[clap(short, long)]
    dry_run: bool,
    
    #[clap(flatten)]
    verbose: clap_verbosity_flag::Verbosity
}

#[tokio::main]
async fn main() -> Result<(), prodctrl::Error> {
    let args = Args::parse();
    debug!("{:?}", args);

    let log_file = timestamped_file("log/cnf", "log");

    WriteLogger::init(
        args.verbose.log_level_filter(),
        Config::default(),
        File::create(&log_file).expect("failed to create log")
    ).expect("Failed to init logger");

    let db = Surreal::new::<Https>("hssl.apmills.xyz").await?;
    db.signin(
        auth::Database {
            namespace: "dev",
            database: "sap_cnf_files",
            username: "cnfproc",
            password: "wallbridge"
        }
    );

    let (tx, mut rx) = mpsc::channel(64);
    let handle = std::thread::spawn(move || {
        ProdFileProcessor::new(args.dry_run, tx)
            .process_files().unwrap();
    });

    // cache sent rows to db
    while let Some(res) = rx.recv().await {
        println!("Got: {}", res.record.mark);
        let created = db
            .create("prod_sent")
            .content(res)
            .await?;

        dbg!(created);
    }

    let _ = handle.join();

    // remove log file if nothing logged
    if is_empty_file(&log_file) {
        std::fs::remove_file(&log_file)?;
    }
    
    Ok(())
}
