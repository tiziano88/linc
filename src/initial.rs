use super::types::*;
use std::collections::{BTreeMap, HashMap};

pub fn initial() -> (NodeStore, String) {
    let node = Node {
        kind: crate::schema::ROOT.to_string(),
        links: BTreeMap::new(),
    };
    let mut node_store = NodeStore::default();
    let h = node_store.put_parsed(&node);
    (node_store, h)
}
