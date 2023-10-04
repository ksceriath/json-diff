use colored::*;
use serde_json::Value;
use std::collections::HashMap;

#[derive(Debug, PartialEq)]
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
            new_s
        }
    }
}

impl KeyNode {
    pub fn absolute_keys_to_vec(&self, max_display_length: Option<usize>) -> Vec<String> {
        let mut vec = Vec::new();
        self.absolute_keys(&mut vec, None, max_display_length);
        vec
    }

    pub fn absolute_keys(
        &self,
        keys: &mut Vec<String>,
        key_from_root: Option<String>,
        max_display_length: Option<usize>,
    ) {
        let max_display_length = max_display_length.unwrap_or(20);
        let val_key = |key: Option<String>| {
            key.map(|mut s| {
                s.push_str(" ->");
                s
            })
            .unwrap_or_default()
        };
        match self {
            KeyNode::Nil => keys.push(key_from_root.unwrap_or_default()),
            KeyNode::Value(a, b) => keys.push(format!(
                "{} [ {} :: {} ]",
                val_key(key_from_root),
                truncate(a.to_string().as_str(), max_display_length)
                    .blue()
                    .bold(),
                truncate(b.to_string().as_str(), max_display_length)
                    .cyan()
                    .bold()
            )),
            KeyNode::Node(map) => {
                for (key, value) in map {
                    value.absolute_keys(
                        keys,
                        Some(format!("{} {}", val_key(key_from_root.clone()), key)),
                        Some(max_display_length),
                    )
                }
            }
        }
    }
}
