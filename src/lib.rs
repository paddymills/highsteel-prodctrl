
#[cfg(feature = "async")]
#[macro_use] extern crate async_trait;

#[macro_use] extern crate lazy_static;
#[macro_use] extern crate log;

mod core;
pub use crate::core::*; // must use crate::core to resolve ambiguity

pub mod config;
pub mod ui;

#[cfg(feature="api")]
pub mod api;

#[cfg(feature="db")]
pub mod db;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

// refactor (remove later)
mod refactor;
pub use refactor::*;

