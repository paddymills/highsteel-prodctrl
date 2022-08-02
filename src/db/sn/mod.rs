
//! Sigmanest database

pub mod keys;

mod jobship;
pub use jobship::*;

use bb8::Pool;
use bb8_tiberius::ConnectionManager;
/// Builds a ['bb8::Pool`] for the Sigmanest database
/// 
/// ['bb8::Pool`]: https://docs.rs/bb8/latest/bb8/struct.Pool.html
pub async fn build_pool() -> Pool<ConnectionManager> {
    super::build_db_pool("Sigmanest".into(), super::HssConfig::Sigmanest, 16u32).await
}
