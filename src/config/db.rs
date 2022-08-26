
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

impl Database {
    /// Creates a new database config from a server and database
    pub fn new(server: impl ToString, database: Option<&str>) -> Self {
        Self {
            server: server.to_string(),
            database: database.map(|s| s.to_string()),
            ..Default::default()
        }
    }

    /// builds server name, concatenating the instance is given
    /// 
    /// ```
    /// use prodctrl::config::Database;
    /// 
    /// let mut db = Database::new("servername", None);
    /// 
    /// assert_eq!(db.server_name(), "servername".to_string());
    /// 
    /// db.instance = Some("test_instance".into());
    /// assert_eq!(db.server_name(), "servername\\test_instance".to_string());
    /// ```
    pub fn server_name(&self) -> String {
        match &self.instance {
            Some(instance) => format!("{}\\{}", self.server, instance),
            None => self.server.clone()
        }
    }
}