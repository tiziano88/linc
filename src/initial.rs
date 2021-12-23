use super::types::*;
use std::collections::{BTreeMap, HashMap};

pub fn initial() -> File {
    let node = Node {
        kind: "root".to_string(),
        value: "".to_string(),
        children: BTreeMap::new(),
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
