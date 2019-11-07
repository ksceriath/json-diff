use serde_json;
use serde_json::Map;
use serde_json::Value;

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

    if value == value2 {
        println!("JSONs match.");
    } else {
        match match_json(&value, &value2) {
            Mismatch::NoMismatch => println!("No mismatch was found."),
            Mismatch::ValueMismatch => println!("Mismatch at root."),
            Mismatch::ObjectMismatch(a,b,c) => {
                if let Some(left_keys) = a {
                    println!("Following keys were not found in second object: {:?}", left_keys);
                }
                if let Some(right_keys) = b {
                    println!("Following keys were not found in first object: {:?}", right_keys);
                }
                if let Some(unequal_keys) = c {
                    println!("Following keys were not found to be equal: {:?}", unequal_keys);
                }
            }
        };
    }
}

enum Mismatch {
    NoMismatch,
    ValueMismatch,
    ObjectMismatch(Option<Vec<String>>, Option<Vec<String>>, Option<Vec<String>>),
}

fn match_json(value: &Value, value1: &Value) -> Mismatch {
    match (value, value1) {
        (Value::Object(a), Value::Object(b)) => {
            let (mut left, mut right, intersection) = intersect_maps(&a, &b);
            let mut unequal_keys = vec![];

            for key in intersection {
                let append_key = |x: &String| { 
                    let mut n = String::from(&key);
                    n.push('.');
                    n.push_str(x);
                    n.to_string()
                };
                let x = match_json(&a.get(&key).unwrap(), &b.get(&key).unwrap());
                if let Some(mut keys) = match x {
                    Mismatch::NoMismatch => None,
                    Mismatch::ValueMismatch => Some(vec![key]),
                    Mismatch::ObjectMismatch(left_keys, right_keys, mismatch_keys) => {
                        if let Some(left_keys) = left_keys {
                            left.append(&mut left_keys.iter().map(append_key).collect::<Vec<String>>());
                        }
                        if let Some(right_keys) = right_keys {
                            right.append(&mut right_keys.iter().map(append_key).collect::<Vec<String>>());
                        }
                        if let Some(mismatch_keys) = mismatch_keys {
                            Some(mismatch_keys.iter().map(append_key).collect::<Vec<String>>())
                        } else {
                            None
                        }
                    },
                } {
                    unequal_keys.append(&mut keys);
                }
            }
            Mismatch::ObjectMismatch(Some(left), Some(right), Some(unequal_keys))
        },
        (a, b) => {
            if a == b {
                Mismatch::NoMismatch
            } else {
                Mismatch::ValueMismatch
            }
        }
    }
}

fn intersect_maps<'a>(a: &Map<String, Value>, 
    b: &Map<String, Value>) -> (Vec<String>, 
    Vec<String>, Vec<String>) {
    let mut intersection = vec![];
    let mut left = vec![];
    let mut right = vec![];
    for a_key in a.keys() {
        if b.contains_key(a_key) {
            intersection.push(String::from(a_key));
        } else {
            left.push(String::from(a_key));
        }
    }
    for b_key in b.keys() {
        if !a.contains_key(b_key) {
            right.push(String::from(b_key));
        }
    }
    (left, right, intersection)
}
