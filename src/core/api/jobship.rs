
use std::fmt::{Display, Formatter, Result};

/// Job and Shipment
// TODO: refactor ship: String -> shipment: u32
#[derive(Clone, Debug, Default, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct JobShipment {
    pub job: String,
    pub ship: String
}

impl Display for JobShipment {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}-{}", self.job, self.ship)
    }
}
