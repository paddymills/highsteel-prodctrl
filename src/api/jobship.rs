
use std::fmt::{Display, Formatter, Result};
use tiberius::Row;

#[derive(Clone, Debug, Default, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct JobShipment {
    pub job: String,
    pub ship: String
}

impl From<&Row> for JobShipment {
    fn from(row: &Row) -> Self {
        Self {
            job: row.get::<&str, _>("Data1").expect("Job is None").into(),
            ship: row.get::<&str, _>("Data2").expect("Shipment is None").into()
        }
    }
}

impl Display for JobShipment {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}-{}", self.job, self.ship)
    }
}
