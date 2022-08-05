
use regex::Regex;

use crate::Plant;
use super::CnfFileRow;

lazy_static! {
    static ref OLD_WBS: Regex = Regex::new(r"S-(\d{7})-2-(\d{2})").expect("Failed to build OLD_WBS Regex");
    static ref JOB_PART: Regex = Regex::new(r"\d{7}[[:alpha:]]-").expect("Failed to build JOB_PART Regex");
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
    /// Convert a [`CnfFileRow`] into an [`IssueFileRow`]
    fn into(self) -> IssueFileRow {
        let (code, user1, user2) = infer_codes(&self);

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

fn infer_codes(row: &CnfFileRow) -> (IssueCode, String, String) {
    match JOB_PART.is_match(&row.mark) {
        // Part name has a job number prefix -> project stock issuing
        true => {
            // infer job and shipment from part WBS element
            let (user1, user2) = match OLD_WBS.captures(&row.part_wbs) {
                Some(caps) => (
                    format!("D-{}", caps.get(1).unwrap().as_str()),
                    caps.get(2).unwrap().as_str().into()
                ),
                None => {
                    // TODO: handle case of D-* wbs
                    error!("failed to parse Part WBS Element: {}", &row.part_wbs);
    
                    panic!("failed to parse Part WBS Element")
                }
            };

            let code = match &row.matl_wbs {
                // project stock material
                Some(wbs) => {
                    // part and material have the same project
                    if wbs.starts_with(&user1) { IssueCode::ProjectFromProject }

                    // part and material have different projects
                    else { IssueCode::ProjectFromOtherProject }
                },

                // plant stock material
                None => IssueCode::ProjectFromStock
            };

            (code, user1, user2)
        },

        // Part name does not have a job number prefix -> cost center issuing
        false => {
            let code = match &row.matl_wbs {
                Some(_) => IssueCode::CostCenterFromProject,
                None => IssueCode::CostCenterFromStock
            };

            // infer cost center and G/L account
            // TODO: fetch cost center from database
            let user1 = "2062".into();

            // TODO: infer if table parts
            // if is_table_parts {
            //     user2 = "634124"
            // }
            let user2 = "637118".into();

            (code, user1, user2)
        }
    }
}
