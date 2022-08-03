
use prodctrl::prelude::*;

use clap::{Parser, Subcommand};
use prodctrl::JobShipment;
use prodctrl::db::bom;

// TODO: completions
#[derive(Debug, Parser)]
#[clap(author, version, about)]
struct Args {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Bom database ops
    Bom {
        job_ship: JobShipment,

        #[clap(short, long)]
        /// Generate xml files for plate parts
        xml: bool,

        #[clap(short, long)]
        /// Secondary parts only
        secondary: bool,

        #[clap(short, long)]
        /// Plate parts only
        plate: bool,
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    println!("{:#?}", args);

    use Commands::*;
    match &args.command {
        Bom { job_ship, .. } => {
            let pool = bom::build_pool().await;
            let parts = bom::init_bom( pool, &job_ship.job, job_ship.ship.parse()? )
                .await?
                .into_iter();

            parts.for_each( |part| println!("{:#?}", part) );
        }
    }

    Ok(())
}
