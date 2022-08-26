
//! config reader

mod config;
pub use config::*;

mod db;
pub use db::Database;

lazy_static! {
    /// lazy evaluated global [`Config`]
    pub static ref CONFIG: Config = Config::read_config();
}
