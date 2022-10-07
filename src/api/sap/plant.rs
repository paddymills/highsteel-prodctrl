
/// SAP Plant
#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub enum Plant {
    /// Lancaster (HS01)
    #[serde(rename = "HS01")]
    Lancaster,
    /// Williamsport (HS02)
    #[serde(rename = "HS02")]
    Williamsport
}
