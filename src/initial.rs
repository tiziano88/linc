use super::types::*;
use maplit::hashmap;
use std::collections::HashMap;

pub fn initial() -> File {
    File {
        nodes: hashmap!["101010".to_string() => Node {
            kind: "rust_fragment".to_string(),
            value: "".to_string(),
            children: HashMap::new(),
        }],
        root: "101010".to_string(),
    }
}
