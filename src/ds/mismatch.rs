use crate::ds::key_node::KeyNode;

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
}
