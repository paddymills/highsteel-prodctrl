
use bb8_tiberius::IntoConfig;
use tiberius::Client;
use tokio::net::TcpStream;
use tokio_util::compat::*;
use tokio::sync::broadcast;

use super::task::*;
use crate::{db, Error};

pub async fn sndb_actor(tx: broadcast::Sender<Task>) -> Result<(), Error> {
    let mut rx = tx.subscribe();
    tx.send(Task::Init(1))?;

    let sndb_cfg = db::HssConfig::Sigmanest.into_config()?;
    let sndb_tcp = TcpStream::connect(sndb_cfg.get_addr()).await?;
    sndb_tcp.set_nodelay(true)?;

    let mut client = Client::connect( sndb_cfg, sndb_tcp.compat_write() ).await?;

    debug!("Sigmanest db actor initialized. awaiting tasks...");

    while let Ok(task) = rx.recv().await {
        match task {
            Task::GetJobs => {
                debug!("Getting jobs list");

                client
                    .simple_query("
                        SELECT DISTINCT Data1, Data2
                        FROM Part
                        WHERE WONumber LIKE '%-%' AND Data1 LIKE '1%'
                        GROUP BY Data1, Data2
                    ").await?
                    .into_first_result().await?
                    .into_iter()
                    // .take(5)
                    .map(|row| JobShip::from(&row) )
                    .for_each(|js| {
                        tx.send(Task::Job(js)).unwrap();
                    });
            },
            Task::GetWorkOrder(js, mark) => {
                debug!("Getting workorder for {} > {}", js, mark);

                client
                    .query(
                        "
                            SELECT
                                PartName,
                                QtyOrdered
                            FROM Part
                            WHERE Data1=@P1 AND Data2=@P2 AND PartName=@P3
                            AND WONumber LIKE '%-%'
                        ", &[&js.job, &js.ship, &format!("{}_{}", js.job, mark)]
                    ).await?
                    .into_first_result().await?
                    .into_iter()
                    .map(|row| Task::WorkOrder(js.clone(), mark.clone(), get_qty(&row)) )
                    .for_each(|res| {
                        tx.send( res ).unwrap();
                    });
            },
            Task::GetDxf(js, mark) => {
                debug!("Getting imported for {} > {}", js, mark);

                let res = client
                    .query(
                        "
                            SELECT 1
                            FROM PartsLibrary
                            WHERE PartName=@P1
                        ", &[&format!("{}_{}", js.job, mark)]
                    ).await?
                    .into_row().await?;

                debug!("looking for {}_{}, got {:?}", js.job, mark, res);
                match res {
                    Some(_) => { tx.send(Task::Dxf(js, mark))?; },
                    None    => { tx.send(Task::NoDxf)?; },
                    // None => ()
                }
            },
            Task::Stop => break,
            _ => ()
        }
    }

    Ok(())
}
