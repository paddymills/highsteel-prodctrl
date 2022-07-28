
use bb8::Pool;
use bb8_tiberius::ConnectionManager;
use bb8_tiberius::rt::Client;
use tiberius::Row;

use crate::api::Part;

// TODO: refactor and remove
use crate::compare::api::JobShip;


pub mod bom_keys {
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
}

pub async fn init_bom(pool: Pool<ConnectionManager>, job: &str, shipment: i32) -> Result<Vec<Part>, crate::Error> {
    let res = pool.get()
        .await?
        .query(
            "EXEC BOM.SAP.GetBOMData @Job=@P1, @Ship=@P2",
            &[&job, &shipment]
        )
        .await?
        .into_results()
        .await?
        .into_iter()
        .flatten()
        .map( |row| Part::from(row) )
        .collect();

    Ok(res)
}

#[async_trait]
pub trait BomDbOps<T> {
    async fn parts_qty(&mut self, js: &JobShip) -> Vec<T>;
}

#[async_trait]
impl<T> BomDbOps<T> for Client
    where T: From<Row>
{
    async fn parts_qty(&mut self, js: &JobShip) -> Vec<T> {
        debug!("** > Db called parts_qty *********************");
        self
            .query(
                "EXEC BOM.SAP.GetBOMData @Job=@P1, @Ship=@P2",
                &[&js.job, &js.ship]
            )
                .await
                .expect(&format!("Failed to get Bom data: {}", js))
            .into_first_result()
                .await
                .expect(&format!("Failed to get Bom data from results: {}", js))
            .into_iter()

            // >> if using part API
            // .map(|row| Part::from(row))
            // .filter(|part| part.is_pl())
            // .map(|part| PartAndQty::from(part))

            // >> if just using this API
            .filter(|row| row.get::<&str, _>(bom_keys::COMM) == Some("PL"))
            .map(|row| T::from(row))

            .collect()
    }
}
