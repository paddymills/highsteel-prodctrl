
//! Bom database

mod bom;
// TODO: explicit exports
pub use bom::*;

pub mod keys;

use super::prelude::{DbPool, DbClient};
/// Builds a ['bb8::Pool`] for the Bom database
/// 
/// ['bb8::Pool`]: https://docs.rs/bb8/latest/bb8/struct.Pool.html
pub async fn build_pool() -> DbPool {
    super::HssDatabase::Bom.build_pool().await
}

/// Connects to Bom database and returns a [`tiberius::Client`]
/// 
/// [`tiberius::Client`]: https://docs.rs/tiberius/latest/tiberius/struct.Client.html
pub async fn connect() -> DbClient {
    super::HssDatabase::Bom.connect().await
}
