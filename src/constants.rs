use std::fmt::{Display, Formatter};
use thiserror::Error;
use vg_errortools::FatIOError;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Error opening file: {0}")]
    IOError(#[from] FatIOError),
    #[error("Error parsing first json: {0}")]
    JSON(#[from] serde_json::Error),
}

#[derive(Debug)]
pub enum DiffType {
    RootMismatch,
    LeftExtra,
    RightExtra,
    Mismatch,
}

impl Display for DiffType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let msg = match self {
            DiffType::RootMismatch => "Mismatch at root.",
            DiffType::LeftExtra => "Extra on left",
            DiffType::RightExtra => "Extra on right",
            DiffType::Mismatch => "Mismatched",
        };
        write!(f, "{}", msg)
    }
}
