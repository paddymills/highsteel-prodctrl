
use std::fmt::{self, Display, Formatter};

/// material type (plate, shape, etc.)
#[derive(Debug)]
pub enum Commodity {
    Plate { thk: f32, wid: f32 },

    // TODO: refactor section to use AISC db
    Shape { thk: f32, section: String },
    Skip(String)
}

impl Default for Commodity {
    fn default() -> Self {
        Commodity::Skip(Default::default())
    }
}

impl Display for Commodity {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match &self {
            Commodity::Plate { thk, wid }    => write!(f, "PL {} x {}", thk, wid),
            Commodity::Shape { section, .. } => write!(f, "{}", section),
            Commodity::Skip(desc)            => write!(f, "UNMATCHED SECTION {:}", desc)
        }
        
    }
}
