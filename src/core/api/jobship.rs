
use std::fmt::{Display, Formatter, Result};

/// Job and Shipment
#[derive(Clone, Debug, Default, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct JobShipment {
    /// Job number (with structure letter)
    // TODO: split job and structure
    pub job: String,
    /// Shipment number
    // TODO: refactor as number
    pub ship: String
}

impl Display for JobShipment {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}-{}", self.job, self.ship)
    }
}
