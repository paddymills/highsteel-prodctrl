
use crate::prelude::Part;

/// For comparing an existing (in work order) part with data changes
struct PartCompare {
    existing: Option<Part>,
    new: Part
}
