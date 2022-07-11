
#[macro_use] extern crate lazy_static;
#[macro_use] extern crate log;

mod app;

pub use app::App;
pub mod cli;
pub mod cnf;
pub mod compare;
pub mod config;
pub mod db;
pub mod part;

pub type Error = Box<dyn std::error::Error>;
