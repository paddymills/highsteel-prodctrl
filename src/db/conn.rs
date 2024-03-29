
use bb8::Pool;
use bb8_tiberius::{ConnectionManager, IntoConfig};
use tiberius::Client;
use tokio::net::TcpStream;
use tokio_util::compat::TokioAsyncWriteCompatExt;

pub use super::prelude::*;

// TODO: add name to debug

/// Builds a connection pool for a database
pub async fn build_db_pool(config: impl IntoConfig, size: u32) -> DbPool {
    debug!("** init db pool");

    let mgr = match ConnectionManager::build(config) {
        Ok(conn_mgr) => conn_mgr,
        Err(_) => panic!("ConnectionManager failed to connect to database")
    };
    
    debug!("** > db connection Manager built");

    let pool = match Pool::builder()
        .max_size(size)
        .build(mgr)
        .await {
            Ok(pool) => pool,
            Err(_) => panic!("Bom Pool failed to build")
        };
    
    debug!("** > db pool built");

    pool
}

/// Builds a single database connection
pub async fn build_db_conn(config: impl IntoConfig) -> DbClient {
    debug!("** > building db connection");

    let cfg = config.into_config().expect("Failed to convert config");

    let tcp = TcpStream::connect(cfg.get_addr()).await.expect("failed to establish TcpStream");
    tcp.set_nodelay(true).expect("Failed to set no delay for TcpStream");

    Client::connect(cfg, tcp.compat_write()).await.expect("Failed to connect db client")
}
