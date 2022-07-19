
use crossbeam::channel;
use tokio::sync::mpsc;
use std::thread;

use crate::compare::actors::run_part_actor;

use super::{
    get_bom_pool,
    actor::{JobShipResults},
    api::{BomDbOps, JobShip},
    super::PartMap
};

type Sender = mpsc::Sender<JobShipResults>;
type Receiver = channel::Receiver<JobShip>;

const NUM_JOB_ACTORS: usize = 8;
const NUM_PART_ACTORS: usize = 4;
const PART_QUEUE_SIZE: usize = 8 * NUM_PART_ACTORS;

async fn run_job_actor(results: Sender, queue: Receiver) {
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
