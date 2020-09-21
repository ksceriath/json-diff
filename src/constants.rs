use colored::*;
use std::fmt;

#[derive(Debug)]
pub enum Message {
    BadOption,
    SOURCE1,
    SOURCE2,
    JSON1,
    JSON2,
    UnknownError,
    NoMismatch,
    RootMismatch,
    LeftExtra,
    RightExtra,
    Mismatch,
}

impl fmt::Display for Message {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let message = match self {
            Message::BadOption => "Invalid option.",
            Message::SOURCE1 => "Could not read source1.",
            Message::SOURCE2 => "Could not read source2.",
            Message::JSON1 => "Could not parse source1.",
            Message::JSON2 => "Could not parse source2.",
            Message::UnknownError => "",
            Message::NoMismatch => "No mismatch was found.",
            Message::RootMismatch => "Mismatch at root.",
            Message::LeftExtra => "Extra on left",
            Message::RightExtra => "Extra on right",
            Message::Mismatch => "Mismatched",
        };

        write!(f, "{}", message.bold())
    }
}
