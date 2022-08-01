
use std::fmt::{self, Display, Formatter};
use super::Material;

#[derive(Debug, Default)]
pub struct Part {
    pub mark: String,
    pub qty: i32,

    pub dwg: Option<String>,
    pub desc: Option<String>,
    pub matl: Material,
    
    pub remark: Option<String>
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
