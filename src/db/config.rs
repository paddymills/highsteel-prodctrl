
use bb8_tiberius::IntoConfig;
use tiberius::{AuthMethod, Config, error::Error};

use crate::config::{Database, CONFIG};


// TODO: move config to each database module

/// Hss database configurations
/// 
/// allows for easy
/// - deserialization from config
/// - conversion into database config
pub enum HssConfig {
    Bom,
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
                let db_cfg = CONFIG.extract_inner::<Database>("bom").unwrap();

                config.host(&db_cfg.server);
            },
            HssConfig::Sigmanest => {
                let db_cfg = CONFIG.extract_inner::<Database>("sigmanest").unwrap();

                config.host(&db_cfg.server);
                config.database(&db_cfg.database.as_ref().unwrap());
            }
        }

        Ok(config)
    }
}
