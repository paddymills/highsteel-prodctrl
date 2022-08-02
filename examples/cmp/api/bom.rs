
use super::Mark;
use prodctrl::api::Part;
use prodctrl::db::bom::keys;

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
            mark: row.get::<&str, _>(keys::MARK).unwrap_or_default().into(),
            qty:  row.get::<i32, _>(keys::QTY).unwrap_or_default() as u32
        }
    }
}
