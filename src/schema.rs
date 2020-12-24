use crate::types::{Model, Node, Path};
use itertools::Itertools;
use yew::{html, Html};

// https://doc.rust-lang.org/stable/reference/expressions.html
const RUST_EXPRESSION: &[&str] = &[
    "field_access",
    "function_call",
    "if",
    "match",
    "operator",
    "string_literal",
];

// https://doc.rust-lang.org/stable/reference/items.html
const RUST_ITEM: &[&str] = &["constant", "enum", "function_definition", "struct"];

// https://doc.rust-lang.org/stable/reference/types.html#type-expressions
const RUST_TYPE: &[&str] = &[
    "array_type",
    "reference_type",
    "simple_path",
    "slice_type",
    "tuple_type",
    "primitive_type",
];

// Alternative implementation: distinct structs implementing a parse_from method that only looks at
//the kind field of Inner, and we then try to parse each element with all of them until one
// matches.

// example: "true" may be an identifier, string literal, bool literal, type name.

/*
fn rust_primitive_type(n: &str) -> Kind {
    Kind {
        name: &format!("rust_primitive_type_{}", n),
        fields: &[],
        inner: None,
        parser: |v: &str| {
            if n.starts_with(v) {
                Some("".to_string())
            } else {
                None
            }
        },
        renderer: |model: &Model, value: &Node, path: &Path| {
            html! {
                <span>{ v.clone() }</div>
            }
        },
    }
}


trait K {
    fn name() -> String;
    fn fields() -> &[Field];
    fn parse(v: &str) -> Option<String>;
    fn render(model: &Model, node: &Node, path: &Path) -> Html;
}

struct RustFragment;

impl K for RustFragment {
    fn name() -> String {
        "rust_fragment".to_string()
    }

    fn fields() -> &[Field] {
        &[Field {
            name: "items",
            kind: RUST_ITEM,
            multiplicity: Multiplicity::Repeated,
        }]
    }

    fn parse(v: &str) -> Option<String> {
        if "rust_fragment".starts_with(v) {
            Some("".to_string())
        } else {
            None
        }
    }

    fn render(model: &Model, node: &Node, path: &Path) -> Html {
        let (items_head, items) = model.view_children(&node, "items", &path);
        let items = items.into_iter().map(|b| {
            html! {
                <div>{ b }</div>
            }
        });
        html! {
            <div>
            { items_head }
            { for items }
            </div>
        }
    }
}
*/

