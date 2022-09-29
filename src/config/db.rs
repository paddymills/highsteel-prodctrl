
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

    /// generates a toml file for embedding at build time
    // TODO: move this to build script
    pub fn generate() {
		let cfg = Self::default();
		let cfg_toml = toml::to_string(&cfg)
			.expect("failed to serialize config");
		std::fs::write("assets/db.toml", cfg_toml)
			.expect("failed to write config data to file");
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
}