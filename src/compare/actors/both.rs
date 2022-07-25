
use crossbeam::channel;

use super::{
    Actor, Message, JobShipResults, PartResults,
    db::{get_bom_pool, get_sndb_pool},
    super::{
        api::{find_dxf_file, BomDbOps, SnDbOps, PartCompare},
        PartMap
    }
};

#[derive(Debug)]
pub struct BothActor {
    #[allow(dead_code)]
    receiver: channel::Receiver<Message>
}

impl Actor for BothActor {
    fn new(receiver: channel::Receiver<Message>) -> Self {
        
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
                    Message::GetPartData { js, mark, mut compare, respond_to } => {
                        debug!("Getting Sn data for {}_{}", js, mark);

                        let qn = get_sndb_pool().await
                            .get().await
                                .expect("Failed to get Sndb db client")
                            .qty_and_nested(&js, &mark).await;

                        compare.workorder = qn.qty;
                        compare.dxf = qn.nested;

                        if !compare.dxf {
                            compare.dxf = find_dxf_file(&js, &mark);
                        }

                        let res = PartResults { js, mark, compare };
                
                        let _ = respond_to.send(res);
                    }
                }
            }
        });

        Self { receiver }
    }
}
  
