use colored::*;
use serde_json::Value;
use std::collections::HashMap;

#[derive(Debug, PartialEq)] // TODO check: do we need PartiaEq ?
pub enum KeyNode {
    Nil,
    Value(Value, Value),
    Node(HashMap<String, KeyNode>),
}

fn truncate(s: &str, max_chars: usize) -> String {
    match s.char_indices().nth(max_chars) {
        None => String::from(s),
        Some((idx, _)) => {
            let shorter = &s[..idx];
            let snip = "//SNIP//";
            let new_s = format!("{}{}", shorter, snip);
            String::from(new_s)
        }
    }
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
                truncate(a.to_string().as_str(), 20).blue().bold(),
                truncate(b.to_string().as_str(), 20).cyan().bold()
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

