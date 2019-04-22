//! Data integrity errors

use std::error::Error as StdError;
use std::fmt::{Display, Formatter, Result};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum Error {
    FailedRead,
    FailedWrite,
    FailedEncode,
    FailedDecode,
}

impl StdError for Error {}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(
            f,
            "{}",
            match self {
                _ => "Unknown failure",
            }
        )
    }
}
