
use std::fmt::{self, Display, Formatter};

/// material type (plate, shape, etc.)
#[derive(Debug)]
pub enum Commodity {
    /// Flat plate
    Plate {
        /// Plate thickness
        thk: f32,
        /// Plate width
        wid: f32
    },

    // TODO: refactor section to use AISC db
    /// Structural shape, represented by thickness and section name
    Shape {
        /// Shape thickness (for punching mainly)
        thk: f32,
        /// Shape section name
        section: String
    },

    /// Miscellaneous commodities that are not currently of interest (nuts, bolts, etc.)
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
