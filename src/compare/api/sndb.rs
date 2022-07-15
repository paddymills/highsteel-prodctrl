
use bb8_tiberius::rt::Client;
use tiberius::Row;

use super::{JobShip, Mark};

#[derive(Debug, Default)]
pub struct QtyAndNested {
    qty: u32,
    nested: bool
}

impl From<&Row> for QtyAndNested {
    fn from(row: &Row) -> Self {
        let qty = row.get::<i32, _>("Qty").unwrap() as u32;
        let nested = match row.get::<i32, _>("Nested").unwrap() {
            0 => false,
            _ => true
        };

        QtyAndNested { qty, nested }
    }
}

#[async_trait]
pub trait PartDbOps {
    async fn get_jobs(&mut self) -> Vec<JobShip>;
    async fn qty_and_nested(&mut self, js: JobShip, mark: Mark) -> QtyAndNested;
    async fn archive_qty_and_nested(&mut self, js: JobShip, mark: Mark) -> QtyAndNested;
    async fn imported(&mut self, js: JobShip, mark: Mark) -> bool;
}

#[async_trait]
impl PartDbOps for Client {
    async fn get_jobs(&mut self) -> Vec<JobShip> {
        debug!("Getting jobs list");

        self
            .simple_query("
                SELECT DISTINCT Data1, Data2
                FROM Part
                WHERE WONumber LIKE '%-%' AND Data1 LIKE '1%'
                GROUP BY Data1, Data2
            ")
            .await?
            .into_first_result().await?
            .into_iter()
            .map(|row| JobShip::from(&row) )
            .collect()
    }

    async fn qty_and_nested(&mut self, js: JobShip, mark: Mark) -> QtyAndNested {
        match self
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
            .into_row().await? {
                Some(row) => row.into(),
                None => self.archive_qty_and_nested(js, mark).await
            }
    }

    async fn archive_qty_and_nested(&mut self, js: JobShip, mark: Mark) -> QtyAndNested {
        match self
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
            .into_row().await? {
                Some(row) => row.into(),
                None => QtyAndNested::default()
            }
    }

    async fn imported(&mut self, js: JobShip, mark: Mark) -> bool {
        self
            .query(
                "
                    SELECT 1
                    FROM PartsLibrary
                    WHERE PartName=@P1
                ", &[&format!("{}_{}", js.job, mark)]
            ).await?
            .into_row().await?
            .is_some()
    }

}
