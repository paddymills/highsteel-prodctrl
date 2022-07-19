
use futures::channel::oneshot;

use super::{
    api::{JobShip, Mark, PartCompare},
    super::PartMap
};

pub struct GetJobShip {
    pub js: JobShip,
    pub respond_to: oneshot::Sender<JobShipResults>
}

#[derive(Debug)]
pub struct JobShipResults {
    pub js: JobShip,
    pub parts: PartMap
}

#[derive(Debug)]
pub struct PartResults {
    pub mark: Mark,
    pub compare: PartCompare
}
