
//! database connections, deserializating and schema

pub mod bom;
pub mod sn;

mod config;
pub use config::*;

pub enum DbType {
    Int(i32),
    Float(f32),
    String(String),
}
