
use clap::Parser;

use super::{CliMenuApp, Menu};
use super::questions;

/// Cli arguments that takes a job and shipment
// TODO: refactor to use api::JobShipment
#[derive(Parser, Debug, Default)]
#[clap(author, version, about)]
pub struct JobShipCli {

    /// Job number (with structure letter)
    #[clap(short, long, value_parser)]
    job: String,

    /// Shipment number
    #[clap(short, long, value_parser, default_value_t=1)]
    shipment: u32,
}

impl CliMenuApp for JobShipCli {}

impl Menu for JobShipCli {
    fn menu() -> Self {

        let job = questions::job();
        let shipment = questions::shipment();

        println!("Job: {:?}, Shipment: {:?}", job, shipment);

        Self { job, shipment }
    }
}
