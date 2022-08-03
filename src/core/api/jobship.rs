
use regex::Regex;
use std::{
    fmt::{self, Display, Formatter},
    str::FromStr
};

lazy_static! {
    // TODO: move to regex module in core
    static ref JOBSHIP_RE: Regex = Regex::new(r"^(\d{7}[[:alpha:]])-(\d+)$").expect("failed to build regex");
}

/// Job and Shipment
#[derive(Clone, Debug, Default, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct JobShipment {
    /// Job number (with structure letter)
    // TODO: split job into project and structure
    pub job: String,
    /// Shipment number
    // TODO: refactor as number
    // TODO: refactor as renamed 'shipment'
    pub ship: String
}

impl FromStr for JobShipment {
    type Err = std::fmt::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match JOBSHIP_RE.captures(s) {
            Some(cap) => {
                Ok(
                    Self {
                        job: cap.get(1).unwrap().as_str().into(),
                        ship: cap.get(2).unwrap().as_str().into(),
                    }
                )
            },
            None => {
                eprintln!("Failed to parse job-shipment: {}", s);

                // TODO: custom error
                panic!("invalid job-shipment")
            }
        }
    }
}

impl Display for JobShipment {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}-{}", self.job, self.ship)
    }
}
