use super::types::*;
use std::collections::{BTreeMap, HashMap};

pub fn initial() -> File {
    let node = Node {
        kind: crate::schema::ROOT.to_string(),
        value: "".to_string(),
        links: BTreeMap::new(),
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
