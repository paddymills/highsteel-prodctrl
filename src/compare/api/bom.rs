
use bb8_tiberius::rt::Client;
use super::{JobShip, Mark};
use crate::part::Part;

pub struct PartAndQty {
    pub mark: Mark,
    pub qty: u32
}

impl From<Part> for PartAndQty {
    fn from(part: Part) -> Self {
        PartAndQty {
            mark: part.mark,
            qty: part.qty as u32
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
            .map(|row| Part::from(row))
            .filter(|part| part.is_pl())
            .map(|part| PartAndQty::from(part))
            .collect()
    }
}