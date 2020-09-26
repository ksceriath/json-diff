use std::collections::HashMap;
use std::collections::HashSet;

use serde_json::Map;
use serde_json::Value;

use crate::constants::Message;
use crate::ds::key_node::KeyNode;
use crate::ds::mismatch::Mismatch;

pub fn compare_jsons(a: &str, b: &str) -> Result<Mismatch, Message> {
    let value1 = match serde_json::from_str(a) {
        Ok(val1) => val1,
        Err(_) => return Err(Message::JSON1),
    };
    let value2 = match serde_json::from_str(b) {
        Ok(val2) => val2,
        Err(_) => return Err(Message::JSON2),
    };
    Ok(match_json(&value1, &value2))
}

pub fn match_json(value1: &Value, value2: &Value) -> Mismatch {
    match (value1, value2) {
        (Value::Object(a), Value::Object(b)) => {
            let (left_only_keys, right_only_keys, intersection_keys) = intersect_maps(&a, &b);

            let mut unequal_keys = KeyNode::Nil;
            let mut left_only_keys = get_map_of_keys(left_only_keys);
            let mut right_only_keys = get_map_of_keys(right_only_keys);

            if let Some(intersection_keys) = intersection_keys {
                for key in intersection_keys {
                    let Mismatch {
                        left_only_keys: l,
                        right_only_keys: r,
                        keys_in_both: u,
                    } = match_json(&a.get(&key).unwrap(), &b.get(&key).unwrap());
                    left_only_keys = insert_child_key_map(left_only_keys, l, &key);
                    right_only_keys = insert_child_key_map(right_only_keys, r, &key);
                    unequal_keys = insert_child_key_map(unequal_keys, u, &key);
                }
            }
            Mismatch::new(left_only_keys, right_only_keys, unequal_keys)
        }
        (a, b) => {
            if a == b {
                Mismatch::new(KeyNode::Nil, KeyNode::Nil, KeyNode::Nil)
            } else {
                Mismatch::new(
                    KeyNode::Nil,
                    KeyNode::Nil,
                    KeyNode::Value(a.clone(), b.clone()),
                )
            }
        }
    }
}

fn get_map_of_keys(set: Option<HashSet<String>>) -> KeyNode {
    if let Some(set) = set {
        KeyNode::Node(
            set.iter()
                .map(|key| (String::from(key), KeyNode::Nil))
                .collect(),
        )
    } else {
        KeyNode::Nil
    }
}

fn insert_child_key_map(parent: KeyNode, child: KeyNode, key: &String) -> KeyNode {
    if child == KeyNode::Nil {
        return parent;
    }
    if let KeyNode::Node(mut map) = parent {
        map.insert(String::from(key), child);
        KeyNode::Node(map) // This is weird! I just wanted to return back `parent` here
    } else if let KeyNode::Nil = parent {
        let mut map = HashMap::new();
        map.insert(String::from(key), child);
        KeyNode::Node(map)
    } else {
        parent // TODO Trying to insert child node in a Value variant : Should not happen => Throw an error instead.
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

    #[test]
    fn parse_err_source_one() {
        let invalid_json1 = r#"{invalid: json}"#;
        let valid_json2 = r#"{"a":"b"}"#;
        match compare_jsons(invalid_json1, valid_json2) {
            Ok(_) => panic!("This shouldn't be an Ok"),
            Err(err) => {
                assert_eq!(Message::JSON1, err);
            }
        };
    }

    #[test]
    fn parse_err_source_two() {
        let valid_json1 = r#"{"a":"b"}"#;
        let invalid_json2 = r#"{invalid: json}"#;
        match compare_jsons(valid_json1, invalid_json2) {
            Ok(_) => panic!("This shouldn't be an Ok"),
            Err(err) => {
                assert_eq!(Message::JSON2, err);
            }
        };
    }
}
