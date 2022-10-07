
use tiberius::Row;
use crate::api::JobShipment;

impl From<&Row> for JobShipment {
    fn from(row: &Row) -> Self {
        Self {
            job: row.get::<&str, _>("Data1").expect("Job is None").into(),
            ship: row.get::<&str, _>("Data2").expect("Shipment is None").into()
        }
    }
}
