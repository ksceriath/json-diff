use crate::ds::key_node::KeyNode;
use crate::ds::mismatch::Mismatch;
use serde_json::Map;
use serde_json::Value;
use std::collections::HashMap;
use std::collections::HashSet;

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
