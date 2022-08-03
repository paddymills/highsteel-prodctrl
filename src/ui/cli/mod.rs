
//! Command line and text user interfaces

mod app;
pub use app::App;

pub mod questions;

mod job_ship;
pub use job_ship::JobShipCli;

/// base CLI and Menu driven app trait
pub trait CliMenuApp
where
    Self: clap::Parser + Menu
{
    /// Init app
    fn init() -> Self {
        match std::env::args().nth(1) {
            Some(_) => Self::parse(),
            None    => Self::menu()
        }
    }
}

/// base Menu system
pub trait Menu {
    /// show menu
    fn menu() -> Self;
}

/// global error impl
// TODO: modularize this based on CLI/GUI usage (maybe features?)
pub fn error(msg: &str) {
    eprintln!("{}", msg);
}

