
//! database connections, deserializating and schema

mod db;
pub use db::*;

mod conn;
pub use conn::*;

pub mod bom;
pub mod sn;

/// Common db types and utils
pub mod prelude {
    use bb8::Pool;
    use bb8_tiberius::ConnectionManager;

    /// Tiberius Error
    pub type SqlError = tiberius::error::Error;

    /// Convenience export of database Pool type
    pub type DbPool = Pool<ConnectionManager>;

    /// Convenience export of Client type
    /// 
    /// equivalent to
    /// ```
    /// use tiberius::Client;
    /// use tokio_util::compat::Compat;
    /// use tokio::net::TcpStream;
    /// 
    /// type DbClient = Client<Compat<TcpStream>>;
    /// ```
    pub type DbClient = bb8_tiberius::rt::Client;
}

