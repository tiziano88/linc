use super::types::*;
use std::collections::{BTreeMap, HashMap};

pub fn initial() -> NodeStore {
    let node = Node {
        kind: crate::schema::ROOT.to_string(),
        value: "".to_string(),
        links: BTreeMap::new(),
    };
    let mut node_store = NodeStore {
        nodes: HashMap::new(),
        root: "".to_string(),
        log: vec![],
    };
    let h = node_store.add_node(&node);
    node_store.root = h;
    node_store
}
