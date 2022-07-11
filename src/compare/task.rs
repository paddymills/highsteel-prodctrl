use std::fmt::Display;


#[derive(Clone, Debug)]
pub enum Task {
    Init(u8),

    GetJobs,
    GetParts(JobShip),
    GetWorkOrder(JobShip, String),
    GetDxf(JobShip, String),

    // results
    Job(JobShip),
    Part(JobShip, String, PartCompare),
    WorkOrder(JobShip, String, i32),
    // Bom(JobShip, String, i32),
    Dxf(JobShip, String),

    // JobSearchComplete,
    NoDxf,

    Stop,
}

#[derive(Clone, Debug, Default, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct JobShip {
    pub job: String,
    pub ship: String
}

#[derive(Clone, Debug, Default)]
pub struct PartCompare {
    pub workorder: i32,
    pub bom: i32,
    pub dxf: bool
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
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}-{}", self.job, self.ship)
    }
}

pub fn get_qty(row: &tiberius::Row) -> i32 {
    row.get::<i32, _>("QtyOrdered").expect("Qty is None")
}
