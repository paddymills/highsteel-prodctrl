
use prodctrl::prelude::*;

use clap::{Parser, Subcommand};
use prodctrl::JobShipment;
use prodctrl::db::bom;
// use prodctrl::fs::cnf::ProdFileProcessor;

#[derive(Debug, Parser)]
#[clap(author, version, about)]
struct Args {
    #[clap(subcommand)]
    command: Commands,

    #[clap(short, long, help="Run without producing output or moving files")]
    dry_run: bool,
}

#[derive(Debug, Subcommand)]
enum Commands {
    Bom {
        job_ship: JobShipment
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    println!("{:#?}", args);

    use Commands::*;
    match &args.command {
        Bom { job_ship } => {
            let pool = bom::build_pool().await;
            bom::init_bom( pool, &job_ship.job, job_ship.ship.parse()? )
                .await?
                .into_iter()
                .for_each( |part| println!("{:#?}", part) );
        }
    }

    Ok(())
}
