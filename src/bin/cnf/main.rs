

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
use std::thread;
use surrealdb::{
    Surreal,
    engine::remote::{http::Https, ws::Ws},
    sql::Thing
};
use tokio::sync::mpsc;

use prodctrl::fs::{timestamped_file, is_empty_file};
use prodctrl::config::DbConfig;

use processor::ProdFileProcessor;

/// Confirmation files for SAP processing
#[derive(Debug, Parser)]
#[clap(name = "Sap Confirmation Files")]
#[clap(author, version)]
struct Args {
    /// Run without producing output or moving files
    #[clap(short, long)]
    dry_run: bool,
    
    #[clap(short, long)]
    upload_processed: bool,

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

    let config = DbConfig::from_embed().prodctrl;
    trace!("database config: {:?}", config);
    let db = Surreal::new::<Https>(config.server.as_ref()).await?;
    db.signin( config.surreal_auth() ).await?;
    db
        .use_ns(config.instance.unwrap())
        .use_db(config.database.unwrap())
        .await?;

    let (tx, mut rx) = mpsc::channel(64);
    if args.upload_processed {
        debug!("getting processed filenames from database");
        let mut result = db
            .query("SELECT * FROM array::distinct((SELECT VALUE filename FROM prod_sent))")
            .await?;
        
        let processed_files: Vec<String> = result.take(0)?;

        thread::spawn(move || 
            ProdFileProcessor::new(args.dry_run, tx)
                .log_processed(processed_files).unwrap()
        );
    }
    
    else {
        thread::spawn(move || 
            ProdFileProcessor::new(args.dry_run, tx)
                .process_files().unwrap()
        );
    }

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
            Ok(c)  => { trace!("{:?}", c); },
            Err(e) => { error!("{:?}", e); }
        }
    }

    // remove log file if nothing logged
    if is_empty_file(&log_file) {
        std::fs::remove_file(&log_file)?;
    }
    
    Ok(())
}
