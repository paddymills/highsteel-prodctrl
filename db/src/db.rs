
use bb8_tiberius::IntoConfig;
use tiberius::{AuthMethod, Config, error::Error};

use prodctrl_config::{DbConfig, DbConnParams};
use super::prelude::*;

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
    /// uses pool size from config or default of 2
    /// 
    /// ['bb8::Pool`]: https://docs.rs/bb8/latest/bb8/struct.Pool.html
    pub async fn build_pool(self) -> DbPool {
        // get pool size from config
        let size = self.get_config().pool_size.unwrap_or(2u32);

        self.build_pool_sized(size).await
    }

    /// Builds a pool of a given size
    pub async fn build_pool_sized(self, size: u32) -> DbPool {
        super::build_db_pool(self.get_name(), self, size).await
    }

    /// Connects to Sigmanest database and returns a [`tiberius::Client`]
    /// 
    /// [`tiberius::Client`]: https://docs.rs/tiberius/latest/tiberius/struct.Client.html
    pub async fn connect(self) -> DbClient {
        super::build_db_conn(self.get_name(), self).await
    }

    fn get_name(&self) -> &str {
        match self {
            Self::Bom => "Bom",
            Self::Sigmanest => "Sigmanest",
        }
    }

    fn get_config(&self) -> DbConnParams {
        match self {
            Self::Bom => DbConfig::from_embed().bom,
            Self::Sigmanest => DbConfig::from_embed().sigmanest,
        }
    }
}

impl IntoConfig for HssDatabase {
    fn into_config(self) -> Result<Config, Error> {
        let mut config = Config::new();

        // use windows authentication
        config.authentication(AuthMethod::Integrated);
        config.trust_cert();

        let cfg = self.get_config();
        config.host(&cfg.server);

        if let Some(inst) = cfg.instance {
            config.instance_name(inst);
        }

        match self {
            HssDatabase::Sigmanest => {
                config.database(&cfg.database.as_ref().unwrap());
            },
            _ => ()
        }

        Ok(config)
    }
}
