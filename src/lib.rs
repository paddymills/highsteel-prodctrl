
#![warn(missing_docs)]

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

#[allow(unused_imports)]
#[macro_use] extern crate log;


pub use prodctrl_config as config;

#[cfg(feature="db")]
pub use prodctrl_db as db;

#[cfg(feature="api")]
pub use prodctrl_api::*;

// TODO: paths module
// TODO: regex module

pub mod logging;

pub mod fs;
pub mod ui;

/// Dynamic error type for convenience
pub type Error = Box<dyn std::error::Error>;
/// Dynamic result type for convenience using [`crate::Error`]
pub type Result<T> = std::result::Result<T, Error>;

/// Common types and utils
/// 
/// ```
/// use prodctrl::prelude::*;
/// ```
pub mod prelude {
    pub use super::Error;
    pub use super::Result;
}
