// possible columns (as of 3-Jun-2022):
//     Piecemark      <BigVarChar>
//     Qty            <Intn>
//     Commodity      <BigVarChar>
//     Description    <BigVarChar>
//     Thick          <Floatn>
//     Width          <Floatn>
//     Length         <Floatn>
//     Specification  <BigVarChar>
//     Grade          <BigVarChar>
//     ImpactTest     <BigVarChar>
//     Remark         <BigVarChar>
//     Item           <BigVarChar>
//     DwgNo          <BigVarChar>
//     AngleThickness <Floatn>          (thickness of L and HSS shapes only)

pub const MARK:    &str = "Piecemark";
pub const QTY:     &str = "Qty";
pub const COMM:    &str = "Commodity";
pub const DESC:    &str = "Description";
pub const THK:     &str = "Thick";
pub const ANG_THK: &str = "AngleThickness";
pub const WID:     &str = "Width";
pub const LEN:     &str = "Length";
pub const SPEC:    &str = "Specification";
pub const GRADE:   &str = "Grade";
pub const TEST:    &str = "ImpactTest";
pub const REMARK:  &str = "Remark";
pub const ITEM:    &str = "Item";
pub const DWG:     &str = "DwgNo";
