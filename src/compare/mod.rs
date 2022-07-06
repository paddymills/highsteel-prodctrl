
mod driver;

pub use driver::BomWoDxfCompare;

#[derive(Clone, Debug, Default, Eq, PartialEq, Hash, Ord, PartialOrd)]
struct JobShip {
    job: String,
    ship: String
}

#[derive(Debug, Default)]
struct PartCompare {
    workorder: i32,
    bom: i32,
    dxf: bool
}

impl From<&tiberius::Row> for JobShip {
    fn from(row: &tiberius::Row) -> Self {
        Self {
            job: row.get::<&str, _>("Data1").expect("Job is None").into(),
            ship: row.get::<&str, _>("Data2").expect("Shipment is None").into()
        }
    }
}

fn get_qty(row: &tiberius::Row) -> i32 {
    row.get::<i32, _>("QtyOrdered").expect("Qty is None")
}
