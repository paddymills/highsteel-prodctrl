
mod driver;
mod progress;

use prodctrl;

pub mod actors;
pub mod api;
pub use driver::BomWoDxfCompare;
pub use progress::ProgressBars;


use prodctrl::api::JobShipment;
use std::collections::BTreeMap;
pub type PartMap = BTreeMap<String, api::PartCompare>;
pub type JobShipMap = BTreeMap<JobShipment, PartMap>;
