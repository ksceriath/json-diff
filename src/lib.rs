pub mod constants;
pub mod ds;
mod process;

use colored::*;
use constants::Message;
use ds::{key_node::KeyNode, mismatch::Mismatch};
use std::io::{self, Write};

pub fn compare_jsons(a: &str, b: &str) -> Result<Mismatch, serde_json::Error> {
    let value1 = serde_json::from_str(a)?;
    let value2 = serde_json::from_str(b)?;
    Ok(process::match_json(&value1, &value2))
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

        let mismatch = compare_jsons(data1, data2).unwrap();
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
            compare_jsons(data1, data2).unwrap(),
            Mismatch::new(KeyNode::Nil, KeyNode::Nil, KeyNode::Nil)
        );
    }

    #[test]
    fn no_json() {
        let data1 = r#"{}"#;
        let data2 = r#"{}"#;

        assert_eq!(
            compare_jsons(data1, data2).unwrap(),
            Mismatch::new(KeyNode::Nil, KeyNode::Nil, KeyNode::Nil)
        );
    }
}
