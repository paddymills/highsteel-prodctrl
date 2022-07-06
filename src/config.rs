
#[allow(unused_imports)]
use figment::{
    Error,
    Figment,
    providers::{Env, Format, Serialized, Toml}
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub bom: Database,

    #[serde(alias = "sndb")]
    pub sigmanest: Database
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Database {
    #[serde(alias = "host")]
    pub server: String,

    #[serde(alias = "db")]
    pub database: Option<String>, 

    #[serde(alias = "uid")]
    pub user: Option<String>,

    #[serde(alias = "pwd")]
    pub password: Option<String>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            bom: Database {
                server: "HSSSQLSERV".into(),
                ..Default::default()
            },
            sigmanest: Database {
                server: "hiiwinbl18".into(),
                database: Some("SNDBase91".into()),
                ..Default::default()
            }
        }

        // Self {
        //     bom: Database { server: "[ENG]".into(), ..Default::default() },
        //     sigmanest: Database { server: "[SNDB]".into(), db: Some("[SNDB.db]".into()), ..Default::default() }
        // }
    }
}

pub fn read_config() -> Figment {

    Figment::from(Serialized::defaults(Config::default()))
        // cannot use env until figment fixes duplicate key error
        // .merge(
        //     // Env::prefixed strips the prefix, which we need
        //     Env::raw()
        //         .filter(|k| k.starts_with("SNDB_"))
        //         .split("_")
        // )
        .merge(Toml::file("config.toml"))
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
    
            let config = read_config();
            println!("{:#?}", config);

            println!("{:#?}", config.extract::<Config>());
    
            assert_eq!(config.extract_inner::<Database>("bom")?.server, "HSSSQLSERV".to_string());
            // assert_eq!(config.extract_inner::<Database>("sigmanest")?.user, Some("SNUser".to_string()));

            Ok(())
        });
    }
}
