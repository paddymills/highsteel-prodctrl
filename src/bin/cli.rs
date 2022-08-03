
use prodctrl::prelude::*;

use clap::{Parser, Subcommand};
use prodctrl::JobShipment;
use prodctrl::db::bom;

// TODO: completions https://docs.rs/clap_complete/latest/clap_complete/index.html
#[derive(Debug, Parser)]
#[clap(author, version, about)]
struct Args {
    #[clap(subcommand)]
    command: Option<Commands>,

    #[clap(flatten)]
    verbose: clap_verbosity_flag::Verbosity,
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
    },
    /// Lateral bracing listing
    Lb {
        job_ship: JobShipment,

        #[clap(short, long)]
        /// Filter parts of this shape
        shape: String,
        #[clap(short, long)]
        /// Filter parts on this drawing
        drawing: String,
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    println!("{:#?}", args);

    use Commands::*;
    if let Some(cmd) = &args.command {
        match cmd {
            Bom { job_ship, .. } => {
                let pool = bom::build_pool().await;
                let parts = bom::init_bom( pool, &job_ship.job, job_ship.ship.parse()? )
                    .await?
                    .into_iter();
    
                // TODO: filters
    
                parts.for_each( |part| println!("{:#?}", part) );
            },
            Lb { .. } => {
                // TODO: implement this from examples
            }
        }
    } else {
        println!("No args given. Default to menu.")
    }

    Ok(())
}
