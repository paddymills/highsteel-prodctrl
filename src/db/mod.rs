
//! database connections, deserializating and schema

pub mod bom;
pub mod sn;

mod conn;
pub use conn::*;

mod config;
pub use config::*;

/// Common db types and utils
pub mod prelude {
    use bb8::Pool;
    use bb8_tiberius::ConnectionManager;
    use tiberius::Client;
    use tokio_util::compat::Compat;
    use tokio::net::TcpStream;

    /// Convenience export of database Pool type
    pub type DbPool = Pool<ConnectionManager>;
    /// Convenience export of Client type 
    pub type DbClient = Client<Compat<TcpStream>>;
}
