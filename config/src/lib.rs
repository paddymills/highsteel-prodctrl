
#[macro_use]
extern crate serde;

mod db;
use db::Databases;

use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "../assets/"]
#[include = "*.toml"]
struct ConfigAssets;
