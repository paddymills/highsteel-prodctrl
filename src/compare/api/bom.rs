
use bb8_tiberius::rt::Client;
use super::JobShip;
use crate::part::Part;

pub struct PartAndQty {
    pub mark: String,
    pub qty: u32
}

#[async_trait]
pub trait BomDbOps {
    async fn parts_qty(&mut self, js: JobShip) -> Vec<PartAndQty>;
}

#[async_trait]
impl BomDbOps for Client {
    async fn parts_qty(&mut self, js: JobShip) -> Vec<PartAndQty> {
        self
            .query(
                "EXEC BOM.SAP.GetBOMData @Job=@P1, @Ship=@P2",
                &[&js.job, &js.ship]
            )
            .await?
            .into_first_result().await?
            .into_iter()
            .map(|row| Part::from(row))
            .filter(|part| part.is_pl())
            .map(|part| PartAndQty { mark: part.mark, qty: part.qty as u32 })
            .collect()
    }
}