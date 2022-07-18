
use tokio::sync::{mpsc, oneshot};

use super::{
    api::{JobShip, JobShipMark, Mark, PartCompare},
    super::PartMap
};

pub enum ActorMessage {
    GetJob(JobShip, oneshot::Sender<ActorResult>),
    GetPart(JobShipMark, oneshot::Sender<ActorResult>)
}

pub enum ActorResult {
    Job(JobShip, PartMap),
    Part(Mark, PartCompare)
}

#[async_trait]
pub trait Actor {
    fn new(receiver: mpsc::Receiver<ActorMessage>) -> Self;
    fn handle_message(&mut self, msg: ActorMessage);
    async fn run_actor(mut actor: Self);
}

#[async_trait]
pub trait Handle<T, R> {
    fn new() -> Self;
    async fn send(&self, vars: T) -> R;
}