
use async_once::AsyncOnce;
use bb8::Pool;
use bb8_tiberius::ConnectionManager;
use lazy_static::lazy_static;
use log::debug;

const BOM_POOL_SIZE: u32 = 8;
const SNDB_POOL_SIZE: u32 = 32;

lazy_static!(
    static ref BOM_POOL: AsyncOnce<Pool<ConnectionManager>> = AsyncOnce::new( async {
        debug!("****************************************");
        debug!("** init Bom db pool  *******************");
        debug!("****************************************");

        let mgr = match ConnectionManager::build(prodctrl::db::HssConfig::Bom) {
            Ok(conn_mgr) => conn_mgr,
            Err(_) => panic!("ConnectionManager failed to connect to database")
        };
        
        debug!("** > Connection Manager built **********");

        let pool = match Pool::builder()
            .max_size(BOM_POOL_SIZE)
            .build(mgr)
            .await {
                Ok(pool) => pool,
                Err(_) => panic!("Bom Pool failed to build")
            };
        
        debug!("** > Db pool built *********************");

        pool
    });
);

lazy_static!(
    static ref SNDB_POOL: AsyncOnce<Pool<ConnectionManager>> = AsyncOnce::new( async {
        debug!("****************************************");
        debug!("** init Sndb db pool  ******************");
        debug!("****************************************");

        let mgr = match ConnectionManager::build(prodctrl::db::HssConfig::Sigmanest) {
            Ok(conn_mgr) => conn_mgr,
            Err(_) => panic!("ConnectionManager failed to connect to database")
        };
        
        debug!("** > Connection Manager built **********");

        let pool = match Pool::builder()
            .max_size(SNDB_POOL_SIZE)
            .build(mgr)
            .await {
                Ok(pool) => pool,
                Err(_) => panic!("Sndb Pool failed to build")
            };
        debug!("** > Db pool built *********************");

        pool
    });
);

pub async fn get_bom_pool() -> Pool<ConnectionManager> {
    let pool = BOM_POOL.get().await;

    debug!("cloning bom pool");

    pool.clone()
}

pub async fn get_sndb_pool() -> Pool<ConnectionManager> {
    let pool = SNDB_POOL.get().await;

    debug!("cloning sndb pool");

    pool.clone()
}
