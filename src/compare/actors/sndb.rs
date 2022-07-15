
pub mod sn_db {
    use async_once::AsyncOnce;
    use bb8::Pool;
    use bb8_tiberius::ConnectionManager;
    
    use super::super::*;
    use crate::Error;
    
    const POOL_SIZE: u32 = ACTORS as u32;
    
    lazy_static!(
        static ref SNDB_POOL: AsyncOnce<Pool<ConnectionManager>> = AsyncOnce::new( async {
            debug!("****************************************");
            debug!("** init Sndb db pool  ******************");
            debug!("****************************************");
    
            let pool = match Pool::builder()
                .max_size(POOL_SIZE)
                .build(
                    match ConnectionManager::build(crate::db::HssConfig::Sigmanest) {
                        Ok(conn_mgr) => conn_mgr,
                        Err(_) => panic!("ConnectionManager failed to connect to database")
                    }
                )
                .await {
                    Ok(pool) => pool,
                    Err(_) => panic!("Sndb Pool failed to build")
                };
    
            pool
        });
    );

    pub async fn get_jobs_list(sender: ActorSender) -> Result<(), Error> {
        debug!("Getting jobs list");

        let iter = SNDB_POOL
            .get().await    // AsyncOnce
            .get().await?   // Pool
            .simple_query("
                SELECT DISTINCT Data1, Data2
                FROM Part
                WHERE WONumber LIKE '%-%' AND Data1 LIKE '1%'
                GROUP BY Data1, Data2
            ")
            .await?
            .into_first_result().await?
            .into_iter()
            .map(|row| JobShip::from(&row) );

        for js in iter {
            debug!("Got Job: {:?}", js);

            let sender = sender.clone();
            tokio::spawn(async move { let _ = sender.send( ActorResult::Job(js) ).await; });
        }

        Ok(())
    }

    pub async fn get_part_qty(sender: ActorSender, js: JobShip, mark: Mark) -> Result<(), Error> {
        debug!("Getting workorder for {} > {}", js, mark);

        let row = SNDB_POOL
            .get().await    // AsyncOnce
            .get().await?   // Pool
            .query(
                "
                    SELECT
                        SUM(Qty) AS Qty,
                        (
                            CASE
                                WHEN SUM(Nested) > 0 THEN 1
                                ELSE 0
                            END
                        ) AS Nested
                    FROM (
                        SELECT
                            PartName,
                            QtyOrdered AS Qty,
                            QtyCompleted AS Nested
                        FROM Part
                        WHERE   PartName LIKE @P3
                            AND Data1=@P1
                            AND Data2=@P2
                            AND WONumber LIKE '%-%'
                    
                        UNION
                    
                        SELECT
                            PartName,
                            QtyOrdered AS Qty,
                            QtyOrdered AS Nested
                        FROM PartArchive
                        WHERE   PartName LIKE @P3
                            AND Data1=@P1
                            AND Data2=@P2
                            AND WONumber LIKE '%-%'
                    ) AS Part
                    GROUP BY PartName
                ", &[&js.job, &js.ship, &format!("{}_{}", js.job, mark)]
            ).await?
            .into_row().await?;

        if let Some(row) = row {
            {
                let sender = sender.clone();
                let js = js.clone();
                let mark = mark.clone();

                let qty = row.get::<i32, _>("Qty").expect("Qty is None") as u32;
                tokio::spawn(async move { let _ = sender.send( ActorResult::WorkOrder(js, mark, qty) ).await; });
            }

            match row.get::<i32, _>("Nested") {
                Some(1i32) => {
                    tokio::spawn(async move { let _ = sender.send( ActorResult::Dxf(js, mark) ).await; });
                },
                _ => { tokio::spawn(async move { let _ = has_dxf(sender, js, mark).await; }); }
            }
        }

        Ok(())
    }

    pub async fn has_dxf(sender: ActorSender, js: JobShip, mark: Mark) -> Result<(), Error> {
        debug!("Getting imported for {}_{}", js.job, mark);

        let has_dxf = SNDB_POOL
            .get().await    // AsyncOnce
            .get().await?   // Pool
            .query(
                "
                    SELECT 1
                    FROM PartsLibrary
                    WHERE PartName=@P1
                ", &[&format!("{}_{}", js.job, mark)]
            ).await?
            .into_row().await?
            .is_some();

        match has_dxf {
            true  => { tokio::spawn(async move { let _ = sender.send(ActorResult::Dxf(js, mark)).await; }); },
            false => { tokio::spawn(async move { let _ = sender.send(ActorResult::NoDxf).await; }); },
        }

        Ok(())
    }
}

