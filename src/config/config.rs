
use figment::{
    Figment,
    providers::{Format, Serialized, Toml}
};

use super::Database;

/// Parent config node
#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    /// Bom database configuration
    pub bom: Database,

    /// Sigmanest database configuration
    pub sigmanest: Database
}

impl Default for Config {
    fn default() -> Self {
        Self {
            bom: Database::new("HSSSQLSERV", None),
            sigmanest: Database::new("hiiwinbl18", Some("SNDBase91"))
        }
    }
}

impl Config {
    /// init config
    pub fn read_config() -> Self {
        Figment::from(Serialized::defaults(Config::default()))
            .merge(Toml::file(r"test\config.toml"))
            .extract()
            .expect("Failed to extract config")
    }
}


#[cfg(test)]
mod config_tests {

    use super::*;

    #[test]
    fn test_config() {
        figment::Jail::expect_with(|jail| {
            jail.set_env("SNDB_HOST", "hiiwinbl5");
            jail.set_env("SNDB_DB", "SNDataDev");
            jail.set_env("SNDB_USER", "SNUser");
    
            jail.create_file("config.toml", r#"
                [bom]
                server = "HSSSQLSERV"
                
                [sigmanest]
                server = "hiiwinbl18"
                database = "SNDBase91"
            "#)?;
    
            let config = Config::read_config();
            println!("{:#?}", config);
    
            assert_eq!(config.bom.server_name(), "HSSSQLSERV".to_string());
            // assert_eq!(config.extract_inner::<Database>("sigmanest")?.user, Some("SNUser".to_string()));

            Ok(())
        });
    }
}
