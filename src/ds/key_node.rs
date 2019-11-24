use colored::*;
use serde_json::Value;
use std::collections::HashMap;

#[derive(Debug, PartialEq)] // TODO check: do we need PartiaEq ?
pub enum KeyNode {
    Nil,
    Value(Value, Value),
    Node(HashMap<String, KeyNode>),
}

impl KeyNode {
    pub fn absolute_keys(&self, keys: &mut Vec<String>, key_from_root: Option<String>) {
        let val_key = |key: Option<String>| {
            key.map(|mut s| {
                s.push_str(" ->");
                s
            })
            .unwrap_or(String::new())
        };
        let nil_key = |key: Option<String>| key.unwrap_or(String::new());
        match self {
            KeyNode::Nil => keys.push(nil_key(key_from_root)),
            KeyNode::Value(a, b) => keys.push(format!(
                "{} [ {} :: {} ]",
                val_key(key_from_root),
                a.to_string().blue().bold(),
                b.to_string().cyan().bold()
            )),
            KeyNode::Node(map) => {
                for (key, value) in map {
                    value.absolute_keys(
                        keys,
                        Some(format!("{} {}", val_key(key_from_root.clone()), key)),
                    )
                }
            }
        }
    }
}
