
//! Embedded configuration files

mod db;
pub use db::{DbConfig, DbConnParams};

use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "assets/"]
#[include = "*.toml"]
/// Embedded config toml files
struct ConfigAssets;
