
use bb8_tiberius::IntoConfig;
use tiberius::{AuthMethod, Config, error::Error};

use crate::config::CONFIG;


// TODO: move config to each database module

/// Hss database configurations
/// 
/// allows for easy
/// - deserialization from config
/// - conversion into database config
pub enum HssConfig {
    /// Bom database
    Bom,
    /// Sigmanest database
    Sigmanest
}

impl IntoConfig for HssConfig {
    fn into_config(self) -> Result<Config, Error> {
        let mut config = Config::new();

        // use windows authentication
        config.authentication(AuthMethod::Integrated);
        config.trust_cert();

        match self {
            HssConfig::Bom => {
                config.host(&CONFIG.bom.server_name());
            },
            HssConfig::Sigmanest => {
                config.host(&CONFIG.sigmanest.server_name());
                config.database(&CONFIG.sigmanest.database.as_ref().unwrap());
            }
        }

        Ok(config)
    }
}