pub const SCHEMA: Schema = Schema {
    kinds: &[
        Kind {
            name: "rust_fragment",
            fields: &[Field {
                name: "items",
                kind: RUST_ITEM,
                multiplicity: Multiplicity::Repeated,
            }],
            inner: None,
            parser: |v: &str| {
                if "rust_fragment".starts_with(v) {
                    Some("".to_string())
                } else {
                    None
                }
            },
            renderer: |model: &Model, value: &Node, path: &Path| {
                let (items_head, items) = model.view_children(&value, "items", &path);
                let items = items.into_iter().map(|b| {
                    html! {
                        <div>{ b }</div>
                    }
                });
                html! {
                    <div>
                    { items_head }
                    { for items }
                    </div>
                }
            },
        },
        Kind {
            name: "tuple_type",
            fields: &[Field {
                name: "components",
                kind: RUST_TYPE,
                multiplicity: Multiplicity::Repeated,
            }],
            inner: Some("components"),
            parser: |v: &str| {
                if "tuple_type".starts_with(v) {
                    Some("".to_string())
                } else {
                    None
                }
            },
            renderer: |model: &Model, value: &Node, path: &Path| todo!(),
        },
        Kind {
            name: "primitive_type",
            fields: &[],
            inner: None,
            parser: |v: &str| match v {
                "bool" | "char" | "str" | "u8" | "u16" | "u32" | "u64" | "u128" | "i8" | "i16"
                | "i32" | "i64" | "i128" | "f32" | "f64" | "usize" | "isize" => Some(v.to_string()),
                _ => None,
            },
            renderer: |model: &Model, node: &Node, path: &Path| {
                html! {
                    <span class="keyword">{ node.value.clone() }</span>
                }
            },
        },
        Kind {
            name: "reference_type",
            fields: &[
                Field {
                    name: "type",
                    kind: RUST_TYPE,
                    multiplicity: Multiplicity::Single,
                },
                Field {
                    name: "mutable",
                    kind: &["bool"],
                    multiplicity: Multiplicity::Single,
                },
                Field {
                    name: "lifetime",
                    // XXX
                    kind: &["bool"],
                    multiplicity: Multiplicity::Single,
                },
            ],
            inner: None,
            parser: |v: &str| {
                if "reference_type".starts_with(v) || "&".starts_with(v) {
                    Some("".to_string())
                } else {
                    None
                }
            },
            renderer: |model: &Model, value: &Node, path: &Path| {
                let type_ = model.view_child(value, "type", &path);
                html! {
                    <span>
                    { "&" }{ type_ }
                    </span>
                }
            },
        },
        Kind {
            name: "constant",
            fields: &[
                Field {
                    name: "identifier",
                    kind: &["identifier"],
                    multiplicity: Multiplicity::Single,
                },
                Field {
                    name: "type",
                    kind: RUST_TYPE,
                    multiplicity: Multiplicity::Single,
                },
                Field {
                    name: "expression",
                    kind: RUST_EXPRESSION,
                    multiplicity: Multiplicity::Single,
                },
            ],
            inner: Some("statements"),
            parser: |v: &str| {
                if "const".starts_with(v) {
                    Some("".to_string())
                } else {
                    None
                }
            },
            renderer: |model: &Model, value: &Node, path: &Path| {
                let identifier = model.view_child(value, "identifier", &path);
                let type_ = model.view_child(value, "type", &path);
                let expression = model.view_child(value, "expression", &path);
                html! {
                    <span>
                    { "const" }{ identifier }{ ":" }{ type_ }{ "=" }{ expression }{ ";" }
                    </span>
                }
            },
        },
        Kind {
            name: "block",
            fields: &[Field {
                name: "statements",
                kind: RUST_EXPRESSION,
                multiplicity: Multiplicity::Repeated,
            }],
            inner: Some("statements"),
            parser: |v: &str| {
                if "block".starts_with(v) || "{".starts_with(v) {
                    Some("".to_string())
                } else {
                    None
                }
            },
            renderer: |model: &Model, value: &Node, path: &Path| {
                let (statements_head, statements) = model.view_children(value, "statements", &path);
                let statements = statements.into_iter().map(|v| {
                    html! {
                        <div class="indent">{ v }{ ";" }</div>
                    }
                });

                html! {
                    <span>
                    { "{" }{ statements_head }{ for statements }{ "}" }
                    </span>
                }
            },
        },
        Kind {
            name: "match",
            fields: &[
                Field {
                    name: "expression",
                    kind: RUST_EXPRESSION,
                    multiplicity: Multiplicity::Single,
                },
                Field {
                    name: "match_arms",
                    // XXX
                    kind: RUST_EXPRESSION,
                    multiplicity: Multiplicity::Single,
                },
            ],
            inner: Some("expression"),
            parser: |v: &str| {
                if "match".starts_with(v) {
                    Some("".to_string())
                } else {
                    None
                }
            },
            renderer: |model: &Model, value: &Node, path: &Path| {
                let expression = model.view_child(value, "expression", &path);
                let (match_arms_head, match_arms) = model.view_children(value, "match_arms", &path);
                let match_arms = match_arms.into_iter().map(|v| {
                    html! {
                        <div class="indent">{ v }{ "," }</div>
                    }
                });
                html! {
                    <span>
                        <div>
                            <span class="keyword">{ "match" }</span>{ expression }{ "{" }
                        </div>
                        { match_arms_head }
                        { for match_arms }
                        <div>
                            { "}" }
                        </div>
                    </span>
                }
            },
        },
        Kind {
            name: "if",
            fields: &[
                Field {
                    name: "condition", // Expression
                    kind: RUST_EXPRESSION,
                    multiplicity: Multiplicity::Single,
                },
                Field {
                    name: "true_body", // Expression
                    kind: RUST_EXPRESSION,
                    multiplicity: Multiplicity::Single,
                },
                Field {
                    name: "false_body", // Expression
                    kind: RUST_EXPRESSION,
                    multiplicity: Multiplicity::Single,
                },
            ],
            inner: Some("true_body"),
            parser: |v: &str| {
                if "if".starts_with(v) {
                    Some("".to_string())
                } else {
                    None
                }
            },
            renderer: |model: &Model, value: &Node, path: &Path| {
                let condition = model.view_child(value, "condition", &path);
                let true_body = model.view_child(value, "true_body", &path);
                let false_body = model.view_child(value, "false_body", &path);
                html! {
                    <span>
                        <div>
                            <span class="keyword">{ "if" }</span>{ condition }{ "{" }
                        </div>
                        <div class="indent">
                            { true_body }
                        </div>
                        <div>
                            { "}" }<span class="keyword">{ "else" }</span>{ "{" }
                        </div>
                        <div class="indent">
                            { false_body }
                        </div>
                        <div>
                            { "}" }
                        </div>
                    </span>
                }
            },
        },
        Kind {
            name: "string_literal",
            fields: &[],
            inner: None,
            parser: |v: &str| Some(v.to_string()),
            renderer: |model: &Model, value: &Node, path: &Path| {
                let value = model.view_child(value, "value", &path);
                html! {
                    <span>
                    { "\"" }{ value }{ "\"" }
                    </span>
                }
            },
        },
        Kind {
            name: "field_access",
            fields: &[
                Field {
                    name: "object",
                    kind: RUST_EXPRESSION,
                    multiplicity: Multiplicity::Single,
                },
                Field {
                    name: "field",
                    kind: &["identifier"],
                    multiplicity: Multiplicity::Single,
                },
            ],
            inner: Some("object"),
            parser: |v: &str| {
                if "field_access".starts_with(v) || ".".starts_with(v) {
                    Some(String::new())
                } else {
                    None
                }
            },
            renderer: |model: &Model, value: &Node, path: &Path| {
                let object = model.view_child(value, "object", &path);
                let field = model.view_child(value, "field", &path);
                html! {
                    <span>
                    { object }
                    { "." }
                    { field }
                    </span>
                }
            },
        },
        Kind {
            name: "simple_path",
            fields: &[Field {
                name: "segments",
                kind: &["identifier"],
                multiplicity: Multiplicity::Repeated,
            }],
            inner: Some("segments"),
            parser: |v: &str| {
                if "simple_path".starts_with(v) {
                    Some(String::new())
                } else {
                    None
                }
            },
            renderer: |model: &Model, value: &Node, path: &Path| {
                let (segments_head, segments) = model.view_children(value, "segments", &path);
                let segments = segments.into_iter().intersperse(html! {{ "::" }});
                html! {
                    <span>{ segments_head }{ for segments }</span>
                }
            },
        },
        Kind {
            name: "identifier",
            fields: &[],
            inner: None,
            parser: |v: &str| {
                if v.contains(' ') {
                    None
                } else {
                    Some(v.to_string())
                }
            },
            renderer: |model: &Model, node: &Node, path: &Path| {
                html! {
                    <span class="identifier">{ node.value.clone() }</span>
                }
            },
        },
        Kind {
            name: "crate",
            fields: &[],
            inner: None,
            parser: |v: &str| {
                if "crate".starts_with(v) {
                    Some(String::new())
                } else {
                    None
                }
            },
            renderer: |model: &Model, value: &Node, path: &Path| {
                html! {
                    <span class="keyword">{ "crate" }</span>
                }
            },
        },
        Kind {
            name: "binary_operator",
            fields: &[
                Field {
                    name: "operator",
                    // XXX
                    kind: RUST_EXPRESSION,
                    multiplicity: Multiplicity::Single,
                },
                Field {
                    name: "left",
                    kind: RUST_EXPRESSION,
                    multiplicity: Multiplicity::Single,
                },
                Field {
                    name: "right",
                    kind: RUST_EXPRESSION,
                    multiplicity: Multiplicity::Single,
                },
            ],
            inner: Some("left"),
            parser: |v: &str| {
                if "binary_operator".starts_with(v) {
                    Some(String::new())
                } else {
                    None
                }
            },
            renderer: |model: &Model, value: &Node, path: &Path| {
                let operator = model.view_child(value, "operator", &path);
                let left = model.view_child(value, "left", &path);
                let right = model.view_child(value, "right", &path);
                html! {
                    <span>
                    { left }
                    { operator }
                    { right }
                    </span>
                }
            },
        },
        // https://doc.rust-lang.org/nightly/reference/items/functions.html
        Kind {
            name: "function_definition",
            fields: &[
                /*
                Field {
                    name: "pub",
                    type_: Type::Bool,
                    multiplicity: Multiplicity::Single,
                    validator: whatever,
                },
                Field {
                    name: "async",
                    type_: Type::Bool,
                    multiplicity: Multiplicity::Single,
                    validator: whatever,
                },
                */
                Field {
                    name: "comment",
                    kind: &["markdown_fragment"],
                    multiplicity: Multiplicity::Single,
                },
                Field {
                    name: "identifier",
                    kind: &["identifier"],
                    multiplicity: Multiplicity::Single,
                },
                Field {
                    name: "parameters",
                    kind: &["function_parameter"],
                    multiplicity: Multiplicity::Repeated,
                },
                Field {
                    name: "return_type",
                    kind: RUST_TYPE,
                    multiplicity: Multiplicity::Single,
                },
                Field {
                    name: "body",
                    kind: RUST_EXPRESSION,
                    multiplicity: Multiplicity::Single,
                },
            ],
            inner: None,
            parser: |v: &str| {
                if "function_definition".starts_with(v) || "fn".starts_with(v) {
                    Some(String::new())
                } else {
                    None
                }
            },
            renderer: |model: &Model, value: &Node, path: &Path| {
                let comment = model.view_child(&value, "comment", &path);
                let identifier = model.view_child(&value, "identifier", &path);
                let (parameters_head, parameters) =
                    model.view_children(&value, "parameters", &path);
                let parameters = parameters.into_iter().intersperse(html! {{ "," }});
                let body = model.view_child(&value, "body", &path);
                let return_type = model.view_child(&value, "return_type", &path);
                // let async_ = self.view_child(&v, "async", &path);
                // let pub_ = self.view_child(&v, "pub", &path);

                html! {
                    <span>
                        <div>{ "//" }{ comment }</div>
                        // { pub_ }
                        <div>
                          <span class="keyword">{ "fn" }</span>{ identifier }
                          { "(" }{ parameters_head }{ for parameters }{ ")" }
                          { "->" }{ return_type }{ "{" }
                        </div>
                        <div class="indent">{ body }</div>
                        { "}" }
                    </span>
                }
            },
        },
        Kind {
            name: "function_parameter",
            fields: &[
                Field {
                    name: "pattern",
                    kind: &["identifier"],
                    multiplicity: Multiplicity::Single,
                },
                Field {
                    name: "type",
                    kind: RUST_TYPE,
                    multiplicity: Multiplicity::Single,
                },
            ],
            inner: None,
            parser: |v: &str| {
                if "function_parameter".starts_with(v) {
                    Some(String::new())
                } else {
                    None
                }
            },
            renderer: |model: &Model, value: &Node, path: &Path| {
                let pattern = model.view_child(&value, "pattern", &path);
                let type_ = model.view_child(&value, "type", &path);
                html! {
                    <span>
                    { pattern }{ ":" }{ type_ }
                    </span>
                }
            },
        },
        // https://doc.rust-lang.org/nightly/reference/statements.html#let-statements
        Kind {
            name: "let",
            fields: &[
                Field {
                    name: "pattern",
                    // XXX
                    kind: &["pattern"],
                    multiplicity: Multiplicity::Single,
                },
                Field {
                    name: "type",
                    kind: RUST_TYPE,
                    multiplicity: Multiplicity::Single,
                },
                Field {
                    name: "value", // Expression
                    kind: RUST_EXPRESSION,
                    multiplicity: Multiplicity::Single,
                },
            ],
            inner: Some("value"),
            parser: |v: &str| {
                if "let".starts_with(v) {
                    Some(String::new())
                } else {
                    None
                }
            },
            renderer: |model: &Model, value: &Node, path: &Path| {
                let name = model.view_child(value, "name", &path);
                let value = model.view_child(value, "value", &path);
                html! {
                    <span>{ "let" }{ name }{ "=" }{ value }</span>
                }
            },
        },
        Kind {
            name: "function_call",
            fields: &[
                Field {
                    name: "expression",
                    kind: RUST_EXPRESSION,
                    multiplicity: Multiplicity::Single,
                },
                Field {
                    name: "arguments",
                    kind: RUST_EXPRESSION,
                    multiplicity: Multiplicity::Repeated,
                },
            ],
            inner: Some("expression"),
            parser: |v: &str| {
                if "function_call".starts_with(v) || "(".starts_with(v) {
                    Some(String::new())
                } else {
                    None
                }
            },
            renderer: |model: &Model, value: &Node, path: &Path| {
                let expression = model.view_child(value, "expression", path);
                let (args_head, args) = model.view_children(value, "arguments", path);
                let args = args.into_iter().intersperse(html! {{ "," }});
                html! {
                    <span>
                    { expression }
                    { "(" }{ args_head }{ for args }{ ")" }
                    </span>
                }
            },
        },
        Kind {
            name: "struct",
            fields: &[
                Field {
                    name: "name",
                    kind: &["identifier"],
                    multiplicity: Multiplicity::Single,
                },
                Field {
                    name: "fields",
                    kind: &["struct_field"],
                    multiplicity: Multiplicity::Repeated,
                },
            ],
            inner: None,
            parser: |v: &str| {
                if "struct".starts_with(v) {
                    Some(String::new())
                } else {
                    None
                }
            },
            renderer: |model: &Model, value: &Node, path: &Path| {
                let label = model.view_child(value, "name", &path);
                let (fields_head, fields) = model.view_children(value, "fields", &path);
                let fields = fields.into_iter().map(|v| {
                    html! {
                        <div class="indent">{ v }{ "," }</div>
                    }
                });

                html! {
                    <span>
                    <span class="keyword">{ "struct" }</span>{ label }
                    { "{" }{ fields_head }{ for fields }{ "}" }
                    </span>
                }
            },
        },
        Kind {
            name: "struct_field",
            fields: &[
                Field {
                    name: "name",
                    kind: &["identifier"],
                    multiplicity: Multiplicity::Single,
                },
                Field {
                    name: "type", // Type
                    kind: RUST_TYPE,
                    multiplicity: Multiplicity::Single,
                },
            ],
            inner: None,
            parser: |v: &str| {
                if "struct_field".starts_with(v) {
                    Some(String::new())
                } else {
                    None
                }
            },
            renderer: |model: &Model, value: &Node, path: &Path| {
                let name = model.view_child(value, "name", &path);
                let type_ = model.view_child(value, "type", &path);
                html! {
                    <span>
                    { name }{ ":" }{ type_ }
                    </span>
                }
            },
        },
        // https://doc.rust-lang.org/nightly/reference/items/enumerations.html
        Kind {
            name: "enum",
            fields: &[
                Field {
                    name: "identifier",
                    kind: &["identifier"],
                    multiplicity: Multiplicity::Single,
                },
                Field {
                    name: "variants",
                    // enum_variant
                    kind: &["identifier"],
                    multiplicity: Multiplicity::Repeated,
                },
            ],
            inner: None,
            parser: |v: &str| {
                if "enum".starts_with(v) {
                    Some(String::new())
                } else {
                    None
                }
            },
            renderer: |model: &Model, value: &Node, path: &Path| {
                let identifier = model.view_child(value, "identifier", &path);
                let (variants_head, variants) = model.view_children(value, "variants", &path);
                let variants = variants.into_iter().map(|v| {
                    html! {
                        <div class="indent">{ v }{ "," }</div>
                    }
                });

                html! {
                    <span>
                    <span class="keyword">{ "enum" }</span>{ identifier }
                    { "{" }{ variants_head }{ for variants }{ "}" }
                    </span>
                }
            },
        },
        Kind {
            name: "enum_variant",
            fields: &[Field {
                name: "identifier",
                kind: &["identifier"],
                multiplicity: Multiplicity::Single,
            }],
            inner: None,
            parser: |v: &str| {
                if "enum_variant".starts_with(v) {
                    Some(String::new())
                } else {
                    None
                }
            },
            renderer: |model: &Model, value: &Node, path: &Path| {
                let label = model.view_child(value, "name", &path);
                let (variants_head, variants) = model.view_children(value, "variants", &path);
                let variants = variants.into_iter().map(|v| {
                    html! {
                        <div class="indent">{ v }{ "," }</div>
                    }
                });

                html! {
                    <span>
                    <span class="keyword">{ "enum" }</span>{ label }
                    { "{" }{ variants_head }{ for variants }{ "}" }
                    </span>
                }
            },
        },
        Kind {
            name: "markdown_fragment",
            fields: &[Field {
                name: "items",
                kind: &[
                    "markdown_paragraph",
                    "markdown_heading",
                    "markdown_code",
                    "markdown_quote",
                    "markdown_list",
                ],
                multiplicity: Multiplicity::Repeated,
            }],
            inner: Some("items"),
            parser: |v: &str| {
                if "markdown_fragment".starts_with(v) {
                    Some(String::new())
                } else {
                    None
                }
            },
            renderer: |model: &Model, value: &Node, path: &Path| {
                let (items_head, items) = model.view_children(value, "items", &path);
                let items = items.into_iter().map(|v| {
                    html! {
                        <div>{ v }</div>
                    }
                });
                html! {
                    <span>
                    { items_head }
                    { for items }
                    </span>
                }
            },
        },
        Kind {
            name: "markdown_heading",
            fields: &[
                Field {
                    name: "level",
                    // XXX
                    kind: &[],
                    multiplicity: Multiplicity::Single,
                },
                Field {
                    name: "text",
                    // XXX
                    kind: &["identifier"],
                    multiplicity: Multiplicity::Single,
                },
            ],
            inner: Some("text"),
            parser: |v: &str| {
                if "markdown_heading".starts_with(v) || "#".starts_with(v) {
                    Some(String::new())
                } else {
                    None
                }
            },
            renderer: |model: &Model, value: &Node, path: &Path| {
                let level = model.view_child(value, "level", &path);
                let text = model.view_child(value, "text", &path);
                html! {
                    <span>
                    { "#" }{ level}{ text }
                    </span>
                }
            },
        },
        Kind {
            name: "markdown_list",
            fields: &[Field {
                name: "items",
                kind: &["markdown_paragraph"],
                multiplicity: Multiplicity::Repeated,
            }],
            inner: Some("items"),
            parser: |v: &str| {
                if "markdown_list".starts_with(v) || "-".starts_with(v) {
                    Some(String::new())
                } else {
                    None
                }
            },
            renderer: |model: &Model, value: &Node, path: &Path| {
                let (items_head, items) = model.view_children(value, "items", &path);
                let items = items.into_iter().map(|v| {
                    html! {
                        <li>{ v }</li>
                    }
                });
                html! {
                    <span>
                        { items_head }
                        <ul>
                            { for items }
                        </ul>
                    </span>
                }
            },
        },
    ],
};

// Parse only value.
type Parser = fn(&str) -> Option<String>;
type Renderer = fn(&Model, &Node, &Path) -> Html;

pub struct Schema {
    pub kinds: &'static [Kind],
}

impl Schema {
    pub fn get_kind(&self, kind: &str) -> Option<&Kind> {
        self.kinds.iter().find(|k| k.name == kind)
    }
}

pub struct Kind {
    pub name: &'static str,
    pub fields: &'static [Field],
    pub inner: Option<&'static str>,
    pub renderer: Renderer,
    pub parser: Parser,
    // pub aliases: &'static [&'static str],
}

impl Kind {
    pub fn get_field(&self, field: &str) -> Option<&Field> {
        self.fields.iter().find(|f| f.name == field)
    }
}

pub struct Field {
    pub name: &'static str,
    pub kind: &'static [&'static str],
    pub multiplicity: Multiplicity,
}

pub enum Multiplicity {
    // Required -- show hole if not present
    // Optional -- hide if not present
    Single,
    Repeated,
}
