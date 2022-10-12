
use super::ConfigAssets;

/// Parent config node
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct DbConfig {
    /// Bom database configuration
    pub bom: DbConnParams,

    /// Sigmanest database configuration
    pub sigmanest: DbConnParams
}

/// Database connection
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct DbConnParams {
    /// Server name
    pub server: String,

    /// Server instance, if applicable
    pub instance: Option<String>,
    
    /// Database name (optional)
    pub database: Option<String>,
    
    /// User (optional)
    pub user: Option<String>,
    
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
            sigmanest: DbConnParams::sample()
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
            user: Some("<optional>".into()),
            password: Some("<optional>".into()),
            pool_size: Some(8),
        }
    }
}