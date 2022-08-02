
use bb8::Pool;
use bb8_tiberius::{ConnectionManager, IntoConfig};
use tiberius::Client;
use tokio::net::TcpStream;
use tokio_util::compat::{Compat, TokioAsyncWriteCompatExt};

/// Builds a connection pool for a database
pub async fn build_db_pool(name: String, config: impl IntoConfig, size: u32) -> Pool<ConnectionManager> {
    // TODO: dynamic debug padding for name
    debug!("****************************************");
    debug!("** init db pool for {} *******************", name);
    debug!("****************************************");

    let mgr = match ConnectionManager::build(config) {
        Ok(conn_mgr) => conn_mgr,
        Err(_) => panic!("ConnectionManager failed to connect to database")
    };
    
    debug!("** > {} db connection Manager built", name);

    let pool = match Pool::builder()
        .max_size(size)
        .build(mgr)
        .await {
            Ok(pool) => pool,
            Err(_) => panic!("Bom Pool failed to build")
        };
    
    debug!("** > {} db pool built", name);

    pool
}

/// Builds a single database connection
pub async fn build_db_conn(name: String, config: impl IntoConfig) -> Client<Compat<TcpStream>> {
    debug!("** > building {} db connection", name);

    let cfg = config.into_config().expect("Failed to convert config");

    let tcp = TcpStream::connect(cfg.get_addr()).await.expect("failed to establish TcpStream");
    tcp.set_nodelay(true).expect("Failed to set no delay for TcpStream");

    Client::connect(cfg, tcp.compat_write()).await.expect("Failed to connect db client")
}
