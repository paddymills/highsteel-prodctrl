
// TODO: move to bin

// TODO: migrate use of part/matl data to separate structs and flatten with serde
//  current cannot do with csv crate: https://github.com/BurntSushi/rust-csv/issues/98

// TODO: refactor use of String to Box<str> in fixed length strings (wbs, plant, etc)
//  saves 8 bytes of memory since length and capacity do not need to be tracked
//  see: https://mahdi.blog/rust-box-str-vs-string/

mod cnf_row;
use std::path::PathBuf;

pub use cnf_row::CnfFileRow;

mod issue_row;
pub use issue_row::{IssueFileRow, InferCodesError};

#[derive(Debug, Serialize, Deserialize)]
pub struct CnfLogRecord {
    pub filename: String,
    pub record: CnfFileRow
}

impl CnfLogRecord {
    pub fn new(record: &CnfFileRow, filename: &PathBuf) -> Self {
        Self {
            filename: filename.file_stem().unwrap_or_default().to_str().unwrap().into(),
            record: record.clone()
        }
    }
}

mod cnf_serde {
    use serde::{self, Serializer};

    pub fn three_digit_f64<S>(val: &f64, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&format!("{:.3}", val))
    }
}