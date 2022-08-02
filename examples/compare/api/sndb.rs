
use bb8_tiberius::rt::Client;
use tiberius::Row;

use super::{JobShipment, Mark};

#[derive(Debug, Default)]
pub struct QtyAndNested {
    pub qty: u32,
    pub nested: bool
}

impl From<&Row> for QtyAndNested {
    fn from(row: &Row) -> Self {
        if let None = row.get::<i32, _>("Qty") {
            error!("Got no Qty for {:?}", row);
        }

        if let None = row.get::<i32, _>("Nested") {
            error!("Got no Nested for {:?}", row);
        }

        let qty = row.get::<i32, _>("Qty").unwrap() as u32;
        let nested = match row.get::<i32, _>("Nested").unwrap() {
            0 => false,
            _ => true
        };

        QtyAndNested { qty, nested }
    }
}

#[async_trait]
pub trait SnDbOps {
    async fn get_jobs(&mut self) -> Vec<JobShipment>;
    async fn qty_and_nested(&mut self, js: &JobShipment, mark: &Mark) -> QtyAndNested;
    async fn archive_qty_and_nested(&mut self, js: &JobShipment, mark: &Mark) -> QtyAndNested;
    async fn imported(&mut self, js: &JobShipment, mark: &Mark) -> bool;
}

#[async_trait]
impl SnDbOps for Client {
    async fn get_jobs(&mut self) -> Vec<JobShipment> {
        debug!("Getting jobs list");

        self
            .simple_query("
                SELECT DISTINCT Data1, Data2
                FROM Part
                WHERE WONumber LIKE '%-%' AND Data1 LIKE '1%'
                GROUP BY Data1, Data2
            ")
            .await.expect("failed to query for jobs")
            .into_first_result().await.expect("failed to get jobs list from results")
            .into_iter()
            .map(|row| JobShipment::from(&row) )
            .collect()
    }

    async fn qty_and_nested(&mut self, js: &JobShipment, mark: &Mark) -> QtyAndNested {
        match async {
            self
                .query(
                "
                    SELECT
                        SUM(QtyOrdered) AS Qty,
                        SUM(QtyCompleted) AS Nested
                    FROM Part
                    WHERE   PartName LIKE @P3
                        AND Data1=@P1
                        AND Data2=@P2
                        AND WONumber LIKE '%-%'
                    GROUP BY PartName
                ", &[&js.job, &js.ship, &format!("{}_{}", js.job, mark)]
                ).await?
                .into_row().await
            }.await {
                Ok(Some(row)) => QtyAndNested::from(&row),
                _             => self.archive_qty_and_nested(js, mark).await
            }
    }

    async fn archive_qty_and_nested(&mut self, js: &JobShipment, mark: &Mark) -> QtyAndNested {
        match async {
            self
                .query(
                "
                    SELECT
                        SUM(QtyOrdered) AS Qty,
                        SUM(QtyProgram) AS Nested
                    FROM PartArchive
                    WHERE   PartName LIKE @P3
                        AND Data1=@P1
                        AND Data2=@P2
                        AND WONumber LIKE '%-%'
                    GROUP BY PartName
                ", &[&js.job, &js.ship, &format!("{}_{}", js.job, mark)]
                ).await?
                .into_row().await
            }.await {
                Ok(Some(row)) => QtyAndNested::from(&row),
                _             => QtyAndNested::default()
            }
    }

    async fn imported(&mut self, js: &JobShipment, mark: &Mark) -> bool {
        if let Ok(Some(_)) = async {
            self
                .query(
                    "
                        SELECT 1
                        FROM PartsLibrary
                        WHERE PartName=@P1
                    ", &[&format!("{}_{}", js.job, mark)]
                )
                .await?
                .into_row()
                .await
            }.await
        {  return true }

        false
    }

}
