
use surrealdb::opt::auth;

use super::ConfigAssets;

/// Parent config node
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct DbConfig {
    /// Bom database configuration
    pub bom: DbConnParams,

    /// Sigmanest database configuration
    pub sigmanest: DbConnParams,

    /// ProdCtrl database configuration
    pub prodctrl: DbConnParams
}

/// Database connection
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct DbConnParams {
    /// Server name
    pub server: String,

    /// Server instance, if applicable (namespace for Surreal databases)
    pub instance: Option<String>,
    
    /// Database name (optional)
    pub database: Option<String>,
    
    /// User (optional)
    pub username: Option<String>,
    
    /// Password (if applicable)
    pub password: Option<String>,
    
    /// Default pool size
    pub pool_size: Option<u32>,
}

impl DbConfig {
    /// Creates a database config from embedded toml file
    pub fn from_embed() -> Self {
        ConfigAssets::get("db.toml")
            .map(|asset| toml::from_slice(&asset.data))   
            // TODO: compile time asset existance check
            //       to make sure .unwrap() won't panic
            .unwrap()
            
            // TODO: compile time deserialization check
            .unwrap()

    }

    fn sample() -> Self {
        Self {
            bom: DbConnParams::sample(),
            sigmanest: DbConnParams::sample(),
            prodctrl: DbConnParams::sample()
        }
    }

    /// generates a toml file for embedding at build time
    pub fn generate() {
		// TODO: default with filled in optional fields with "<optional>"
		let cfg_toml = toml::to_string_pretty(&Self::sample())
			.expect("failed to serialize config");
		std::fs::write("assets/db.toml", cfg_toml)
			.expect("failed to write config data to file");

        println!("Database config has been generated. Correct data and build crate.")
	}
}

impl DbConnParams {
    /// Creates a new database config from a server and database
    pub fn new(server: impl ToString, database: Option<&str>) -> Self {
        Self {
            server: server.to_string(),
            database: database.map(|s| s.to_string()),
            ..Default::default()
        }
    }

    fn sample() -> Self {
        Self {
            server: "server".into(),
            instance: Some("<optional>".into()),
            database: Some("<optional>".into()),
            username: Some("<optional>".into()),
            password: Some("<optional>".into()),
            pool_size: Some(8),
        }
    }

    /// generates ['surrealdb::opt::auth::Database`] from database connection parameters
    /// 
    /// ['surrealdb::opt::auth::Database`]: https://docs.rs/surrealdb/latest/surrealdb/opt/auth/struct.Database.html
    pub fn surreal_auth(&self) -> auth::Database {
        let namespace = &self.instance.as_ref().expect("No namespace (instance) supplied for Surreal database");
        let database  = &self.database.as_ref().expect("No database supplied for Surreal database");
        let username  = &self.username.as_ref().expect("No username supplied for Surreal database");
        let password  = &self.password.as_ref().expect("No password supplied for Surreal database");

        auth::Database { namespace, database, username, password }
    }
}
