
mod driver;
mod progress;

pub mod actors;
pub mod api;
pub use driver::BomWoDxfCompare;
pub use progress::ProgressBars;


use crate::api::JobShipment;
use std::collections::BTreeMap;
pub type PartMap = BTreeMap<String, api::PartCompare>;
pub type JobShipMap = BTreeMap<JobShipment, PartMap>;
