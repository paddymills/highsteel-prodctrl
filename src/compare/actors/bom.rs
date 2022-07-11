
use bb8_tiberius::IntoConfig;
use tiberius::Client;
use tokio::net::TcpStream;
use tokio_util::compat::*;
use tokio::sync::broadcast;

use super::task::*;
use crate::{db, Error, part::Part};

pub async fn bom_actor(tx: broadcast::Sender<Task>) -> Result<(), Error> {
    let mut rx = tx.subscribe();
    tx.send(Task::Init(1))?;

    let bom_cfg = db::HssConfig::Bom.into_config()?;
    let bom_tcp = TcpStream::connect(bom_cfg.get_addr()).await?;
    bom_tcp.set_nodelay(true)?;

    let mut client = Client::connect( bom_cfg, bom_tcp.compat_write() ).await?;

    debug!("Bom db actor initialized. awaiting tasks...");
    while let Ok(task) = rx.recv().await {
        match task {
            Task::GetParts(js) => {
                debug!("Getting parts for {}", js);

                client
                    .query(
                        "EXEC BOM.SAP.GetBOMData @Job=@P1, @Ship=@P2",
                        &[&js.job, &js.ship]
                    )
                    .await?
                    .into_first_result().await?
                    .into_iter()
                    .map(|row| Part::from(row))
                    .filter(|part| part.is_pl())
                    .map(|part| Task::Part(js.clone(), part.mark, PartCompare { bom: part.qty, ..Default::default() }) )
                    .for_each(|result| {
                        tx.send( result ).unwrap();
                    });
            },
            Task::Stop => break,
            _ => ()
        }
    }

    Ok(())
}
