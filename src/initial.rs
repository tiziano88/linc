use super::types::*;
use std::collections::HashMap;

pub fn initial() -> File {
    let node = Node {
        kind: "rust_fragment".to_string(),
        value: "".to_string(),
        children: HashMap::new(),
    };
    let mut file = File {
        nodes: HashMap::new(),
        root: "".to_string(),
        log: vec![],
    };
    let h = file.add_node(&node);
    file.root = h;
    file
}
