
use serde::{Deserialize, Serialize};

/// Confirmation file row
/// 
/// tab delimited row in the format
/// ```tsv
/// {mark}	S-{job}	{part wbs}	{part location: PROD}	{part qty}	{part UoM: EA}	{material master}	{material wbs}	{material qty}	{material UoM: IN2}	{material location}	{plant}	{program}	
/// ```
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all="PascalCase")]
pub struct CnfFileRow {
    pub mark:     String,
    pub job:      String,
    pub part_wbs: String,
    pub part_loc: String,
    pub part_qty: u64,
    pub part_uom: String,

    pub matl:     String,
    pub matl_wbs: Option<String>,
    pub matl_qty: f32,
    pub matl_uom: String,
    pub matl_loc: Option<String>,

    pub plant:    Plant,
    pub program:  String
}

/// Issue file row
/// 
/// ### Text format
/// tab delimited row in the format:
/// ```tsv
/// {code}	{user1}	{user2}	{material master}	{material wbs}	{material qty}	{material UoM: IN2}	{material location}	{plant}	{program}	
/// ```
/// 
/// ### Transaction Codes
/// 
/// | code | SAP transactions | description |
/// |---|---|---|
/// | PR01 | MIGO 221Q | Comsumption for project from project |
/// | PR02 | MIGO 221 | Consumption for project from warehouse |
/// | PR03 | MIGO 311Q + MIGO 221Q | Transfer from project to project And consumption from latter project |
/// | CC01 | MIGO 201 | Consumption for cost center from warehouse |
/// | CC02 | MIGO [transfer from WBS] & 201 | Consumption for cost center from project |
/// 
/// ### User1 and User2 Columns
/// 
/// | code | user1 | user2 |
/// |---|---|---|
/// | PR* | `D-{job}` | Shipment |
/// | CC* | Cost Center | G/L Account[^note] |
/// 
/// [^note]: G/L Account should always be `637118`, unless for a machine project (i.e. CNC table parts)
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all="PascalCase")]
pub struct IssueFileRow {
    pub code: IssueCode,
    pub user1: String,      // Project/Cost Center
    pub user2: String,      // Shipment/GL Account

    pub matl:     String,
    pub matl_wbs: Option<String>,
    pub matl_qty: f32,
    pub matl_uom: String,
    pub matl_loc: Option<String>,

    pub plant:    Plant,
    pub program:  String
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub enum Plant {
    #[serde(rename = "HS01")]
    Lancaster,
    #[serde(rename = "HS02")]
    Williamsport
}


/// 
#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub enum IssueCode {
    #[serde(rename = "PR01")]
    ProjectFromProject,
    #[serde(rename = "PR02")]
    ProjectFromStock,
    #[serde(rename = "PR03")]
    ProjectFromOtherProject,
    #[serde(rename = "CC01")]
    CostCenterFromStock,
    #[serde(rename = "CC02")]
    CostCenterFromProject,
}

impl Into<IssueFileRow> for CnfFileRow {
    fn into(self) -> IssueFileRow {
        // TODO: infer code, user1, user2 from database
        let code = IssueCode::ProjectFromProject;
        let (user1, user2) = ("D-1200211".into(), "01".into());

        IssueFileRow {
            code, user1, user2,

            matl:     self.matl,
            matl_wbs: self.matl_wbs,
            matl_qty: self.matl_qty,
            matl_uom: self.matl_uom,
            matl_loc: self.matl_loc,
            plant:    self.plant,
            program:  self.program
        }
    }
}
