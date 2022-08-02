
use std::fmt::{self, Display, Formatter};
use super::{Commodity, Grade};

/// Material representation for geometry
#[derive(Debug, Default)]
pub struct Material {
    pub comm: Commodity,
    pub grade: Grade,
    pub len: f32
}

impl Material {
    pub fn new_pl(thk: f32, wid: f32, len: f32) -> Self{
        Self {
            comm: Commodity::Plate { thk, wid },
            len,

            ..Default::default()
        }
    }

    pub fn new_shape(thk: f32, section: String, len: f32) -> Self{
        Self {
            comm: Commodity::Shape { thk, section },
            len,

            ..Default::default()
        }
    }

    pub fn can_punch(&self, min_punch_thk: f32) -> bool {
        match self.comm {
            Commodity::Plate { thk, .. } => thk >= min_punch_thk,
            Commodity::Shape { thk, .. } => thk >= min_punch_thk,
            Commodity::Skip(_)           => false
        }
    }

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
