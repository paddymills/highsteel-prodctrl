
// #![warn(missing_docs)]

//! what run production control
//! 
//! # Production Control's Automation
//! 
//! ## Feature Flags
//! 
//! Since certain non-core features require a large number of external libraries
//! there are [feature flags] for some modules to reduce code compilation
//! where the features are not needed. No features are enabled by default.
//! 
//! - `full`: Enables all features
//! - `db`: Enables database integration (along with async and mssql dependencies)
//! - `gui`: Enables graphical interfaces
//! - `xl`: Enables excel data contectors
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

#[cfg(feature="db")]
pub mod db;

pub mod fs;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;
