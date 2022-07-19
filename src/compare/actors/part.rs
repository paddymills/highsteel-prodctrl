
use crossbeam::channel;
use tokio::sync::mpsc;

use super::{
    get_sndb_pool,
    actor::*,
    api::{SnDbOps, JobShipMark, PartCompare}
};

type Sender = mpsc::Sender<PartResults>;
type Receiver = channel::Receiver<JobShipMark>;


pub async fn run_part_actor(results: Sender, queue: Receiver) {
    while let Ok((js, mark, qty)) = queue.recv() {
        debug!("Getting Sn data for {}_{}", js, mark);

        let qn = get_sndb_pool().await
            .get().await
                .expect("Failed to get Sndb db client")
            .qty_and_nested(&js, &mark).await;

        let compare = PartCompare { workorder: qn.qty, bom: qty, dxf: qn.nested };

        let res = PartResults { mark, compare };

        if let Err(_) = results.send( res ).await {
            debug!("failed to send Part results");
        }
    }
}
