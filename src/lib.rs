
// #![warn(missing_docs)]

//! what run production control
//! 
//! # Production Control's Automation
//! 
//! ## Feature Flags
//! 
//! There are [feature flags] for some modules to reduce code compilation
//! where the features are not needed. By default, only the api is enabled.
//! 
//! - `full`: Enables all features
//! - `api`: Enables the internal business logic api
//! - `async`: Enables the [tokio] runtime and async dependencies
//! - `db`: Enables the mssql integration
//! - `xl`: Enables the excel data contectors
//! 
//! [feature flags]: https://doc.rust-lang.org/cargo/reference/features.html#the-features-section
//! [tokio]: https://tokio.rs
//! [tokio docs]: https://docs.rs/tokio/latest/tokio/
//! 

#[cfg(feature = "async")]
#[macro_use] extern crate async_trait;

#[macro_use] extern crate lazy_static;
#[macro_use] extern crate log;

mod core;
pub use crate::core::*; // must use crate::core to resolve ambiguity

pub mod config;
pub mod ui;

#[cfg(feature = "api")]
pub mod api;

#[cfg(feature="db")]
pub mod db;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

// refactor (remove later)
mod refactor;
pub use refactor::*;

