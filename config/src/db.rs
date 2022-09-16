
use prodctrl_db::Database;
use crate::ConfigAssets;

/// Parent config node
#[derive(Debug, Deserialize, Serialize)]
pub struct Databases {
    /// Bom database configuration
    pub bom: Database,

    /// Sigmanest database configuration
    pub sigmanest: Database
}

impl Databases {
    pub fn from_embed() -> Self {
        ConfigAssets::get("db.toml")
            .map(|asset| toml::from_slice(&asset.data))   
            // TODO: compile time asset existance check
            //       to make sure .unwrap() won't panic
            .unwrap()

            // TODO: compile time deserialization check
    }
}