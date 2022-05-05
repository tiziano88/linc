use crate::schema::{Field, Kind, Schema};

use super::types::*;
use std::collections::{BTreeMap, HashMap};

pub fn initial() -> (NodeStore, String) {
    let node = Node::default();
    let mut node_store = NodeStore::default();
    let h = node_store.put_parsed(&node);
    (node_store, h)
}

pub fn initial_schema() -> Schema {
    Schema {
        kinds: vec![
            Kind {
                kind_id: 3021731,
                name: "root".to_string(),
                fields: vec![
                    Field {
                        field_id: 3021731,
                        name: "git_command".to_string(),
                        kind_id: 23427,
                        raw: 0,
                    },
                    Field {
                        field_id: 3021732,
                        name: "docker_command".to_string(),
                        kind_id: 23428,
                        raw: 0,
                    },
                ],
            },
            Kind {
                kind_id: 23427,
                name: "git_command".to_string(),
                fields: vec![
                    Field {
                        field_id: 131987,
                        name: "git_add".to_string(),
                        kind_id: 2188123,
                        raw: 0,
                    },
                    Field {
                        field_id: 2429447,
                        name: "git_push".to_string(),
                        kind_id: 349872,
                        raw: 0,
                    },
                ],
            },
            Kind {
                kind_id: 131987,
                name: "git_add".to_string(),
                fields: vec![],
            },
            Kind {
                kind_id: 2429447,
                name: "git_push".to_string(),
                fields: vec![],
            },
            Kind {
                kind_id: 23428,
                name: "docker_command".to_string(),
                fields: vec![],
            },
        ],
    }
}
