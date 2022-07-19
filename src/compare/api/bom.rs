
use bb8_tiberius::rt::Client;
use super::{JobShip, Mark};
use crate::part::{bom_keys, Part};

pub struct PartAndQty {
    pub mark: Mark,
    pub qty: u32
}

impl From<Part> for PartAndQty {
    fn from(part: Part) -> Self {
        Self {
            mark: part.mark,
            qty: part.qty as u32
        }
    }
}

impl From<tiberius::Row> for PartAndQty {
    fn from(row: tiberius::Row) -> Self {
        Self {
            mark: row.get::<&str, _>(bom_keys::MARK).unwrap_or_default().into(),
            qty:  row.get::<i32, _>(bom_keys::QTY).unwrap_or_default() as u32
        }
    }
}

#[async_trait]
pub trait BomDbOps {
    async fn parts_qty(&mut self, js: &JobShip) -> Vec<PartAndQty>;
}

#[async_trait]
impl BomDbOps for Client {
    async fn parts_qty(&mut self, js: &JobShip) -> Vec<PartAndQty> {
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
            .map(|row| PartAndQty::from(row))

            .collect()
    }
}