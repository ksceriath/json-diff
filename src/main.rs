use serde_json;
use serde_json::Map;
use serde_json::Value;
use std::collections::BTreeSet;

fn main() {
    let data = r#"{
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
    let value: Value = serde_json::from_str(data).unwrap();
    let value2: Value = serde_json::from_str(data2).unwrap();

    match match_json(&value, &value2) {
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

#[derive(Eq, PartialEq, Ord, PartialOrd, Debug)]
struct KeyMap {
    key: String,
    children: Option<BTreeSet<KeyMap>>,
}

#[derive(Debug)]
enum Mismatch {
    NoMismatch,
    ValueMismatch,
    ObjectMismatch(
        Option<BTreeSet<KeyMap>>,
        Option<BTreeSet<KeyMap>>,
        Option<BTreeSet<KeyMap>>,
    ),
}

fn match_json(value: &Value, value1: &Value) -> Mismatch {
    match (value, value1) {
        (Value::Object(a), Value::Object(b)) => {
            let (left, right, intersection) = intersect_maps(&a, &b);
            let mut unequal_keys = None;

            let mut left = left.map(|l| {
                l.iter()
                    .map(|x| KeyMap {
                        key: String::from(x),
                        children: None,
                    })
                    .collect::<BTreeSet<KeyMap>>()
            });
            let mut right = right.map(|r| {
                r.iter()
                    .map(|x| KeyMap {
                        key: String::from(x),
                        children: None,
                    })
                    .collect::<BTreeSet<KeyMap>>()
            });

            if let Some(intersection) = intersection {
                for key in intersection {
                    if let Some(keys) =
                        match match_json(&a.get(&key).unwrap(), &b.get(&key).unwrap()) {
                            Mismatch::NoMismatch => None,
                            Mismatch::ValueMismatch => Some(KeyMap {
                                key,
                                children: None,
                            }),
                            Mismatch::ObjectMismatch(left_keys, right_keys, mismatch_keys) => {
                                if let Some(left_keys) = left_keys {
                                    left.get_or_insert(BTreeSet::new()).insert(KeyMap {
                                        key: String::from(&key),
                                        children: Some(left_keys),
                                    });
                                }
                                if let Some(right_keys) = right_keys {
                                    right.get_or_insert(BTreeSet::new()).insert(KeyMap {
                                        key: String::from(&key),
                                        children: Some(right_keys),
                                    });
                                }
                                if let Some(mismatch_keys) = mismatch_keys {
                                    Some(KeyMap {
                                        key: String::from(&key),
                                        children: Some(mismatch_keys),
                                    })
                                } else {
                                    None
                                }
                            }
                        }
                    {
                        unequal_keys.get_or_insert(BTreeSet::new()).insert(keys);
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
    Option<BTreeSet<String>>,
    Option<BTreeSet<String>>,
    Option<BTreeSet<String>>,
) {
    let mut intersection = BTreeSet::new();
    let mut left = BTreeSet::new();
    let mut right = BTreeSet::new();
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
