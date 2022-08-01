
mod app;
pub use app::App;

pub mod questions;

mod job_ship;
pub use job_ship::JobShipCli;

pub trait CliMenuApp
where
    Self: clap::Parser + Menu
{
    fn init() -> Self {
        match std::env::args().nth(1) {
            Some(_) => Self::parse(),
            None    => Self::menu()
        }
    }
}

pub trait Menu {
    fn menu() -> Self;
}

pub fn error(msg: &str) {
    eprintln!("{}", msg);
}

