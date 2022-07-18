
use tokio::sync::{mpsc, oneshot};

use super::{
    {BOM_POOL, PartActorHandle},
    actor::*,
    api::{BomDbOps, JobShip},
    super::PartMap
};

pub struct JobActorHandle {
    sender: mpsc::Sender<ActorMessage>
}

type ResultMap = (JobShip, PartMap);

#[async_trait]
impl Handle<JobShip, ResultMap> for JobActorHandle {
    fn new() -> Self {
        let (sender, recv) = mpsc::channel(8);
        let actor = JobActor::new(recv);
        tokio::spawn(JobActor::run_actor(actor));

        Self { sender }
    }

    async fn send(&self, js: JobShip) -> ResultMap {
        let (send, recv) = oneshot::channel();
        let msg = ActorMessage::GetJob(js, send);

        let _ = self.sender.send( msg ).await;
        match recv.await.expect("Job actor task was killed") {
            ActorResult::Job(js, map) => (js, map),
            ActorResult::Part(_, _) => unreachable!()
        }
    }
}

struct JobActor {
    receiver: mpsc::Receiver<ActorMessage>
}

impl JobActor {
    async fn get_job(js: &JobShip) -> PartMap {
        let qtys = BOM_POOL
            .get()      // AsyncOnce
                .await
            .get()      // Pool
                .await
                .expect("Failed to get Bom db client")
            .parts_qty(&js)
                .await;

        let (tx, mut rx) = mpsc::channel(64);
        for part in qtys {
            let tx = tx.clone();
            let js = js.clone();
            tokio::spawn(async move {
                let handle = PartActorHandle::new();
                let (mark, mut comp) = handle.send((js, part.mark)).await;
                comp.bom = part.qty;

                let _ = tx.send( (mark, comp) ).await;
            });
        }
        
        drop(tx);
        let mut res: PartMap = PartMap::new();
        while let Some((mark, compare)) = rx.recv().await {
            res.insert(mark, compare);
        }
        
        res
    }
}

#[async_trait]
impl Actor for JobActor {
    fn new(receiver: mpsc::Receiver<ActorMessage>) -> Self {
        Self { receiver }
    }

    fn handle_message(&mut self, msg: ActorMessage) {
        use ActorMessage::*;

        match msg {
            GetJob(js, respond_to) => {
                tokio::spawn(async move {
                    let parts = Self::get_job(&js).await;
                    
                    let _ = respond_to.send( ActorResult::Job(js, parts) );
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
