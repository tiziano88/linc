use super::types::*;
use maplit::hashmap;

pub fn initial() -> File {
    File {
        nodes: hashmap!["101010".to_string() => Node {
            kind: "document".to_string(),
            value: Value::Inner(Inner {
                children: hashmap! {
                    "bindings".to_string() => vec![
                        "111".to_string(),
                        "12".to_string(),
                        "87498273489273".to_string(),
                    ],
                },
            }),
        }],
        root: "101010".to_string(),
    }
}
