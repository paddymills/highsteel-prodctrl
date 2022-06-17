
mod app;

pub use app::App;
pub mod part;

pub type Error = Box<dyn std::error::Error>;
