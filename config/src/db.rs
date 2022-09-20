
use crate::ConfigAssets;

/// Parent config node
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Databases {
    /// Bom database configuration
    pub bom: Database,

    /// Sigmanest database configuration
    pub sigmanest: Database
}

/// Database connection
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Database {
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
}

impl Databases {
    pub fn from_embed() -> Self {
        ConfigAssets::get("db.toml")
            .map(|asset| toml::from_slice(&asset.data))   
            // TODO: compile time asset existance check
            //       to make sure .unwrap() won't panic
            .unwrap()
            .unwrap()

            // TODO: compile time deserialization check
    }
    
    pub fn generate() {
		let cfg = Self::default();
		let cfg_toml = toml::to_string(&cfg)
			.expect("failed to serialize config");
		std::fs::write("assets/config.toml", cfg_toml)
			.expect("failed to write config data to file");
	}
}