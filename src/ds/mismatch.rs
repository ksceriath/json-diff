use crate::ds::key_node::KeyNode;
use crate::enums::{DiffType, ValueType};

#[derive(Debug, PartialEq)]
pub struct Mismatch {
    pub left_only_keys: KeyNode,
    pub right_only_keys: KeyNode,
    pub keys_in_both: KeyNode,
}

impl Mismatch {
    pub fn new(l: KeyNode, r: KeyNode, u: KeyNode) -> Mismatch {
        Mismatch {
            left_only_keys: l,
            right_only_keys: r,
            keys_in_both: u,
        }
    }

    pub fn empty() -> Self {
        Mismatch {
            left_only_keys: KeyNode::Nil,
            keys_in_both: KeyNode::Nil,
            right_only_keys: KeyNode::Nil,
        }
    }

    pub fn all_diffs(&self) -> Vec<(DiffType, ValueType)> {
        self.all_diffs_trunc(None)
    }

    pub fn all_diffs_trunc(&self, truncation_length: Option<usize>) -> Vec<(DiffType, ValueType)> {
        let both = self
            .keys_in_both
            .absolute_keys_to_vec(truncation_length)
            .into_iter()
            .map(|k| (DiffType::Mismatch, k));
        let left = self
            .left_only_keys
            .absolute_keys_to_vec(truncation_length)
            .into_iter()
            .map(|k| (DiffType::LeftExtra, k));
        let right = self
            .right_only_keys
            .absolute_keys_to_vec(truncation_length)
            .into_iter()
            .map(|k| (DiffType::RightExtra, k));

        both.chain(left).chain(right).collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn empty_diffs() {
        let empty = Mismatch::empty();
        let all_diffs = empty.all_diffs();
        assert!(all_diffs.is_empty());
    }
}
