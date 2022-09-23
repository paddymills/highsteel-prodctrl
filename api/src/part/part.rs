

use std::fmt::{self, Display, Formatter};
use super::Material;

/// Part (piecemark)
#[derive(Debug, Default)]
pub struct Part {
    /// Piecemark
    pub mark: String,
    /// Quantity
    pub qty: i32,

    /// Drawing name
    pub dwg: Option<String>,
    /// Description
    pub desc: Option<String>,
    /// Geometry information
    pub matl: Material,
    
    /// Additional remarks
    pub remark: Option<String>
}

impl Part {
    /// Create a new part from a given mark
    pub fn new(mark: String) -> Self {
        Self { mark, ..Default::default() }
    }

    /// Is part a plate
    /// Re-elevated from [`Material`]
    /// 
    /// ['Material']: crate::Material
    pub fn is_pl(&self) -> bool {
        self.matl.is_pl()
    }
}

impl Display for Part {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{} ({:?}) [{}]", self.mark, self.matl, self.matl.grade.force_cvn())
    }
}
