
use tiberius::Row;

use crate::{Commodity, Grade, Material, Part};
use crate::JobShipment;

use super::{keys, super::prelude::*};


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
    async fn init_bom(&mut self, job: &str, shipment: i32) -> Result<Vec<T>, crate::Error>;

    /// Gets all parts and their quantities or a given [`JobShipment`]
    async fn parts_qty(&mut self, js: &JobShipment) -> Vec<T>;
}

#[async_trait]
impl<T> BomDbOps<T> for DbClient
    where T: From<Row>
{
    async fn init_bom(&mut self, job: &str, shipment: i32) -> Result<Vec<T>, crate::Error> {
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

impl From<Row> for Part {
    fn from(row: Row) -> Self {
        Self::from(&row)
    }
}

impl From<&Row> for Part {
    fn from(row: &Row) -> Self {
        Self {
            mark: row.get::<&str, _>(keys::MARK).unwrap_or_default().into(),
            qty:  row.get::<i32, _>(keys::QTY).unwrap_or_default(),

            dwg:  row.get::<&str, _>(keys::DWG).map(Into::into),
            desc: row.get::<&str, _>(keys::DESC).map(Into::into),
            matl: Material::from(row),

            remark: row.get::<&str, _>(keys::REMARK).map(Into::into),

            ..Default::default()
        }
    }
}

impl From<&Row> for Material {
    fn from(row: &Row) -> Self {
        let len = row.get::<f32, _>(keys::LEN).unwrap_or_default();
        let grade = Grade::from(row);

        let comm = match row.get::<&str, _>(keys::COMM).unwrap_or_default() {
            "PL" => Commodity::Plate {
                thk: row.get::<f32, _>(keys::THK).unwrap_or_default(),
                wid: row.get::<f32, _>(keys::WID).unwrap_or_default()
            },
            
            "L" | "HSS" => Commodity::Shape {
                thk: row.get::<f32, _>(keys::ANG_THK).unwrap_or_default(),
                section: row.get::<&str, _>(keys::DESC).unwrap_or_default().into()
            },

            "MC" | "C" | "W" | "WT" => Commodity::Shape {
                // TODO: AISC shape db thickness
                thk: row.get::<f32, _>(keys::THK).unwrap_or_default(),
                section: row.get::<&str, _>(keys::DESC).unwrap_or_default().into()
            },
            
            _ => Commodity::Skip(
                row.get::<&str, _>(keys::DESC).unwrap_or_default().into()
            )
        };

        Self { comm, grade, len }
    }
}

impl From<&Row> for Grade {
    fn from(row: &Row) -> Self {
        Self::new(
            row.get::<&str, _>(keys::SPEC ).expect("Failed to get spec for Grade"),
            row.get::<&str, _>(keys::GRADE).expect("Failed to get grade for Grade"),
            row.get::<&str, _>(keys::TEST ).expect("Failed to get test for Grade"),
            0
        )
    }
}
