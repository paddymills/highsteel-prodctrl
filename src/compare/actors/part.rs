
use crossbeam::channel;
use tokio::sync::mpsc;

use super::{
    get_sndb_pool,
    actor::*,
    api::{SnDbOps, JobShipMark, PartCompare}
};

type Sender = mpsc::Sender<PartResults>;
type Receiver = channel::Receiver<JobShipMark>;

#[derive(Debug)]
pub struct PartActor {
    #[allow(dead_code)]
    receiver: channel::Receiver<Message>
}

impl Actor for PartActor {
    fn new(receiver: channel::Receiver<Message>) -> Self {
        
        let rx = receiver.clone();
        tokio::spawn(async move {
            while let Ok(msg) = rx.recv() {
                match msg {
                    Message::GetPartData { js, mark, mut compare, respond_to } => {
                        debug!("Getting Sn data for {}_{}", js, mark);

                        let qn = get_sndb_pool().await
                            .get().await
                                .expect("Failed to get Sndb db client")
                            .qty_and_nested(&js, &mark).await;

                        compare.workorder = qn.qty;
                        compare.dxf = qn.nested;

                        let res = PartResults { js, mark, compare };
                
                        let _ = respond_to.send(res);
                    },
                    _ => ()
                }
            }
        });

        Self { receiver }
    }
}

pub async fn run_part_actor(results: Sender, queue: Receiver) {
    while let Ok((js, mark, qty)) = queue.recv() {
        debug!("Getting Sn data for {}_{}", js, mark);

        let qn = get_sndb_pool().await
            .get().await
                .expect("Failed to get Sndb db client")
            .qty_and_nested(&js, &mark).await;

        let compare = PartCompare { workorder: qn.qty, bom: qty, dxf: qn.nested };

        let res = PartResults { js, mark, compare };

        if let Err(_) = results.send( res ).await {
            debug!("failed to send Part results");
        }
    }
}
