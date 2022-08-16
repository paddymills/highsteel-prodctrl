
mod cnf_row;
pub use cnf_row::CnfFileRow;

mod issue_row;
pub use issue_row::IssueFileRow;

mod cnf_serde {
    use serde::{self, Serializer};

    pub fn three_digit_f64<S>(val: &f64, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&format!("{:.3}", val))
    }
}