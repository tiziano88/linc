use std::collections::HashMap;

use maplit::hashmap;

type UUID = String;

// Schema of the schema.
struct Schema {
    types: HashMap<&'static str, Type>,
}

struct Type {
    name: &'static str,
    fields: HashMap<usize, Field>,
}

struct Field {
    name: &'static str,
    repeated: bool,
    required: bool,
    raw: bool,
    types: Vec<&'static str>,
}

fn s() -> Schema {
    Schema {
        types: hashmap! {
            "8e2b2238-b4a1-45ef-b518-863b1a1ccf10" => Type {
                name: "root",
                fields: hashmap! {
                    0 => Field {
                        name: "value",
                        repeated: false,
                        required: false,
                        raw: false,
                        types: vec!["8e2b2238-b4a1-45ef-b518-863b1a1ccf10"],
                    },
                },
            }
        },
    }
}
