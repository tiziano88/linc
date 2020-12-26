use crate::types::{Model, Node, Path};
use itertools::Itertools;
use yew::{html, Html};

// https://doc.rust-lang.org/stable/reference/expressions.html
const RUST_EXPRESSION: &[&str] = &[
    // XXX
    "rust_identifier",
    "rust_field_access",
    "rust_function_call",
    "rust_tuple_expression",
    "rust_if",
    "rust_match",
    "rust_operator",
    "rust_string_literal",
    "rust_number_literal",
    "rust_bool_literal_false",
    "rust_bool_literal_true",
];

// https://doc.rust-lang.org/stable/reference/items.html
const RUST_ITEM: &[&str] = &[
    "rust_constant",
    "rust_enum",
    "rust_function_definition",
    "rust_struct",
];

const RUST_PATTERN: &[&str] = &[
    "rust_literal_pattern",
    "rust_identifier_pattern",
    "rust_wildcard_pattern",
    "rust_rest_pattern",
    "rust_reference_pattern",
    "rust_struct_pattern",
    "rust_tuple_struct_pattern",
    "rust_tuple_pattern",
    "rust_grouped_pattern",
    "rust_path_pattern",
    "rust_macro_invocation",
];

// https://doc.rust-lang.org/stable/reference/types.html#type-expressions
const RUST_TYPE: &[&str] = &[
    "rust_type_path",
    "rust_array_type",
    "rust_reference_type",
    "rust_simple_path",
    "rust_slice_type",
    "rust_tuple_type",
    "rust_primitive_type",
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
            parser: |v: &str| vec!["".to_string()],
            renderer: |model: &Model, node: &Node, path: &Path| {
                let (items_head, items) = model.view_children(node, "items", path);
                let items = items.into_iter().map(|b| {
                    html! {
                        <div>{ b }</div>
                    }
                });
                html! {
                    <div>
                    <div class="fragment-type">{ "rust" }</div>
                    { for items }
                    { items_head }
                    </div>
                }
            },
        },
        Kind {
            name: "rust_tuple_type",
            fields: &[Field {
                name: "components",
                kind: RUST_TYPE,
                multiplicity: Multiplicity::Repeated,
            }],
            inner: Some("components"),
            parser: |v: &str| vec!["(".to_string()],
            renderer: |model: &Model, node: &Node, path: &Path| {
                let (components_head, components) = model.view_children(node, "components", path);
                let components = components
                    .into_iter()
                    .intersperse(html! { <span>{ "," }</span>});
                html! {
                    <span>
                      <span>{ "(" }</span>
                      { for components }{ components_head }
                      <span>{ ")" }</span>
                    </span>
                }
            },
        },
        Kind {
            name: "rust_primitive_type",
            fields: &[],
            inner: None,
            parser: |v: &str| {
                vec![
                    "bool".to_string(),
                    "char".to_string(),
                    "str".to_string(),
                    "u8".to_string(),
                    "u16".to_string(),
                    "u32".to_string(),
                    "u64".to_string(),
                    "u128".to_string(),
                    "i8".to_string(),
                    "i16".to_string(),
                    "i32".to_string(),
                    "i64".to_string(),
                    "i128".to_string(),
                    "f32".to_string(),
                    "f64".to_string(),
                    "usize".to_string(),
                    "isize".to_string(),
                ]
            },
            renderer: |model: &Model, node: &Node, path: &Path| {
                html! {
                    <span class="keyword">{ node.value.clone() }</span>
                }
            },
        },
        Kind {
            name: "rust_path_ident_segment",
            fields: &[Field {
                name: "segments",
                kind: RUST_TYPE,
                multiplicity: Multiplicity::Repeated,
            }],
            inner: Some("segments"),
            parser: |v: &str| {
                vec![
                    v.to_string(),
                    "super".to_string(),
                    "self".to_string(),
                    "Self".to_string(),
                    "crate".to_string(),
                    "$crate".to_string(),
                ]
            },
            renderer: |model: &Model, node: &Node, path: &Path| {
                html! {
                    <span>
                    { node.value.clone() }
                    </span>
                }
            },
        },
        Kind {
            name: "rust_type_path",
            fields: &[Field {
                name: "segments",
                kind: RUST_TYPE,
                multiplicity: Multiplicity::Repeated,
            }],
            inner: Some("segments"),
            parser: |v: &str| vec!["&".to_string()],
            renderer: |model: &Model, node: &Node, path: &Path| {
                let (segments_head, segments) = model.view_children(node, "segments", path);
                let segments = segments
                    .into_iter()
                    .intersperse(html! { <span>{ "::" }</span>});
                html! {
                    <span>
                    { for segments }{ segments_head }
                    </span>
                }
            },
        },
        Kind {
            name: "rust_reference_type",
            fields: &[
                Field {
                    name: "type",
                    kind: RUST_TYPE,
                    multiplicity: Multiplicity::Single,
                },
                Field {
                    name: "mutable",
                    kind: &["rust_bool"],
                    multiplicity: Multiplicity::Single,
                },
                Field {
                    name: "lifetime",
                    // XXX
                    kind: &["rust_bool"],
                    multiplicity: Multiplicity::Single,
                },
            ],
            inner: Some("type"),
            parser: |v: &str| vec!["&".to_string()],
            renderer: |model: &Model, node: &Node, path: &Path| {
                let type_ = model.view_child(node, "type", path);
                html! {
                    <span>
                    { "&" }{ type_ }
                    </span>
                }
            },
        },
        Kind {
            name: "rust_constant",
            fields: &[
                Field {
                    name: "identifier",
                    kind: &["rust_identifier"],
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
            parser: |v: &str| vec!["const".to_string()],
            renderer: |model: &Model, node: &Node, path: &Path| {
                let identifier = model.view_child(node, "identifier", path);
                let type_ = model.view_child(node, "type", path);
                let expression = model.view_child(node, "expression", path);
                html! {
                    <span>
                    { "const" }{ identifier }{ ":" }{ type_ }{ "=" }{ expression }{ ";" }
                    </span>
                }
            },
        },
        Kind {
            name: "rust_block",
            fields: &[Field {
                name: "statements",
                kind: RUST_EXPRESSION,
                multiplicity: Multiplicity::Repeated,
            }],
            inner: Some("statements"),
            parser: |v: &str| vec!["{".to_string()],
            renderer: |model: &Model, node: &Node, path: &Path| {
                let (statements_head, statements) = model.view_children(node, "statements", path);
                let statements = statements.into_iter().map(|v| {
                    html! {
                        <div class="indent">{ v }{ ";" }</div>
                    }
                });

                html! {
                    <span>
                    { "{" }{ for statements }{ statements_head }{ "}" }
                    </span>
                }
            },
        },
        Kind {
            name: "rust_match",
            fields: &[Field {
                name: "match_arms",
                kind: &["rust_match_arm"],
                multiplicity: Multiplicity::Repeated,
            }],
            inner: Some("match_arms"),
            parser: |v: &str| vec!["match_arm".to_string()],
            renderer: |model: &Model, node: &Node, path: &Path| {
                let expression = model.view_child(node, "expression", path);
                let (match_arms_head, match_arms) = model.view_children(node, "match_arms", path);
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
                        { for match_arms }
                        { match_arms_head }
                        <div>
                            { "}" }
                        </div>
                    </span>
                }
            },
        },
        Kind {
            name: "rust_match_arm",
            fields: &[
                Field {
                    name: "patterns",
                    kind: RUST_PATTERN,
                    multiplicity: Multiplicity::Repeated,
                },
                Field {
                    name: "guard",
                    kind: RUST_EXPRESSION,
                    multiplicity: Multiplicity::Single,
                },
                Field {
                    name: "expression",
                    kind: RUST_EXPRESSION,
                    multiplicity: Multiplicity::Single,
                },
            ],
            inner: Some("match_arms"),
            parser: |v: &str| vec!["match_arm".to_string()],
            renderer: |model: &Model, node: &Node, path: &Path| {
                let (patterns_head, patterns) = model.view_children(node, "patterns", path);
                let patterns = patterns.into_iter().intersperse(html! {
                        <span>{ "|" }</span>

                });
                let guard = model.view_child(node, "guard", path);
                let expression = model.view_child(node, "expression", path);
                html! {
                    <span>
                        <span>{ for patterns }{ patterns_head }</span>
                        <span>{ "if" }{ guard }</span>
                        <span>{ "=>" }</span>
                        <span>{ expression }</span>
                    </span>
                }
            },
        },
        Kind {
            name: "rust_if",
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
            parser: |v: &str| vec!["if".to_string()],
            renderer: |model: &Model, node: &Node, path: &Path| {
                let condition = model.view_child(node, "condition", path);
                let true_body = model.view_child(node, "true_body", path);
                let false_body = model.view_child(node, "false_body", path);
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
            name: "rust_string_literal",
            fields: &[],
            inner: None,
            parser: |v: &str| vec![v.to_string()],
            renderer: |model: &Model, node: &Node, path: &Path| {
                html! {
                    <span>
                    { "\"" }{ node.value.clone() }{ "\"" }
                    </span>
                }
            },
        },
        Kind {
            name: "rust_number_literal",
            fields: &[],
            inner: None,
            // TODO: regex
            parser: |v: &str| {
                if v.parse::<i32>().is_ok() {
                    vec![v.to_string()]
                } else {
                    vec![]
                }
            },
            renderer: |model: &Model, node: &Node, path: &Path| {
                html! {
                    <span>
                    { node.value.clone() }
                    </span>
                }
            },
        },
        Kind {
            name: "rust_bool_literal",
            fields: &[],
            inner: None,
            // TODO: regex
            parser: |v: &str| vec!["false".to_string(), "true".to_string()],
            renderer: |model: &Model, node: &Node, path: &Path| {
                html! {
                    <span class="literal">{ node.value.clone() }</span>
                }
            },
        },
        Kind {
            name: "rust_field_access",
            fields: &[
                Field {
                    name: "object",
                    kind: RUST_EXPRESSION,
                    multiplicity: Multiplicity::Single,
                },
                Field {
                    name: "field",
                    kind: &["rust_identifier"],
                    multiplicity: Multiplicity::Single,
                },
            ],
            inner: Some("object"),
            parser: |v: &str| vec![".".to_string()],
            renderer: |model: &Model, node: &Node, path: &Path| {
                let object = model.view_child(node, "object", &path);
                let field = model.view_child(node, "field", &path);
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
            name: "rust_simple_path",
            fields: &[Field {
                name: "segments",
                kind: &["rust_path_ident_segment"],
                multiplicity: Multiplicity::Repeated,
            }],
            inner: Some("segments"),
            parser: |v: &str| vec!["::".to_string()],
            renderer: |model: &Model, node: &Node, path: &Path| {
                let (segments_head, segments) = model.view_children(node, "segments", &path);
                let segments = segments.into_iter().intersperse(html! {{ "::" }});
                html! {
                    <span>{ for segments }{ segments_head }</span>
                }
            },
        },
        Kind {
            name: "rust_identifier",
            fields: &[],
            inner: None,
            parser: |v: &str| {
                if v.starts_with(|c: char| c.is_alphabetic()) && !v.contains(' ') {
                    vec![v.to_string()]
                } else {
                    vec![]
                }
            },
            renderer: |model: &Model, node: &Node, path: &Path| {
                html! {
                    <span class="identifier">{ node.value.clone() }</span>
                }
            },
        },
        Kind {
            name: "rust_crate",
            fields: &[],
            inner: None,
            parser: |v: &str| vec!["crate".to_string()],
            renderer: |model: &Model, node: &Node, path: &Path| {
                html! {
                    <span class="keyword">{ "crate" }</span>
                }
            },
        },
        Kind {
            name: "rust_binary_operator",
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
                vec![
                    "==".to_string(),
                    "+".to_string(),
                    "+=".to_string(),
                    "-".to_string(),
                    "-=".to_string(),
                    "<<".to_string(),
                    ">>".to_string(),
                    "<".to_string(),
                    ">".to_string(),
                    "&&".to_string(),
                    "||".to_string(),
                    "&".to_string(),
                    "|".to_string(),
                    "^".to_string(),
                ]
            },
            renderer: |model: &Model, node: &Node, path: &Path| {
                let operator = model.view_child(node, "operator", &path);
                let left = model.view_child(node, "left", &path);
                let right = model.view_child(node, "right", &path);
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
            name: "rust_function_definition",
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
                    kind: &["rust_identifier"],
                    multiplicity: Multiplicity::Single,
                },
                Field {
                    name: "parameters",
                    kind: &["rust_function_parameter"],
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
            parser: |v: &str| vec!["fn".to_string()],
            renderer: |model: &Model, node: &Node, path: &Path| {
                let comment = model.view_child(node, "comment", path);
                let identifier = model.view_child(node, "identifier", path);
                let (parameters_head, parameters) = model.view_children(node, "parameters", path);
                let parameters = parameters.into_iter().intersperse(html! {{ "," }});
                let body = model.view_child(node, "body", path);
                let return_type = model.view_child(node, "return_type", path);
                // let async_ = self.view_child(&v, "async", &path);
                // let pub_ = self.view_child(&v, "pub", &path);

                html! {
                    <span>
                        <div>{ "//" }{ comment }</div>
                        // { pub_ }
                        <div>
                          <span class="keyword">{ "fn" }</span>{ identifier }
                          { "(" }{ for parameters }{ parameters_head }{ ")" }
                          { "->" }{ return_type }{ "{" }
                        </div>
                        <div class="indent">{ body }</div>
                        { "}" }
                    </span>
                }
            },
        },
        Kind {
            name: "rust_function_parameter",
            fields: &[
                Field {
                    name: "pattern",
                    kind: &["rust_identifier"],
                    multiplicity: Multiplicity::Single,
                },
                Field {
                    name: "type",
                    kind: RUST_TYPE,
                    multiplicity: Multiplicity::Single,
                },
            ],
            inner: None,
            parser: |v: &str| vec!["param".to_string()],
            renderer: |model: &Model, node: &Node, path: &Path| {
                let pattern = model.view_child(node, "pattern", path);
                let type_ = model.view_child(node, "type", path);
                html! {
                    <span>
                    { pattern }{ ":" }{ type_ }
                    </span>
                }
            },
        },
        // https://doc.rust-lang.org/nightly/reference/statements.html#let-statements
        Kind {
            name: "rust_let",
            fields: &[
                Field {
                    name: "pattern",
                    kind: RUST_PATTERN,
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
            parser: |v: &str| vec!["let".to_string()],
            renderer: |model: &Model, node: &Node, path: &Path| {
                let pattern = model.view_child(node, "pattern", path);
                let value = model.view_child(node, "value", path);
                html! {
                    <span>{ "let" }{ pattern }{ "=" }{ value }</span>
                }
            },
        },
        Kind {
            name: "rust_function_call",
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
            parser: |v: &str| vec!["(".to_string()],
            renderer: |model: &Model, node: &Node, path: &Path| {
                let expression = model.view_child(node, "expression", path);
                let (args_head, args) = model.view_children(node, "arguments", path);
                let args = args.into_iter().intersperse(html! {{ "," }});
                html! {
                    <span>
                    { expression }
                    { "(" }{ for args }{ args_head }{ ")" }
                    </span>
                }
            },
        },
        Kind {
            name: "rust_tuple_expression",
            fields: &[Field {
                name: "elements",
                kind: RUST_EXPRESSION,
                multiplicity: Multiplicity::Repeated,
            }],
            inner: Some("elements"),
            parser: |v: &str| vec!["(".to_string()],
            renderer: |model: &Model, node: &Node, path: &Path| {
                let (elements_head, elements) = model.view_children(node, "elements", path);
                let elements = elements.into_iter().intersperse(html! {{ "," }});
                html! {
                    <span>
                    { "(" }{ for elements }{ elements_head }{ ")" }
                    </span>
                }
            },
        },
        Kind {
            name: "rust_struct",
            fields: &[
                Field {
                    name: "identifier",
                    kind: &["rust_identifier"],
                    multiplicity: Multiplicity::Single,
                },
                Field {
                    name: "fields",
                    kind: &["rust_struct_field"],
                    multiplicity: Multiplicity::Repeated,
                },
            ],
            inner: None,
            parser: |v: &str| vec!["struct".to_string()],
            renderer: |model: &Model, node: &Node, path: &Path| {
                let identifier = model.view_child(node, "identifier", path);
                let (fields_head, fields) = model.view_children(node, "fields", path);
                let fields = fields.into_iter().map(|v| {
                    html! {
                        <div class="indent">{ v }{ "," }</div>
                    }
                });

                html! {
                    <span>
                    <span class="keyword">{ "struct" }</span>{ identifier }
                    { "{" }{ for fields }{ fields_head }{ "}" }
                    </span>
                }
            },
        },
        Kind {
            name: "rust_struct_field",
            fields: &[
                Field {
                    name: "identifier",
                    kind: &["rust_identifier"],
                    multiplicity: Multiplicity::Single,
                },
                Field {
                    name: "type", // Type
                    kind: RUST_TYPE,
                    multiplicity: Multiplicity::Single,
                },
            ],
            inner: None,
            parser: |v: &str| vec!["struct_field".to_string()],
            renderer: |model: &Model, node: &Node, path: &Path| {
                let identifier = model.view_child(node, "identifier", path);
                let type_ = model.view_child(node, "type", path);
                html! {
                    <span>
                    { identifier }{ ":" }{ type_ }
                    </span>
                }
            },
        },
        // https://doc.rust-lang.org/nightly/reference/items/enumerations.html
        Kind {
            name: "rust_enum",
            fields: &[
                Field {
                    name: "identifier",
                    kind: &["rust_identifier"],
                    multiplicity: Multiplicity::Single,
                },
                Field {
                    name: "variants",
                    // enum_variant
                    kind: &["rust_identifier"],
                    multiplicity: Multiplicity::Repeated,
                },
            ],
            inner: None,
            parser: |v: &str| vec!["enum".to_string()],
            renderer: |model: &Model, node: &Node, path: &Path| {
                let identifier = model.view_child(node, "identifier", path);
                let (variants_head, variants) = model.view_children(node, "variants", path);
                let variants = variants.into_iter().map(|v| {
                    html! {
                        <div class="indent">{ v }{ "," }</div>
                    }
                });

                html! {
                    <span>
                    <span class="keyword">{ "enum" }</span>{ identifier }
                    { "{" }{ for variants }{ variants_head }{ "}" }
                    </span>
                }
            },
        },
        Kind {
            name: "rust_enum_variant",
            fields: &[Field {
                name: "identifier",
                kind: &["rust_identifier"],
                multiplicity: Multiplicity::Single,
            }],
            inner: None,
            parser: |v: &str| vec!["enum_variant".to_string()],
            renderer: |model: &Model, node: &Node, path: &Path| {
                let label = model.view_child(node, "identifier", path);
                let (variants_head, variants) = model.view_children(node, "variants", path);
                let variants = variants.into_iter().map(|v| {
                    html! {
                        <div class="indent">{ v }{ "," }</div>
                    }
                });

                html! {
                    <span>
                    <span class="keyword">{ "enum" }</span>{ label }
                    { "{" }{ for variants }{ variants_head }{ "}" }
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
            parser: |v: &str| vec!["markdown_fragment".to_string()],
            renderer: |model: &Model, node: &Node, path: &Path| {
                let (items_head, items) = model.view_children(node, "items", path);
                let items = items.into_iter().map(|v| {
                    html! {
                        <div>{ v }</div>
                    }
                });
                html! {
                    <div>
                        <div class="fragment-type">{ "markdown" }</div>
                        { for items }
                        { items_head }
                    </div>
                }
            },
        },
        Kind {
            name: "markdown_paragraph",
            fields: &[],
            inner: None,
            parser: |v: &str| vec![v.to_string()],
            renderer: |model: &Model, node: &Node, path: &Path| {
                html! {
                    <span>
                    { node.value.clone() }
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
                    kind: &["rust_identifier"],
                    multiplicity: Multiplicity::Single,
                },
            ],
            inner: Some("text"),
            parser: |v: &str| vec!["#".to_string()],
            renderer: |model: &Model, node: &Node, path: &Path| {
                let level = model.view_child(node, "level", path);
                let text = model.view_child(node, "text", path);
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
            parser: |v: &str| vec!["-".to_string()],
            renderer: |model: &Model, node: &Node, path: &Path| {
                let (items_head, items) = model.view_children(node, "items", path);
                let items = items.into_iter().map(|v| {
                    html! {
                        <li>{ v }</li>
                    }
                });
                html! {
                    <span>
                        <ul class="list-disc">
                            { for items }
                            <li>{ items_head }</li>
                        </ul>
                    </span>
                }
            },
        },
    ],
};

// Generate valid values.
type Parser = fn(&str) -> Vec<String>;
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
    /* pub aliases: &'static [&'static str],
     * TODO: create list of elements, and then have validator function to filter them and
     * provide feedback if not matching, without hiding the entry. Or return one of three values
     * from parse: ok, hide, invalid */
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