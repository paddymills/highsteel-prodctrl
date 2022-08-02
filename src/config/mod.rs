
//! config reader

mod config;
pub use config::*;


use figment::Figment;
lazy_static! {
    pub static ref CONFIG: Figment = config::read_config();
}
