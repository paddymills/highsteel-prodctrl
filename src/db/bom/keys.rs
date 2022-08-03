
//! Bom database column keys
//! 
//! possible columns (as of 3-Jun-2022):
//! 
//! | Key | Column Name | Type |
//! |---|---|---|
//! | [`MARK`] | Piecemark | BigVarChar |
//! | [`QTY`] | Qty | Intn |
//! | [`COMM`] | Commodity | BigVarChar |
//! | [`DESC`] | Description | BigVarChar |
//! | [`THK`] | Thick | Floatn |
//! | [`WID`] | Width | Floatn |
//! | [`LEN`] | Length | Floatn |
//! | [`SPEC`] | Specification | BigVarChar |
//! | [`GRADE`] | Grade | BigVarChar |
//! | [`TEST`] | ImpactTest | BigVarChar |
//! | [`REMARK`] | Remark | BigVarChar |
//! | [`ITEM`] | Item | BigVarChar |
//! | [`DWG`] | DwgNo | BigVarChar |
//! | [`ANG_THK`] | AngleThickness[^note] | Floatn |
//! 
//! [^note]: thickness of L and HSS shapes only

/// Piecemark
pub const MARK:    &str = "Piecemark";
/// Quantity
pub const QTY:     &str = "Qty";
/// Commodity
pub const COMM:    &str = "Commodity";
/// Description
pub const DESC:    &str = "Description";
/// Thickness
pub const THK:     &str = "Thick";
/// Angular thickness
pub const ANG_THK: &str = "AngleThickness";
/// Width
pub const WID:     &str = "Width";
/// Length
pub const LEN:     &str = "Length";
/// Material Specification
pub const SPEC:    &str = "Specification";
/// Material Grade
pub const GRADE:   &str = "Grade";
/// Material Impact Test
pub const TEST:    &str = "ImpactTest";
/// Remarks
pub const REMARK:  &str = "Remark";
/// Item number
pub const ITEM:    &str = "Item";
/// Drawing
pub const DWG:     &str = "DwgNo";
