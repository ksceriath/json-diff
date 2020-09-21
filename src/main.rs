use json_diff::{compare_jsons, constants::Message, display_output};
use std::{fmt, fs, process as proc, str::FromStr};
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
