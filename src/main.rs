use serde_json;
use serde_json::Map;
use serde_json::Value;
use std::collections::HashMap;
use std::collections::HashSet;
use std::env;
use std::fs;

fn main() {
    let args = env::args().collect::<Vec<String>>();
    let file1 = &args[1];
    let file2 = &args[2];

    let data =
        &fs::read_to_string(file1).expect(&format!("Error occurred while reading {}", file1));
    let data2 =
        &fs::read_to_string(file2).expect(&format!("Error occurred while reading {}", file2));

    display_output(compare_jsons(data, data2));
}

fn compare_jsons(a: &str, b: &str) -> Mismatch {
    let value: Value = serde_json::from_str(a).unwrap();
    let value2: Value = serde_json::from_str(b).unwrap();

    match_json(&value, &value2)
}

fn display_output(result: Mismatch) {
    match result {
        Mismatch::NoMismatch => println!("No mismatch was found."),
        Mismatch::ValueMismatch => println!("Mismatch at root."),
        Mismatch::ObjectMismatch(None, None, None) => println!("No mismatch was found."),
        Mismatch::ObjectMismatch(a, b, c) => {
            if let Some(left_keys) = a {
                println!(
                    "Following keys were not found in second object: {:?}",
                    left_keys
                );
            }
            if let Some(right_keys) = b {
                println!(
                    "Following keys were not found in first object: {:?}",
                    right_keys
                );
            }
            if let Some(unequal_keys) = c {
                println!(
                    "Following keys were not found to be equal: {:?}",
                    unequal_keys
                );
            }
        }
    };
}

#[derive(Debug, PartialEq)]
struct KeyMap {
    keys: HashMap<String, Option<KeyMap>>,
}

#[derive(Debug)]
enum Mismatch {
    NoMismatch,
    ValueMismatch,
    ObjectMismatch(Option<KeyMap>, Option<KeyMap>, Option<KeyMap>),
}

fn match_json(value: &Value, value1: &Value) -> Mismatch {
    match (value, value1) {
        (Value::Object(a), Value::Object(b)) => {
            let (left, right, intersection) = intersect_maps(&a, &b);
            let mut unequal_keys = None;

            let mut left = left.map(|l| KeyMap {
                keys: l
                    .iter()
                    .map(|x| (String::from(x), None))
                    .collect::<HashMap<String, Option<KeyMap>>>(),
            });

            let mut right = right.map(|r| KeyMap {
                keys: r
                    .iter()
                    .map(|x| (String::from(x), None))
                    .collect::<HashMap<String, Option<KeyMap>>>(),
            });

            if let Some(intersection) = intersection {
                for key in intersection {
                    if let Some((key, value)) =
                        match match_json(&a.get(&key).unwrap(), &b.get(&key).unwrap()) {
                            Mismatch::NoMismatch => None,

                            Mismatch::ValueMismatch => Some((key, None)),

                            Mismatch::ObjectMismatch(left_keys, right_keys, mismatch_keys) => {
                                if let Some(left_keys) = left_keys {
                                    left.get_or_insert(KeyMap {
                                        keys: HashMap::new(),
                                    })
                                    .keys
                                    .insert(String::from(&key), Some(left_keys));
                                }
                                if let Some(right_keys) = right_keys {
                                    right
                                        .get_or_insert(KeyMap {
                                            keys: HashMap::new(),
                                        })
                                        .keys
                                        .insert(String::from(&key), Some(right_keys));
                                }
                                if let Some(mismatch_keys) = mismatch_keys {
                                    Some((String::from(&key), Some(mismatch_keys)))
                                } else {
                                    None
                                }
                            }
                        }
                    {
                        unequal_keys
                            .get_or_insert(KeyMap {
                                keys: HashMap::new(),
                            })
                            .keys
                            .insert(key, value);
                    }
                }
            }
            Mismatch::ObjectMismatch(left, right, unequal_keys)
        }
        (a, b) => {
            if a == b {
                Mismatch::NoMismatch
            } else {
                Mismatch::ValueMismatch
            }
        }
    }
}

fn intersect_maps(
    a: &Map<String, Value>,
    b: &Map<String, Value>,
) -> (
    Option<HashSet<String>>,
    Option<HashSet<String>>,
    Option<HashSet<String>>,
) {
    let mut intersection = HashSet::new();
    let mut left = HashSet::new();
    let mut right = HashSet::new();
    for a_key in a.keys() {
        if b.contains_key(a_key) {
            intersection.insert(String::from(a_key));
        } else {
            left.insert(String::from(a_key));
        }
    }
    for b_key in b.keys() {
        if !a.contains_key(b_key) {
            right.insert(String::from(b_key));
        }
    }
    let left = if left.len() == 0 { None } else { Some(left) };
    let right = if right.len() == 0 { None } else { Some(right) };
    let intersection = if intersection.len() == 0 {
        None
    } else {
        Some(intersection)
    };
    (left, right, intersection)
}

#[cfg(test)]
mod tests {
    use super::*;
    use maplit::hashmap;

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

        let mismatch = compare_jsons(data1, data2);
        match mismatch {
            Mismatch::ObjectMismatch(Some(a), Some(b), Some(c)) => {
                let expected_left = KeyMap {
                    keys: hashmap! {
                        "b".to_string() => Some(KeyMap {
                            keys: hashmap! {
                                "c".to_string() => Some(KeyMap {
                                    keys: hashmap! {
                                        "f".to_string() => None,
                                        "h".to_string() => Some(KeyMap {
                                            keys: hashmap! {
                                                "j".to_string() => None,
                                            }
                                        })
                                    }
                                })
                            }
                        })
                    },
                };

                let expected_right = KeyMap {
                    keys: hashmap! {
                        "b".to_string() => Some(KeyMap {
                            keys: hashmap! {
                                "c".to_string() => Some(KeyMap {
                                    keys: hashmap! {
                                        "g".to_string() => None,
                                        "h".to_string() => Some(KeyMap {
                                            keys: hashmap! {
                                                "k".to_string() => None,
                                            }
                                        })
                                    }
                                })
                            }
                        })
                    },
                };

                let expected_uneq = KeyMap {
                    keys: hashmap! {
                        "b".to_string() => Some(KeyMap {
                            keys: hashmap! {
                                "c".to_string() => Some(KeyMap {
                                    keys: hashmap! {
                                        "e".to_string() => None,
                                        "h".to_string() => Some(KeyMap {
                                            keys: hashmap! {
                                                "i".to_string() => None,
                                            }
                                        })
                                    }
                                })
                            }
                        })
                    },
                };

                assert_eq!(a, expected_left, "Left was incorrect.");
                assert_eq!(b, expected_right, "Right was incorrect.");
                assert_eq!(c, expected_uneq, "unequals were incorrect.");
            }
            _ => assert!(false, "Mismatch was not of type ObjectMismatch"),
        }
    }
}
