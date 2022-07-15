
use tokio::sync::Mutex;
use std::sync::Arc;
use std::thread;
use super::*;

type Responder = DriverChannel::Sender<ActorResult>;

pub const ACTORS: usize = 16;
const CHANNEL_SIZE: usize = ACTORS * 4;
const RETURN_CHANNEL_MULT: usize = 4;

#[derive(Debug)]
pub struct ActorHandle {
    // handle -> actor queue
    q_send: QueueChannel::Sender<ActorTask>,
    q_recv: QueueChannel::Receiver<ActorTask>,

    // actor -> handle results
    r_send: ResultChannel::Sender<ActorResult>,
    // r_recv: ResultChannel::Receiver<ActorResult>,

    // handle -> driver results
    // respond_to: Responder,

    actors: Arc<Mutex<usize>>,
}

impl ActorHandle
    // where T: PartActor + Send + 'static
{

    pub async fn new(respond_to: Responder) -> Self {
        let (q_send, q_recv) = QueueChannel::bounded(CHANNEL_SIZE);
        let (r_send, mut r_recv) = ResultChannel::channel(CHANNEL_SIZE * RETURN_CHANNEL_MULT);
        let actors = Arc::new(Mutex::new(0usize));

        {
            let actors = Arc::clone(&actors);
            tokio::spawn(async move {
                while let Some(msg) = r_recv.recv().await {
                    match msg {
                        ActorResult::ActorShutdown => {
                            *actors.lock().await -= 1
                        },
                        _ => {
                            if let Err(_) = respond_to.send(msg).await {
                                debug!("Receiver dropped");
                            }
                        }
                    }
                }
            });
        }
        
        Self { q_send, q_recv, r_send, actors }
    }

    fn get_actor_channels(&self) -> (ActorSender, ActorReceiver) {
        (self.r_send.clone(), self.q_recv.clone())
    }

    pub fn send(&self, task: ActorTask) {
        let actors = Arc::clone(&self.actors);
        let (send, recv) = self.get_actor_channels();
        tokio::spawn(async move {
            let mut num_actors = actors.lock().await;
    
            // increases actor pool until max
            // this means
            //    number of actors = min(actor pool size, queue size)
            // might be neat to spawn actors based on size of queue
            if *num_actors < ACTORS {
                tokio::spawn(Actor::spawn(send, recv));
    
                *num_actors += 1;
            }
        });

        if self.is_full() {
            debug!("db queue channel full. Send will have to wait...");
        }
        
        let sender = self.q_send.clone();
        thread::spawn(move || { let _ = sender.send( task ); });
    }

    pub fn is_full(&self) -> bool {
        self.q_send.is_full()
    }

    pub async fn is_done(&self) -> bool {
        match *self.actors.lock().await {
            0 => true,
            _ => false
        }
    }
}
