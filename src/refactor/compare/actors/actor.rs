
use tokio::sync::oneshot;

use crate::api::JobShipment;
use super::{
    api::{Mark, PartCompare},
    super::PartMap
};

pub enum Message {
    GetJobShip {
        js: JobShipment,
        respond_to: oneshot::Sender<JobShipResults>
    },
    GetPartData {
        js: JobShipment,
        mark: Mark,
        compare: PartCompare,
        respond_to: oneshot::Sender<PartResults>
    }
}


#[derive(Debug)]
pub struct JobShipResults {
    pub js: JobShipment,
    pub parts: PartMap
}

#[derive(Debug)]
pub struct PartResults {
    pub js: JobShipment,
    pub mark: Mark,
    pub compare: PartCompare
}

pub trait Actor {
    fn new(receiver: crossbeam::channel::Receiver<Message>) -> Self;
}
