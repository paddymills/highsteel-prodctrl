
use async_once::AsyncOnce;
use bb8::Pool;
use bb8_tiberius::ConnectionManager;

lazy_static!(
    pub static ref BOM_POOL: AsyncOnce<Pool<ConnectionManager>> = AsyncOnce::new( async {
        debug!("****************************************");
        debug!("** init Bom db pool  *******************");
        debug!("****************************************");

        let pool = match Pool::builder()
            .max_size(8)
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

    pub static ref SNDB_POOL: AsyncOnce<Pool<ConnectionManager>> = AsyncOnce::new( async {
        debug!("****************************************");
        debug!("** init Sndb db pool  ******************");
        debug!("****************************************");

        let pool = match Pool::builder()
            .max_size(8)
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