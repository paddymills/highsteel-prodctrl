
#![warn(missing_docs)]

//! database connections, deserializating and schema

#[macro_use] extern crate serde;

mod db;
pub use db::*;


// ----------------- from old ----------
pub mod bom;
pub mod sn;

mod conn;
pub use conn::*;

mod config;
pub use config::*;

/// Global access to database pools
pub mod pools;

/// Common db types and utils
pub mod prelude {
    use bb8::Pool;
    use bb8_tiberius::ConnectionManager;

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

