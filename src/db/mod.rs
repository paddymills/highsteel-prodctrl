
//! database connections, deserializating and schema

pub mod bom;
pub mod sn;

mod conn;
pub use conn::*;

mod config;
pub use config::*;

/// Common database datatypes
pub enum DbType {
    Int(i32),
    Float(f32),
    String(String),
}
