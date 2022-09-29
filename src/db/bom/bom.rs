
use tiberius::Row;

use super::api_compat::*;

use super::keys;
use crate::prelude::*;


/// Trait to add Bom db operations to database Client
#[async_trait]
pub trait BomDbOps<T>
    where T: From<Row>
{
    /// Builds a list of [`Parts`] from the database for a given [`JobShipment`]
    /// 
    /// [`Parts`]: crate::Part
    /// [`JobShipment`]: crate::JobShipment
    // TODO: refactor job and shipment to a JobShipment
    async fn init_bom(&mut self, job: &str, shipment: i32) -> Result<Vec<T>, SqlError>;

    /// Gets all parts and their quantities or a given [`JobShipment`]
    async fn parts_qty(&mut self, js: &JobShipment) -> Vec<T>;
}

#[async_trait]
impl<T> BomDbOps<T> for DbClient
    where T: From<Row>
{
    async fn init_bom(&mut self, job: &str, shipment: i32) -> Result<Vec<T>, SqlError> {
        let res = self
            .query(
                "EXEC BOM.SAP.GetBOMData @Job=@P1, @Ship=@P2",
                &[&job, &shipment]
            )
            .await?
            .into_first_result()
            .await?
            .into_iter()
            .map( |row| T::from(row) )
            .collect();
    
        Ok(res)
    }

    async fn parts_qty(&mut self, js: &JobShipment) -> Vec<T> {
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
            .filter(|row| row.get::<&str, _>(keys::COMM) == Some("PL"))
            .map(|row| T::from(row))

            .collect()
    }
}
