

//! Bom database

mod bom;
// TODO: explicit exports
pub use bom::*;

pub mod keys;

use bb8::Pool;
use bb8_tiberius::ConnectionManager;
/// Builds a ['bb8::Pool`] for the Bom database
/// 
/// ['bb8::Pool`]: https://docs.rs/bb8/latest/bb8/struct.Pool.html
pub async fn build_pool() -> Pool<ConnectionManager> {
    super::build_db_pool("Bom".into(), super::HssConfig::Bom, 2u32).await
}
