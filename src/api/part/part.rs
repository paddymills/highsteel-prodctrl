
use std::fmt::{self, Display, Formatter};
use super::Material;
use crate::db::bom::bom_keys;

#[derive(Debug, Default)]
pub struct Part {
    pub mark: String,
    pub qty: i32,

    pub dwg: Option<String>,
    pub desc: Option<String>,
    pub matl: Material,
    
    pub remark: Option<String>
}

impl From<tiberius::Row> for Part {
    fn from(row: tiberius::Row) -> Self {
        Self {
            mark: row.get::<&str, _>(bom_keys::MARK).unwrap_or_default().into(),
            qty:  row.get::<i32, _>(bom_keys::QTY).unwrap_or_default(),

            dwg:  row.get::<&str, _>(bom_keys::DWG).map(Into::into),
            desc: row.get::<&str, _>(bom_keys::DESC).map(Into::into),
            matl: Material::from(&row),

            remark: row.get::<&str, _>(bom_keys::REMARK).map(Into::into),

            ..Default::default()
        }
    }
}

impl Part {
    pub fn is_pl(&self) -> bool {
        self.matl.is_pl()
    }
}

impl Display for Part {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{} ({:?}) [{}]", self.mark, self.matl, self.matl.grade.force_cvn())
    }
}
