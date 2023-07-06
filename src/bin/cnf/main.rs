

#[macro_use] extern crate lazy_static;
#[macro_use] extern crate log;
#[macro_use] extern crate serde;

pub mod api;
pub mod paths;
pub mod processor;

use clap::Parser;
use sha2::{Digest, Sha256};
use simplelog::{Config, WriteLogger};
use std::fs::File;
use surrealdb::{
    Surreal,
    engine::remote::http::Https,
    opt::auth, sql::Thing
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

#[derive(Debug, Deserialize)]
struct Record {
    #[allow(dead_code)]
    id: Thing,
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
    ).await?;
    db
        .use_ns("dev")
        .use_db("sap_cnf_files")
        .await?;

    let (tx, mut rx) = mpsc::channel(64);
    let handle = std::thread::spawn(move || {
        ProdFileProcessor::new(args.dry_run, tx)
            .process_files().unwrap();
    });

    // cache sent rows to db
    let mut hasher = Sha256::new();
    while let Some(res) = rx.recv().await {
        // create id from (Mark, PartWbs, Program)
        hasher.update(&res.record.mark);
        hasher.update(&res.record.part_wbs);
        hasher.update(&res.record.program);
        let id = hex::encode( hasher.finalize_reset() );

        let created: Result<Record, surrealdb::Error> = db
            .update( ("prod_sent", id) )
            .content(res)
            .await;

        match created {
            Ok(c)  => { dbg!(c); },
            Err(e) => { error!("{:#?}", e); }
        }
    }

    let _ = handle.join();

    // remove log file if nothing logged
    if is_empty_file(&log_file) {
        std::fs::remove_file(&log_file)?;
    }
    
    Ok(())
}
