
use bb8_tiberius::IntoConfig;
use tiberius::{AuthMethod, Config, error::Error};

use prodctrl::config::Databases;
use super::prelude::*;

// TODO: move config to each database module

/// Hss database configurations
/// 
/// allows for easy
/// - deserialization from config
/// - conversion into database config
pub enum HssDatabase {
    /// Bom database
    Bom,
    /// Sigmanest database
    Sigmanest
}

impl HssDatabase {
    /// Builds a ['bb8::Pool`] for the Sigmanest database
    /// 
    /// ['bb8::Pool`]: https://docs.rs/bb8/latest/bb8/struct.Pool.html
    pub async fn build_pool(self) -> DbPool {
        let (name, size) = match &self {
            Self::Bom => ("Bom", 2u32),
            Self::Sigmanest => ("Sigmanest", 16u32),
        };

        super::build_db_pool(name, self, size).await
    }

    /// Connects to Sigmanest database and returns a [`tiberius::Client`]
    /// 
    /// [`tiberius::Client`]: https://docs.rs/tiberius/latest/tiberius/struct.Client.html
    pub async fn connect(self) -> DbClient {
        let name = match &self {
            Self::Bom => "Bom",
            Self::Sigmanest => "Sigmanest",
        };

        super::build_db_conn(name, self).await
    }
}

impl IntoConfig for HssDatabase {
    fn into_config(self) -> Result<Config, Error> {
        let mut config = Config::new();

        // use windows authentication
        config.authentication(AuthMethod::Integrated);
        config.trust_cert();

        let cfg = Databases::from_embed();
        match self {
            HssDatabase::Bom => {
                config.host(&cfg.bom.server);
            },
            HssDatabase::Sigmanest => {
                config.host(&cfg.sigmanest.server);
                config.database(&cfg.sigmanest.database.as_ref().unwrap());
            }
        }

        Ok(config)
    }
}
