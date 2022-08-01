
use serde::{Deserialize, Serialize};

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
