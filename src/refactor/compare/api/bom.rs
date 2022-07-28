
use super::Mark;
use crate::api::Part;
use crate::db::bom::bom_keys;

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
