
//! Sigmanest database

pub(crate) mod api_compat;

pub mod keys;

mod cnf;
pub use cnf::SnCnfDbOps;

use crate::prelude::{DbPool, DbClient};
/// Builds a ['bb8::Pool`] for the Sigmanest database
/// 
/// ['bb8::Pool`]: https://docs.rs/bb8/latest/bb8/struct.Pool.html
pub async fn build_pool() -> DbPool {
    super::HssDatabase::Sigmanest.build_pool().await
}

/// Connects to Sigmanest database and returns a [`tiberius::Client`]
/// 
/// [`tiberius::Client`]: https://docs.rs/tiberius/latest/tiberius/struct.Client.html
pub async fn connect() -> DbClient {
    super::HssDatabase::Sigmanest.connect().await
}