use super::types::*;
use maplit::hashmap;

pub fn initial() -> File {
    File {
        nodes: vec![
            Node {
                reference: "101010".to_string(),
                value: Value::Inner(Inner {
                    kind: "document".to_string(),
                    children: hashmap! {
                        "bindings".to_string() => vec![
                            "111".to_string(),
                            "12".to_string(),
                            "87498273489273".to_string(),
                        ],
                    },
                }),
            },
            Node {
                reference: "111".to_string(),
                value: Value::Inner(Inner {
                    kind: "function_definition".to_string(),
                    children: hashmap! {
                        "name".to_string() => vec!["125".to_string()],
                        "arguments".to_string() => vec![],
                        "outer_attributes".to_string() => vec![],
                        "inner_attributes".to_string() => vec![],
                        "return_type".to_string() => vec![],
                        "body".to_string() => vec!["99999999".to_string()],
                        "pub".to_string() => vec!["126".to_string()],
                    },
                }),
            },
            Node {
                reference: "124".to_string(),
                value: Value::Int(123),
            },
            Node {
                reference: "125".to_string(),
                value: Value::String("main".to_string()),
            },
            Node {
                reference: "126".to_string(),
                value: Value::Bool(true),
            },
            Node {
                reference: "12".to_string(),
                value: Value::Inner(Inner {
                    kind: "function_definition".to_string(),
                    children: hashmap! {
                        "name".to_string() => vec!["126".to_string()],
                        "arguments".to_string() => vec!["222".to_string()],
                        "outer_attributes".to_string() => vec![],
                        "inner_attributes".to_string() => vec![],
                        "return_type".to_string() => vec![],
                        "body".to_string() => vec!["228".to_string()],
                    },
                }),
            },
            Node {
                reference: "126".to_string(),
                value: Value::String("factorial".to_string()),
            },
            Node {
                reference: "222".to_string(),
                value: Value::Inner(Inner {
                    kind: "pattern".to_string(),
                    children: hashmap! {
                        "name".to_string() => vec!["2223".to_string()],
                    },
                }),
            },
            Node {
                reference: "2223".to_string(),
                value: Value::String("x".to_string()),
            },
            Node {
                reference: "228".to_string(),
                value: Value::Inner(Inner {
                    kind: "binary_operator".to_string(),
                    children: hashmap! {
                        "operator".to_string() => vec![],
                        "left".to_string() => vec!["1231".to_string()],
                        "right".to_string() => vec!["1232".to_string()]
                    },
                }),
            },
            Node {
                reference: "1231".to_string(),
                value: Value::Inner(Inner {
                    kind: "ref".to_string(),
                    children: hashmap! {
                        "target".to_string() => vec!["222".to_string()],
                    },
                }),
            },
            Node {
                reference: "1232".to_string(),
                value: Value::Inner(Inner {
                    kind: "function_call".to_string(),
                    children: hashmap! {
                        "function".to_string() => vec!["126".to_string()],
                        "arguments".to_string() => vec!["229".to_string()]
                    },
                }),
            },
            Node {
                reference: "229".to_string(),
                value: Value::Inner(Inner {
                    kind: "binary_operator".to_string(),
                    // TODO: -
                    children: hashmap! {
                        "operator".to_string() => vec![],
                        "left".to_string() => vec!["230".to_string()],
                        "right".to_string() => vec!["231".to_string()]
                    },
                }),
            },
            Node {
                reference: "230".to_string(),
                value: Value::Inner(Inner {
                    kind: "ref".to_string(),
                    children: hashmap! {
                        "target".to_string() => vec!["222".to_string()],
                    },
                }),
            },
            Node {
                reference: "231".to_string(),
                value: Value::Int(1),
            },
            // Nameless function.
            Node {
                reference: "87498273489273".to_string(),
                value: Value::Inner(Inner {
                    kind: "function_definition".to_string(),
                    children: hashmap! {
                        "name".to_string() => vec![],
                        "arguments".to_string() => vec![],
                        "outer_attributes".to_string() => vec![],
                        "inner_attributes".to_string() => vec![],
                        "return_type".to_string() => vec![],
                        "body".to_string() => vec![],
                    },
                }),
            },
        ],
        root: "101010".to_string(),
    }
}
