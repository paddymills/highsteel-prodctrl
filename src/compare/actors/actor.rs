
use tokio::sync::oneshot;

use super::{
    api::{JobShip, Mark, PartCompare},
    super::PartMap
};

pub enum Message {
    GetJobShip {
        js: JobShip,
        respond_to: oneshot::Sender<JobShipResults>
    },
    GetPartData {
        js: JobShip,
        mark: Mark,
        compare: PartCompare,
        respond_to: oneshot::Sender<PartResults>
    }
}


#[derive(Debug)]
pub struct JobShipResults {
    pub js: JobShip,
    pub parts: PartMap
}

#[derive(Debug)]
pub struct PartResults {
    pub js: JobShip,
    pub mark: Mark,
    pub compare: PartCompare
}

pub trait Actor {
    fn new(receiver: crossbeam::channel::Receiver<Message>) -> Self;
}
