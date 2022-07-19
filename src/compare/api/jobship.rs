
use std::fmt::{Display, Formatter, Result};

use super::{Mark, Qty};

pub type JobShipMark = (JobShip, Mark, Qty);

#[derive(Clone, Debug, Default, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct JobShip {
    pub job: String,
    pub ship: String
}

impl From<&tiberius::Row> for JobShip {
    fn from(row: &tiberius::Row) -> Self {
        Self {
            job: row.get::<&str, _>("Data1").expect("Job is None").into(),
            ship: row.get::<&str, _>("Data2").expect("Shipment is None").into()
        }
    }
}

impl Display for JobShip {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}-{}", self.job, self.ship)
    }
}

// pub struct JobShipMark {
//     pub job: String,
//     pub ship: String,
//     pub mark: Mark
// }

// impl JobShipMark {
//     pub fn from(js: JobShip, mark: Mark)
// }
