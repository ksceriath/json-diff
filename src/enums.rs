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

pub enum ValueType {
    Key(String),
    Value {
        key: String,
        value_left: String,
        value_right: String,
    },
}

impl ValueType {
    pub fn new_value(key: String, value_left: String, value_right: String) -> Self {
        Self::Value {
            value_right,
            value_left,
            key,
        }
    }
    pub fn new_key(key: String) -> Self {
        Self::Key(key)
    }

    pub fn get_key(&self) -> &str {
        match self {
            ValueType::Value { key, .. } => key.as_str(),
            ValueType::Key(key) => key.as_str(),
        }
    }
}

impl Display for ValueType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ValueType::Key(key) => write!(f, "{key}"),
            ValueType::Value {
                value_left,
                key,
                value_right,
            } => {
                write!(f, "{key} [ {value_left} :: {value_right} ]")
            }
        }
    }
}
