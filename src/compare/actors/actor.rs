
use std::time::Duration;

// actor based system from https://ryhl.io/blog/actors-with-tokio/
use super::*;

pub const ACTOR_TIMEOUT: Duration = Duration::from_secs(5);

#[derive(Debug)]
pub enum ActorTask {
    GetJobs,
    GetJob { js: JobShip  },
    GetPart { js: JobShip, mark: Mark  },
    // GetDxf { js: JobShip, mark: Mark  },
}

#[derive(Clone, Debug)]
pub enum ActorResult {
    Job(JobShip),
    JobProcessed(JobShip),

    Part(JobShip, Mark, Qty),
    Bom(JobShip, Mark, Qty),
    WorkOrder(JobShip, Mark, Qty),
    Dxf(JobShip, Mark),
    NoDxf,

    ActorShutdown,
    NotImplemented(String),
    Error
}


#[derive(Debug)]
pub struct Actor {
    pub sender: ActorSender,
    receiver: ActorReceiver,
}

impl Actor {
    fn new(sender: ActorSender, receiver: ActorReceiver) -> Self {
        debug!("Actor initialized. awaiting tasks...");

        Self { sender, receiver }
    }

    pub async fn spawn(sender: ActorSender, receiver: ActorReceiver) {
        debug!("actor spawning");

        let actor = Self::new(sender, receiver);
        tokio::spawn(Self::run_actor(actor));
    }

    pub async fn handle_task(&mut self, task: ActorTask) -> Result<(), crate::Error> {
        let send = self.sender.clone();

        match task {
            // bom_db
            ActorTask::GetJob { js } => bom_db::get_parts(send, js).await?,

            // sn_db
            ActorTask::GetJobs => sn_db::get_jobs_list(send).await?,
            ActorTask::GetPart { js, mark } => sn_db::get_part_qty(send, js, mark).await?,
            // ActorTask::GetDxf {js, mark } => sn_db::has_dxf(send, js, mark).await?,

            // Unimplemented
            // _ => {
            //     let _ = self.sender.send( ActorResult::NotImplemented(format!("Actor received unimplemented task: {:?}", task)) ).await;
            // }
        }

        Ok(())
    }

    async fn run_actor(mut actor: Self) {
        while let Ok(msg) = actor.receiver.recv_timeout(ACTOR_TIMEOUT) {
            let _ = actor.handle_task(msg).await;
        }

        tokio::spawn(async move {
            let _ = actor.sender.send( ActorResult::ActorShutdown ).await;
            debug!("actor disconnected");
        });
    }
}
