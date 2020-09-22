use colored::*;
use json_diff::{
    compare_jsons,
    constants::Message,
    ds::{key_node::KeyNode, mismatch::Mismatch},
};
use std::{
    fmt, fs,
    io::{self, Write},
    process as proc,
    str::FromStr,
};
use structopt::StructOpt;

const HELP: &str = r#"
Example:
json_diff f source1.json source2.json
json_diff d '{...}' '{...}'

Option:
f   :   read input from json files
d   :   read input from command line"#;

#[derive(Debug)]
struct AppError {
    message: Message,
}
impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

enum InputReadMode {
    D,
    F,
}
impl FromStr for InputReadMode {
    type Err = AppError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "d" => Ok(InputReadMode::D),
            "f" => Ok(InputReadMode::F),
            _ => Err(Self::Err {
                message: Message::BadOption,
            }),
        }
    }
}

#[derive(StructOpt)]
#[structopt(about = HELP)]
struct Cli {
    read_mode: InputReadMode,
    source1: String,
    source2: String,
}

fn main() {
    let args = Cli::from_args();

    let (data1, data2) = match args.read_mode {
        InputReadMode::D => (args.source1, args.source2),
        InputReadMode::F => {
            if let Ok(d1) = fs::read_to_string(args.source1) {
                if let Ok(d2) = fs::read_to_string(args.source2) {
                    (d1, d2)
                } else {
                    error_exit(Message::SOURCE2);
                }
            } else {
                error_exit(Message::SOURCE1);
            }
        }
    };
    let mismatch = match compare_jsons(&data1, &data2) {
        Ok(mismatch) => mismatch,
        Err(err) => {
            eprintln!("{}", err);
            proc::exit(1)
        }
    };
    match display_output(mismatch) {
        Ok(_) => (),
        Err(err) => eprintln!("{}", err),
    };
}

fn error_exit(message: Message) -> ! {
    eprintln!("{}", message);
    proc::exit(1);
}

pub fn display_output(result: Mismatch) -> Result<(), std::io::Error> {
    let no_mismatch = Mismatch {
        left_only_keys: KeyNode::Nil,
        right_only_keys: KeyNode::Nil,
        keys_in_both: KeyNode::Nil,
    };

    let stdout = io::stdout();
    let mut handle = io::BufWriter::new(stdout.lock());
    Ok(if no_mismatch == result {
        writeln!(handle, "\n{}", Message::NoMismatch)?;
    } else {
        match result.keys_in_both {
            KeyNode::Node(_) => {
                let mut keys = Vec::new();
                result.keys_in_both.absolute_keys(&mut keys, None);
                writeln!(handle, "\n{}:", Message::Mismatch)?;
                for key in keys {
                    writeln!(handle, "{}", key)?;
                }
            }
            KeyNode::Value(_, _) => writeln!(handle, "{}", Message::RootMismatch)?,
            KeyNode::Nil => (),
        }
        match result.left_only_keys {
            KeyNode::Node(_) => {
                let mut keys = Vec::new();
                result.left_only_keys.absolute_keys(&mut keys, None);
                writeln!(handle, "\n{}:", Message::LeftExtra)?;
                for key in keys {
                    writeln!(handle, "{}", key.red().bold())?;
                }
            }
            KeyNode::Value(_, _) => (),
            KeyNode::Nil => (),
        }
        match result.right_only_keys {
            KeyNode::Node(_) => {
                let mut keys = Vec::new();
                result.right_only_keys.absolute_keys(&mut keys, None);
                writeln!(handle, "\n{}:", Message::RightExtra)?;
                for key in keys {
                    writeln!(handle, "{}", key.green().bold())?;
                }
            }
            KeyNode::Value(_, _) => (),
            KeyNode::Nil => (),
        }
    })
}
