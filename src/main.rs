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

    let data1 =
        &fs::read_to_string(file1).expect(&format!("Error occurred while reading {}", file1));
    let data2 =
        &fs::read_to_string(file2).expect(&format!("Error occurred while reading {}", file2));

    display_output(compare_jsons(data1, data2));
}

fn display_output(result: Mismatch) {
    match result {
        Mismatch::None => println!("No mismatch was found."),
        Mismatch::Values => println!("Mismatch at root."),
        Mismatch::Objects(None, None, None) => println!("No mismatch was found."),
        Mismatch::Objects(a, b, c) => {
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

#[derive(Debug, PartialEq)]
enum Mismatch {
    None,
    Values,
    Objects(Option<KeyMap>, Option<KeyMap>, Option<KeyMap>),
}

fn compare_jsons(a: &str, b: &str) -> Mismatch {
    let value: Value = serde_json::from_str(a).unwrap();
    let value2: Value = serde_json::from_str(b).unwrap();

    match_json(&value, &value2)
}

fn match_json(value1: &Value, value2: &Value) -> Mismatch {
    match (value1, value2) {
        (Value::Object(a), Value::Object(b)) => {
            let (left_only_keys, right_only_keys, intersection_keys) = intersect_maps(&a, &b);

            let mut unequal_keys = None;
            let mut left_only_keys = get_map_of_keys(left_only_keys);
            let mut right_only_keys = get_map_of_keys(right_only_keys);

            if let Some(intersection_keys) = intersection_keys {
                for key in intersection_keys {
                    match match_json(&a.get(&key).unwrap(), &b.get(&key).unwrap()) {
                        Mismatch::Values => {
                            unequal_keys.get_or_insert(KeyMap {
                                keys: HashMap::new(),
                            })
                            .keys
                            .insert(String::from(&key), None);
                        },

                        Mismatch::Objects(left_keys, right_keys, mismatch_keys) => {
                            insert_child_key_map(&mut left_only_keys, left_keys, &key);
                            insert_child_key_map(&mut right_only_keys, right_keys, &key);
                            insert_child_key_map(&mut unequal_keys, mismatch_keys, &key);
                        },

                        Mismatch::None => (),
                    }
                }
            }
            Mismatch::Objects(left_only_keys, right_only_keys, unequal_keys)
        }
        (a, b) => {
            if a == b {
                Mismatch::None
            } else {
                Mismatch::Values
            }
        }
    }
}

fn get_map_of_keys(set: Option<HashSet<String>>) -> Option<KeyMap> {
    set.map(|s| KeyMap {
        keys: s
            .iter()
            .map(|key| (String::from(key), None))
            .collect(),
    })
}

fn insert_child_key_map(parent: &mut Option<KeyMap>, child: Option<KeyMap>, key: &String) {
    if child.is_some() {
        parent.get_or_insert(KeyMap {
            keys: HashMap::new(),
        })
        .keys
        .insert(String::from(key), child); // TODO check: do we ever insert 'None' here?
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
            Mismatch::Objects(Some(a), Some(b), Some(c)) => {
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
            _ => assert!(false, "Mismatch was not of type Objects"),
        }
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

        assert_eq!(compare_jsons(data1, data2), Mismatch::Objects(None, None, None));
    }
}
