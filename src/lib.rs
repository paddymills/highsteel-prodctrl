
mod app;

pub use app::App;
pub mod cli;
pub mod cnf;
pub mod part;

pub type Error = Box<dyn std::error::Error>;
