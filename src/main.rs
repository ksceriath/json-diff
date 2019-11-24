mod constants;
mod ds;
mod process;

use colored::*;
use constants::Message;
use ds::key_node::KeyNode;
use ds::mismatch::Mismatch;
use serde_json;
use std::fmt;
use std::fs;
use std::process as proc;
use std::str::FromStr;
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

fn error_exit(message: constants::Message) -> ! {
    eprintln!("{}", message);
    proc::exit(1);
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
    display_output(compare_jsons(&data1, &data2));
}

fn display_output(result: Mismatch) {
    let no_mismatch = Mismatch {
        left_only_keys: KeyNode::Nil,
        right_only_keys: KeyNode::Nil,
        keys_in_both: KeyNode::Nil,
    };
    if no_mismatch == result {
        println!("{}", Message::NoMismatch);
    } else {
        match result.keys_in_both {
            KeyNode::Node(_) => {
                let mut keys = Vec::new();
                result.keys_in_both.absolute_keys(&mut keys, None);
                println!("{}:", Message::Mismatch);
                for key in keys {
                    println!("{}", key);
                }
            }
            KeyNode::Value(_, _) => println!("{}", Message::RootMismatch),
            KeyNode::Nil => (),
        }
        match result.left_only_keys {
            KeyNode::Node(_) => {
                let mut keys = Vec::new();
                result.left_only_keys.absolute_keys(&mut keys, None);
                println!("{}:", Message::LeftExtra);
                for key in keys {
                    println!("{}", key.red().bold());
                }
            }
            KeyNode::Value(_, _) => error_exit(Message::UnknownError),
            KeyNode::Nil => (),
        }
        match result.right_only_keys {
            KeyNode::Node(_) => {
                let mut keys = Vec::new();
                result.right_only_keys.absolute_keys(&mut keys, None);
                println!("{}:", Message::RightExtra);
                for key in keys {
                    println!("{}", key.green().bold());
                }
            }
            KeyNode::Value(_, _) => error_exit(Message::UnknownError),
            KeyNode::Nil => (),
        }
    }
}

fn compare_jsons(a: &str, b: &str) -> Mismatch {
    if let Ok(value1) = serde_json::from_str(a) {
        if let Ok(value2) = serde_json::from_str(b) {
            process::match_json(&value1, &value2)
        } else {
            error_exit(Message::JSON2);
        }
    } else {
        error_exit(Message::JSON1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use maplit::hashmap;
    use serde_json::json;

    #[test]
    fn nested_diff() {
        let data1 = r#"{
            "a":"b", 
            "b":{
                "c":{
                    "d":true,
                    "e":5,
                    "f":9,
                    "h":{
                        "i":true,
                        "j":false
                    }
                }
            }
        }"#;
        let data2 = r#"{
            "a":"b",
            "b":{
                "c":{
                    "d":true,
                    "e":6,
                    "g":0,
                    "h":{
                        "i":false,
                        "k":false
                    }
                }
            }
        }"#;

        let expected_left = KeyNode::Node(hashmap! {
        "b".to_string() => KeyNode::Node(hashmap! {
                "c".to_string() => KeyNode::Node(hashmap! {
                        "f".to_string() => KeyNode::Nil,
                        "h".to_string() => KeyNode::Node( hashmap! {
                                "j".to_string() => KeyNode::Nil,
                            }
                        ),
                }
                ),
            }),
        });
        let expected_right = KeyNode::Node(hashmap! {
            "b".to_string() => KeyNode::Node(hashmap! {
                    "c".to_string() => KeyNode::Node(hashmap! {
                            "g".to_string() => KeyNode::Nil,
                            "h".to_string() => KeyNode::Node(hashmap! {
                                    "k".to_string() => KeyNode::Nil,
                                }
                            )
                        }
                    )
                }
            )
        });
        let expected_uneq = KeyNode::Node(hashmap! {
            "b".to_string() => KeyNode::Node(hashmap! {
                    "c".to_string() => KeyNode::Node(hashmap! {
                            "e".to_string() => KeyNode::Value(json!(5), json!(6)),
                            "h".to_string() => KeyNode::Node(hashmap! {
                                    "i".to_string() => KeyNode::Value(json!(true), json!(false)),
                                }
                            )
                        }
                    )
                }
            )
        });
        let expected = Mismatch::new(expected_left, expected_right, expected_uneq);

        let mismatch = compare_jsons(data1, data2);
        assert_eq!(mismatch, expected, "Diff was incorrect.");
    }

    #[test]
    fn no_diff() {
        let data1 = r#"{
            "a":"b", 
            "b":{
                "c":{
                    "d":true,
                    "e":5,
                    "f":9,
                    "h":{
                        "i":true,
                        "j":false
                    }
                }
            }
        }"#;
        let data2 = r#"{
            "a":"b", 
            "b":{
                "c":{
                    "d":true,
                    "e":5,
                    "f":9,
                    "h":{
                        "i":true,
                        "j":false
                    }
                }
            }
        }"#;

        assert_eq!(
            compare_jsons(data1, data2),
            Mismatch::new(KeyNode::Nil, KeyNode::Nil, KeyNode::Nil)
        );
    }

    #[test]
    fn no_json() {
        let data1 = r#"{}"#;
        let data2 = r#"{}"#;

        assert_eq!(
            compare_jsons(data1, data2),
            Mismatch::new(KeyNode::Nil, KeyNode::Nil, KeyNode::Nil)
        );
    }
}
