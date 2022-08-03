

//! Bom database

mod bom;
// TODO: explicit exports
pub use bom::*;

pub mod keys;

use super::prelude::*;
/// Builds a ['bb8::Pool`] for the Bom database
/// 
/// ['bb8::Pool`]: https://docs.rs/bb8/latest/bb8/struct.Pool.html
pub async fn build_pool() -> DbPool {
    super::build_db_pool("Bom".into(), super::HssConfig::Bom, 2u32).await
}

/// Connects to a database and returns a [`tiberius::Client`]
/// 
/// [`tiberius::Client`]: https://docs.rs/tiberius/latest/tiberius/struct.Client.html
pub async fn connect() -> DbClient {
    super::build_db_conn("Bom".into(), super::HssConfig::Bom).await
}
