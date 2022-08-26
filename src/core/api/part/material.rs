
use std::fmt::{self, Display, Formatter};
use super::{Commodity, Grade};

/// Material representation for geometry
#[derive(Debug, Default)]
pub struct Material {
    /// Plate or shape type
    pub comm: Commodity,
    /// Material grade
    pub grade: Grade,
    /// Material length
    pub len: f32
}

impl Material {
    /// Create a new plate from thickness, width and length
    pub fn new_pl(thk: f32, wid: f32, len: f32) -> Self{
        Self {
            comm: Commodity::Plate { thk, wid },
            len,

            ..Default::default()
        }
    }

    /// Create a new shape from thickness, section name and length
    pub fn new_shape(thk: f32, section: String, len: f32) -> Self{
        Self {
            comm: Commodity::Shape { thk, section },
            len,

            ..Default::default()
        }
    }

    /// Infer if [`Commodity`] can punch based on minimal punch thickness
    /// 
    /// [`Commodity`]: crate::Commodity
    // TODO: infer punch requirements from Engineering (if possible)
    pub fn can_punch(&self, min_punch_thk: f32) -> bool {
        match self.comm {
            Commodity::Plate { thk, .. } => thk >= min_punch_thk,
            Commodity::Shape { thk, .. } => thk >= min_punch_thk,
            Commodity::Skip(_)           => false
        }
    }

    /// Tells if [`Commodity`] is a plate.
    /// 
    /// Useful for filtering items for Sigmanest
    /// 
    /// [`Commodity`]: crate::Commodity
    pub fn is_pl(&self) -> bool {
        match self.comm {
            Commodity::Plate { .. } => true,
            _                       => false,
        }
    }

}

impl Display for Material {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:} x {}", &self.comm, self.len)
    }
}
