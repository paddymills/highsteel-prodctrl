
pub mod bom_db {
    use async_once::AsyncOnce;
    use bb8::Pool;
    use bb8_tiberius::ConnectionManager;
    
    use super::super::*;
    use crate::{Error, part::Part};
    
    const POOL_SIZE: u32 = ACTORS as u32;
    
    lazy_static!(
        static ref BOM_POOL: AsyncOnce<Pool<ConnectionManager>> = AsyncOnce::new( async {
            debug!("****************************************");
            debug!("** init Bom db pool  *******************");
            debug!("****************************************");
    
            let pool = match Pool::builder()
                .max_size(POOL_SIZE)
                .build(
                    match ConnectionManager::build(crate::db::HssConfig::Bom) {
                        Ok(conn_mgr) => conn_mgr,
                        Err(_) => panic!("ConnectionManager failed to connect to database")
                    }
                )
                .await {
                    Ok(pool) => pool,
                    Err(_) => panic!("Bom Pool failed to build")
                };
        
            pool    
        });
    );

    pub async fn get_parts(sender: ActorSender, js: JobShip) -> Result<(), Error> {
        debug!("Getting parts for {}", js);

        // let mut results: Vec<ActorResult> = BOM_POOL
        //     .get().await    // AsyncOnce
        //     .get().await?   // Pool
        //     .query(
        //         "EXEC BOM.SAP.GetBOMData @Job=@P1, @Ship=@P2",
        //         &[&js.job, &js.ship]
        //     )
        //     .await?
        //     .into_first_result().await?
        //     .into_iter()
        //     .map(|row| Part::from(row))
        //     .filter(|part| part.is_pl())
        //     .map(|part| ActorResult::Bom(js.clone(), part.mark, part.qty as Qty) )
        //     .collect();

        let iter = BOM_POOL
            .get().await    // AsyncOnce
            .get().await?   // Pool
            .query(
                "EXEC BOM.SAP.GetBOMData @Job=@P1, @Ship=@P2",
                &[&js.job, &js.ship]
            )
            .await?
            .into_first_result().await?
            .into_iter()
            .map(|row| Part::from(row))
            .filter(|part| part.is_pl());

        for part in iter {
            let sender = sender.clone();
            let res = ActorResult::Bom(js.clone(), part.mark, part.qty as Qty);

            tokio::spawn(async move { let _ = sender.send( res ).await; });
        }

        tokio::spawn(async move { let _ = sender.send( ActorResult::JobProcessed(js.clone()) ).await; });

        Ok(())
    }
}
