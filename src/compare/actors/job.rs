
use crossbeam::channel;
use tokio::sync::mpsc;
use std::{thread};

use crate::compare::actors::run_part_actor;

use super::{
    get_bom_pool,
    actor::{Actor, JobShipResults},
    api::{BomDbOps, JobShip, PartCompare},
    super::PartMap, Message
};

type Sender = mpsc::Sender<JobShipResults>;
type Receiver = channel::Receiver<JobShip>;

const NUM_JOB_ACTORS: usize = 8;
const NUM_PART_ACTORS: usize = 4;
const PART_QUEUE_SIZE: usize = 8 * NUM_PART_ACTORS;

#[derive(Debug)]
pub struct JobActor {
    #[allow(dead_code)]
    receiver: channel::Receiver<Message>
}

impl Actor for JobActor {
    fn new(receiver: channel::Receiver<Message>) -> Self {
        debug!("Spawning Job Actor");

        let rx = receiver.clone();
        tokio::spawn(async move {
            while let Ok(msg) = rx.recv() {
                match msg {
                    Message::GetJobShip { js, respond_to } => {
                        let mut parts = PartMap::new();
                        
                        get_bom_pool().await
                            .get().await
                                .expect("Failed to get Bom db client")
                            .parts_qty(&js).await
                            .into_iter()
                            .for_each(|res| {
                                let _ = parts.insert(res.mark, PartCompare { bom: res.qty, ..Default::default() });
                            });
                        
                        let res = JobShipResults { js, parts };
                
                        let _ = respond_to.send(res);
                    },
                    _ => ()
                }
            }
        });

        Self { receiver }
    }
}

pub async fn run_job_actor(results: Sender, queue: Receiver) {
    while let Ok(js) = queue.recv() {
        // spawn part workers
        debug!("Spawning {} part workers", js);
        let (tx, mut rx) = {
            // queue channel
            let (tx_queue, rx_queue) = channel::bounded(PART_QUEUE_SIZE);
            
            // results channel
            let (tx_results, rx_results) = mpsc::channel(PART_QUEUE_SIZE);

            // spawn actors
            for _i in 0..NUM_PART_ACTORS {
                // debug!("Spawning {} part worker {}", js, i);
                // spawn part actor
                let (tx, rx) = (tx_results.clone(), rx_queue.clone());
                tokio::spawn( run_part_actor(tx, rx) );
            }

            (tx_queue, rx_results)
        };
        
        {
            let js = js.clone();
            tokio::spawn(async move {
                // get parts
                let qtys = get_bom_pool().await
                    .get().await
                        .expect("Failed to get Bom db client")
                    .parts_qty(&js).await;
        
                if qtys.len() == 0 {
                    debug!("Received no Bom results for {}", js);
                }
        
                for part in qtys {
                    debug!("got Bom result: {}_{} = {}", js, part.mark, part.qty);
        
                    let tx = tx.clone();
                    let js = js.clone();
                    thread::spawn(move || {
                        if tx.is_full() {
                            debug!("{} queue is full", js);
                        }
                        let _ = tx.send( (js, part.mark, part.qty) );
                    });
                }
            });
        }
        
        // collect results
        let mut parts: PartMap = PartMap::new();
        while let Some(res) = rx.recv().await {
            debug!("Received part result: {}", res.mark);
            parts.insert(res.mark, res.compare);
        }

        // send sesult
        let res = JobShipResults { js, parts };
        if let Err(_) = results.send( res ).await {
            debug!("failed to send Job results");
        }
    }
}

pub async fn spawn_actors(tx: Sender, rx: Receiver) {
    for _ in 0..NUM_JOB_ACTORS {
        let (tx, rx) = (tx.clone(), rx.clone());

        tokio::spawn( run_job_actor(tx, rx) );
    }
}
