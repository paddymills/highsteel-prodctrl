
use tokio::sync::{mpsc, oneshot};

use super::{actor::*, SNDB_POOL};
use super::api::{SnDbOps, JobShip, Mark, JobShipMark, PartCompare};

pub struct PartActorHandle {
    sender: mpsc::Sender<ActorMessage>
}

type ResultTuple = (Mark, PartCompare);

#[async_trait]
impl Handle<JobShipMark, ResultTuple> for PartActorHandle {
    fn new() -> Self {
        let (sender, recv) = mpsc::channel(8);
        let actor = PartActor::new(recv);
        tokio::spawn(PartActor::run_actor(actor));

        Self { sender }
    }

    async fn send(&self, vars: JobShipMark) -> ResultTuple {
        let (send, recv) = oneshot::channel();
        let msg = ActorMessage::GetPart(vars, send);

        let _ = self.sender.send( msg ).await;
        let res = recv.await.expect("Part actor task was killed");

        match res {
            ActorResult::Job(_, _) => unreachable!(),
            ActorResult::Part(mark, comp) => (mark, comp)
        }
    }
}

struct PartActor {
    receiver: mpsc::Receiver<ActorMessage>
}

impl PartActor {
    async fn get_part(js: &JobShip, mark: &Mark) -> PartCompare {
        let qn = SNDB_POOL
            .get()      // AsyncOnce
                .await
            .get()      // Pool
                .await
                .expect("Failed to get Bom db client")
            .qty_and_nested(&js, &mark)
                .await;

        PartCompare { workorder: qn.qty, dxf: qn.nested, ..Default::default() }
    }
}

#[async_trait]
impl Actor for PartActor {
    fn new(receiver: mpsc::Receiver<ActorMessage>) -> Self {
        Self { receiver }
    }

    fn handle_message(&mut self, msg: ActorMessage) {
        use ActorMessage::*;

        match msg {
            GetPart(jsm, respond_to) => {
                tokio::spawn(async move {
                    let (js, mark) = jsm;
                    let part = Self::get_part(&js, &mark).await;
                    
                    let _ = respond_to.send( ActorResult::Part(mark, part) );
                });
            },
            _ => ()
        }
    }

    async fn run_actor(mut actor: Self) {
        while let Some(msg) = actor.receiver.recv().await {
            actor.handle_message(msg);
        }
    }
}
