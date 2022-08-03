
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
    /// Part mark (piecemark)
    pub mark:     String,
    /// Job number (without structure) in the format `S-{job}`
    pub job:      String,
    /// WBS element for part
    pub part_wbs: String,
    /// Location for part (PROD)
    pub part_loc: String,
    /// Part quantity
    pub part_qty: u64,
    /// Part unit of measure (EA)
    pub part_uom: String,

    /// Material master
    pub matl:     String,
    /// Material WBS Element
    pub matl_wbs: Option<String>,
    /// Material quantity
    ///
    /// This is the amount consumed for all parts.
    /// 
    /// `{qty per part} * {part_qty} = {matl_qty}`
    pub matl_qty: f32,
    /// Material unit of measure (IN2, usually)
    pub matl_uom: String,
    /// Material storage location
    pub matl_loc: Option<String>,

    /// Plant for Part and Material
    /// 
    /// If the part is consuming in 1 plant and the material from another,
    /// this should be the plant of the part.
    /// 
    /// Reason being that the part confirmation will fail for the wrong
    /// plant but the material consumption will cause a COGI error,
    /// which can be easily fixed in COGI.
    pub plant:    Plant,
    /// Program number
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
    /// [Transaction code](#transaction-codes)
    pub code: IssueCode,
    /// Project or Cost Center ([User1 Column](#user1-and-user2-columns))
    pub user1: String,
    /// Shipment/GL Account ([User2 Column](#user1-and-user2-columns))
    pub user2: String,

    /// Material master
    pub matl:     String,
    /// Material WBS Element
    pub matl_wbs: Option<String>,
    /// Material quantity
    pub matl_qty: f32,
    /// Material unit of measure
    pub matl_uom: String,
    /// Material storage location
    pub matl_loc: Option<String>,

    /// Material plant
    pub plant:    Plant,
    /// Program number
    pub program:  String
}

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


/// Issue codes
#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub enum IssueCode {
    /// Issue material to the same project
    #[serde(rename = "PR01")]
    ProjectFromProject,
    /// Issue material from stock (no WBS element) to a project
    #[serde(rename = "PR02")]
    ProjectFromStock,
    /// Issue material to a project from a different project
    #[serde(rename = "PR03")]
    ProjectFromOtherProject,
    /// Issue material from stock to a cost center
    #[serde(rename = "CC01")]
    CostCenterFromStock,
    /// Issue material from a project to a cost center
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
