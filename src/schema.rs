use crate::types::{Model, Node, Path, Ref};
use itertools::Itertools;
use yew::{html, Html};

// https://doc.rust-lang.org/stable/reference/expressions.html
const RUST_EXPRESSION: &[&str] = &[
    "rust_field_access",
    "rust_function_call",
    "rust_tuple_expression",
    "rust_if",
    "rust_match",
    "rust_operator",
    "rust_comparison_expression",
    "rust_bool_literal",
    "rust_number_literal",
    "rust_identifier",
    "rust_string_literal",
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
    "rust_wildcard_pattern",
    "rust_rest_pattern",
    "rust_reference_pattern",
    "rust_struct_pattern",
    "rust_tuple_struct_pattern",
    "rust_tuple_pattern",
    "rust_grouped_pattern",
    "rust_path_pattern",
    "rust_macro_invocation",
    "rust_identifier_pattern",
];

// https://doc.rust-lang.org/stable/reference/types.html#type-expressions
const RUST_TYPE: &[&str] = &[
    "rust_type_path",
    "rust_array_type",
    "rust_reference_type",
    "rust_slice_type",
    "rust_tuple_type",
    "rust_primitive_type",
];

const RUST_VISIBILITY: &[&str] = &[
    "rust_visibility_pub",
    "rust_visibility_pub_crate",
    "rust_visibility_pub_self",
    "rust_visibility_pub_super",
    "rust_visibility_pub_in",
];

// const RUST_BOOL_LITERAL: &[&str] = &["rust_bool_literal_false", "rust_bool_literal_true"];

const RUST_PATH_IDENT_SEGMENT: &[&str] = &[
    "rust_path_ident_segment_super",
    "rust_path_ident_segment_self",
    "rust_path_ident_segment_self_upper",
    "rust_path_ident_segment_crate",
    "rust_path_ident_segment_crate_dollar",
    "rust_identifier",
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


trait K: Sized {
    const NAME: &'static str;
    fn fields() -> &'static [Field];
    fn generate() -> Vec<String>;
    // fn parse(v: &str) -> Option<Self>;
    fn render(&self, model: &Model, path: &Path) -> Html;
    // fn decode(&self) -> Node,
    // fn encode(&self) -> Node,
}

pub struct RustVisibility;

impl K for RustVisibility {
    const NAME: &'static str = "rust_visibility";

    fn render(&self, model: &Model, path: &Path) -> Html {
        todo!()
    }

    fn generate() -> Vec<String> {
        vec![
            "pub".to_string(),
            "pub_crate".to_string(),
            "pub_self".to_string(),
            "pub_super".to_string(),
            "pub_in".to_string(),
        ]
    }

    fn fields() -> &'static [Field] {
        todo!()
    }
}
*/

pub const SCHEMA: Schema = Schema {
    kinds: &[
        Kind {
            name: "rust_fragment",
            value: KindValue::Struct {
                fields: &[Field {
                    name: "items",
                    kind: RUST_ITEM,
                    multiplicity: Multiplicity::Repeated,
                }],
                inner: None,
                parser: |v: &str| vec![Ok("".to_string())],
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
        },
        Kind {
            name: "rust_tuple_type",
            value: KindValue::Struct {
                fields: &[Field {
                    name: "components",
                    kind: RUST_TYPE,
                    multiplicity: Multiplicity::Repeated,
                }],
                inner: Some("components"),
                parser: |v: &str| vec![Ok("(".to_string())],
                renderer: |model: &Model, node: &Node, path: &Path| {
                    let (components_head, components) =
                        model.view_children(node, "components", path);
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
        },
        Kind {
            name: "rust_primitive_type",
            value: KindValue::Struct {
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
                    .into_iter()
                    .map(Ok)
                    .collect()
                },
                renderer: |model: &Model, node: &Node, path: &Path| {
                    html! {
                        <span class="keyword">{ node.value.clone() }</span>
                    }
                },
            },
        },
        Kind {
            name: "rust_path_ident_segment_super",
            value: KindValue::Struct {
                fields: &[],
                inner: None,
                parser: |v: &str| vec![Ok("super".to_string())],
                renderer: |model: &Model, node: &Node, path: &Path| {
                    html! {
                        <span class="keyword">{ "super" }</span>
                    }
                },
            },
        },
        Kind {
            name: "rust_path_ident_segment_self",
            value: KindValue::Struct {
                fields: &[],
                inner: None,
                parser: |v: &str| vec![Ok("self".to_string())],
                renderer: |model: &Model, node: &Node, path: &Path| {
                    html! {
                        <span class="keyword">{ "self" }</span>
                    }
                },
            },
        },
        Kind {
            name: "rust_path_ident_segment_self_upper",
            value: KindValue::Struct {
                fields: &[],
                inner: None,
                parser: |v: &str| vec![Ok("Self".to_string())],
                renderer: |model: &Model, node: &Node, path: &Path| {
                    html! {
                        <span class="keyword">{ "Self" }</span>
                    }
                },
            },
        },
        Kind {
            name: "rust_path_ident_segment_crate",
            value: KindValue::Struct {
                fields: &[],
                inner: None,
                parser: |v: &str| vec![Ok("crate".to_string())],
                renderer: |model: &Model, node: &Node, path: &Path| {
                    html! {
                        <span class="keyword">{ "crate" }</span>
                    }
                },
            },
        },
        Kind {
            name: "rust_path_ident_segment_crate_dollar",
            value: KindValue::Struct {
                fields: &[],
                inner: None,
                parser: |v: &str| vec![Ok("$crate".to_string())],
                renderer: |model: &Model, node: &Node, path: &Path| {
                    html! {
                        <span class="keyword">{ "$crate" }</span>
                    }
                },
            },
        },
        Kind {
            name: "rust_visibility_pub",
            value: KindValue::Struct {
                fields: &[],
                inner: None,
                parser: |v: &str| vec![Ok("pub".to_string())],
                renderer: |model: &Model, node: &Node, path: &Path| {
                    html! {
                        <span class="keyword">{ "pub" }</span>
                    }
                },
            },
        },
        Kind {
            name: "rust_visibility_pub_crate",
            value: KindValue::Struct {
                fields: &[],
                inner: None,
                parser: |v: &str| vec![Ok("pub_crate".to_string())],
                renderer: |model: &Model, node: &Node, path: &Path| {
                    html! {
                        <span class="keyword">{ "pub(crate)" }</span>
                    }
                },
            },
        },
        Kind {
            name: "rust_visibility_pub_self",
            value: KindValue::Struct {
                fields: &[],
                inner: None,
                parser: |v: &str| vec![Ok("pub_self".to_string())],
                renderer: |model: &Model, node: &Node, path: &Path| {
                    html! {
                        <span class="keyword">{ "pub(self)" }</span>
                    }
                },
            },
        },
        Kind {
            name: "rust_visibility_pub_in",
            value: KindValue::Struct {
                fields: &[Field {
                    name: "path",
                    kind: &["rust_simple_path"],
                    multiplicity: Multiplicity::Single,
                }],
                inner: None,
                parser: |v: &str| vec![Ok("pub_in".to_string())],
                renderer: |model: &Model, node: &Node, path: &Path| {
                    let path = model.view_child(node, "path", &path);
                    html! {
                        <span>
                            <span class="keyword">{ "pub" }</span>
                            <span>{ "(" }</span>
                            <span class="keyword">{ "in" }</span>
                            { path }
                            <span>{ ")" }</span>
                        </span>
                    }
                },
            },
        },
        Kind {
            name: "rust_type_path",
            value: KindValue::Struct {
                fields: &[Field {
                    name: "segments",
                    kind: RUST_TYPE,
                    multiplicity: Multiplicity::Repeated,
                }],
                inner: Some("segments"),
                parser: |v: &str| vec![Ok("::".to_string())],
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
        },
        Kind {
            name: "rust_reference_type",
            value: KindValue::Struct {
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
                parser: |v: &str| vec![Ok("&".to_string())],
                renderer: |model: &Model, node: &Node, path: &Path| {
                    let type_ = model.view_child(node, "type", path);
                    html! {
                        <span>
                        { "&" }{ type_ }
                        </span>
                    }
                },
            },
        },
        Kind {
            name: "rust_constant",
            value: KindValue::Struct {
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
                parser: |v: &str| vec![Ok("const".to_string())],
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
        },
        Kind {
            name: "rust_block",
            value: KindValue::Struct {
                fields: &[Field {
                    name: "statements",
                    kind: RUST_EXPRESSION,
                    multiplicity: Multiplicity::Repeated,
                }],
                inner: Some("statements"),
                parser: |v: &str| vec![Ok("{".to_string())],
                renderer: |model: &Model, node: &Node, path: &Path| {
                    let (statements_head, statements) =
                        model.view_children(node, "statements", path);
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
        },
        Kind {
            name: "rust_match",
            value: KindValue::Struct {
                fields: &[Field {
                    name: "match_arms",
                    kind: &["rust_match_arm"],
                    multiplicity: Multiplicity::Repeated,
                }],
                inner: Some("match_arms"),
                parser: |v: &str| vec![Ok("match_arm".to_string())],
                renderer: |model: &Model, node: &Node, path: &Path| {
                    let expression = model.view_child(node, "expression", path);
                    let (match_arms_head, match_arms) =
                        model.view_children(node, "match_arms", path);
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
        },
        Kind {
            name: "rust_match_arm",
            value: KindValue::Struct {
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
                parser: |v: &str| vec![Ok("match_arm".to_string())],
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
        },
        Kind {
            name: "rust_if",
            value: KindValue::Struct {
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
                parser: |v: &str| vec![Ok("if".to_string())],
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
        },
        Kind {
            name: "rust_string_literal",
            value: KindValue::Struct {
                fields: &[],
                inner: None,
                parser: |v: &str| vec![Ok(v.to_string())],
                renderer: |model: &Model, node: &Node, path: &Path| {
                    html! {
                        <span>
                        { "\"" }{ node.value.clone() }{ "\"" }
                        </span>
                    }
                },
            },
        },
        Kind {
            name: "rust_number_literal",
            value: KindValue::Struct {
                fields: &[],
                inner: None,
                // TODO: regex
                parser: |v: &str| {
                    vec![if v.parse::<i32>().is_ok() {
                        Ok(v.to_string())
                    } else {
                        Err("not a valid number".to_string())
                    }]
                },
                renderer: |model: &Model, node: &Node, path: &Path| {
                    html! {
                        <span>
                        { node.value.clone() }
                        </span>
                    }
                },
            },
        },
        Kind {
            name: "rust_bool_literal",
            value: KindValue::Enum {
                variants: &["rust_bool_literal_false", "rust_bool_literal_true"],
            },
        },
        Kind {
            name: "rust_bool_literal_false",
            value: KindValue::Struct {
                fields: &[],
                inner: None,
                parser: |v: &str| vec![Ok("false".to_string())],
                renderer: |model: &Model, node: &Node, path: &Path| {
                    html! {
                        <span class="literal">{ "false" }</span>
                    }
                },
            },
        },
        Kind {
            name: "rust_bool_literal_true",
            value: KindValue::Struct {
                fields: &[],
                inner: None,
                parser: |v: &str| vec![Ok("true".to_string())],
                renderer: |model: &Model, node: &Node, path: &Path| {
                    html! {
                        <span class="literal">{ "true" }</span>
                    }
                },
            },
        },
        Kind {
            name: "rust_field_access",
            value: KindValue::Struct {
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
                parser: |v: &str| vec![Ok(".".to_string())],
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
        },
        Kind {
            name: "rust_simple_path",
            value: KindValue::Struct {
                fields: &[Field {
                    name: "segments",
                    kind: RUST_PATH_IDENT_SEGMENT,
                    multiplicity: Multiplicity::Repeated,
                }],
                inner: Some("segments"),
                parser: |v: &str| vec![Ok("::".to_string())],
                renderer: |model: &Model, node: &Node, path: &Path| {
                    let (segments_head, segments) = model.view_children(node, "segments", &path);
                    let segments = segments.into_iter().intersperse(html! {{ "::" }});
                    html! {
                        <span>{ for segments }{ segments_head }</span>
                    }
                },
            },
        },
        Kind {
            name: "rust_identifier",
            value: KindValue::Struct {
                fields: &[],
                inner: None,
                parser: |v: &str| {
                    vec![if v.is_empty() {
                        Err("cannot be empty".to_string())
                    } else if v.contains(' ') {
                        Err("cannot contain whitespace".to_string())
                    } else if !v.starts_with(|c: char| c.is_alphabetic()) {
                        Err("must start with alphabetic character".to_string())
                    } else {
                        Ok(v.to_string())
                    }]
                },
                renderer: |model: &Model, node: &Node, path: &Path| {
                    html! {
                        <span class="identifier">{ node.value.clone() }</span>
                    }
                },
            },
        },
        Kind {
            name: "rust_crate",
            value: KindValue::Struct {
                fields: &[],
                inner: None,
                parser: |v: &str| vec![Ok("crate".to_string())],
                renderer: |model: &Model, node: &Node, path: &Path| {
                    html! {
                        <span class="keyword">{ "crate" }</span>
                    }
                },
            },
        },
        // https://doc.rust-lang.org/stable/reference/expressions/operator-expr.html#comparison-operators
        Kind {
            name: "rust_comparison_expression",
            value: KindValue::Struct {
                fields: &[
                    Field {
                        name: "operator",
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
                        "!=".to_string(),
                        ">".to_string(),
                        "<".to_string(),
                        ">=".to_string(),
                        "<=".to_string(),
                    ]
                    .into_iter()
                    .map(Ok)
                    .collect()
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
        },
        Kind {
            name: "rust_binary_operator",
            value: KindValue::Struct {
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
                    .into_iter()
                    .map(Ok)
                    .collect()
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
        },
        // https://doc.rust-lang.org/nightly/reference/items/functions.html
        Kind {
            name: "rust_function_definition",
            value: KindValue::Struct {
                fields: &[
                    Field {
                        name: "comment",
                        kind: &["markdown_fragment"],
                        multiplicity: Multiplicity::Single,
                    },
                    Field {
                        name: "async",
                        kind: &["rust_bool_literal"],
                        multiplicity: Multiplicity::Single,
                    },
                    Field {
                        name: "extern",
                        kind: &["rust_bool_literal"],
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
                parser: |v: &str| vec![Ok("fn".to_string())],
                renderer: |model: &Model, node: &Node, path: &Path| {
                    let comment = model.view_child(node, "comment", path);
                    let async_ = model.view_child(node, "async", path);
                    let extern_ = model.view_child(node, "extern", path);
                    let identifier = model.view_child(node, "identifier", path);
                    let (parameters_head, parameters) =
                        model.view_children(node, "parameters", path);
                    let parameters = parameters.into_iter().intersperse(html! {{ "," }});
                    let body = model.view_child(node, "body", path);
                    let return_type = model.view_child(node, "return_type", path);

                    let async_0 = if node.children.get("async").is_some() {
                        html! {
                              <span class="keyword">{ "async" }</span>
                        }
                    } else {
                        html! {
                              <></>
                        }
                    };

                    html! {
                        <span>
                            <div>{ "//" }{ comment }</div>
                            <div>{ "async" }{ async_ }</div>
                            <div>{ "extern" }{ extern_ }</div>
                            <div>
                              { async_0 }
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
        },
        Kind {
            name: "rust_function_parameter",
            value: KindValue::Struct {
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
                parser: |v: &str| vec![Ok("param".to_string())],
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
        },
        // https://doc.rust-lang.org/nightly/reference/statements.html#let-statements
        Kind {
            name: "rust_let",
            value: KindValue::Struct {
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
                parser: |v: &str| vec![Ok("let".to_string())],
                renderer: |model: &Model, node: &Node, path: &Path| {
                    let pattern = model.view_child(node, "pattern", path);
                    let value = model.view_child(node, "value", path);
                    html! {
                        <span>{ "let" }{ pattern }{ "=" }{ value }</span>
                    }
                },
            },
        },
        Kind {
            name: "rust_function_call",
            value: KindValue::Struct {
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
                parser: |v: &str| vec![Ok("(".to_string())],
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
        },
        Kind {
            name: "rust_tuple_expression",
            value: KindValue::Struct {
                fields: &[Field {
                    name: "elements",
                    kind: RUST_EXPRESSION,
                    multiplicity: Multiplicity::Repeated,
                }],
                inner: Some("elements"),
                parser: |v: &str| vec![Ok("(".to_string())],
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
        },
        Kind {
            name: "rust_struct",
            value: KindValue::Struct {
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
                parser: |v: &str| vec![Ok("struct".to_string())],
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
        },
        Kind {
            name: "rust_struct_field",
            value: KindValue::Struct {
                fields: &[
                    Field {
                        name: "visibility",
                        kind: RUST_VISIBILITY,
                        multiplicity: Multiplicity::Single,
                    },
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
                parser: |v: &str| vec![Ok("struct_field".to_string())],
                renderer: |model: &Model, node: &Node, path: &Path| {
                    let visibility = model.view_child(node, "visibility", path);
                    let identifier = model.view_child(node, "identifier", path);
                    let type_ = model.view_child(node, "type", path);
                    html! {
                        <span>
                        { visibility }{ identifier }{ ":" }{ type_ }
                        </span>
                    }
                },
            },
        },
        // https://doc.rust-lang.org/nightly/reference/items/enumerations.html
        Kind {
            name: "rust_enum",
            value: KindValue::Struct {
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
                parser: |v: &str| vec![Ok("enum".to_string())],
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
        },
        Kind {
            name: "rust_enum_variant",
            value: KindValue::Struct {
                fields: &[Field {
                    name: "identifier",
                    kind: &["rust_identifier"],
                    multiplicity: Multiplicity::Single,
                }],
                inner: None,
                parser: |v: &str| vec![Ok("enum_variant".to_string())],
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
        },
        Kind {
            name: "markdown_fragment",
            value: KindValue::Struct {
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
                parser: |v: &str| vec![Ok("markdown_fragment".to_string())],
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
        },
        Kind {
            name: "markdown_paragraph",
            value: KindValue::Struct {
                fields: &[],
                inner: None,
                parser: |v: &str| {
                    vec![if v.is_empty() {
                        Err("must not be empty".to_string())
                    } else {
                        Ok(v.to_string())
                    }]
                },
                renderer: |model: &Model, node: &Node, path: &Path| {
                    html! {
                        <span>
                        { node.value.clone() }
                        </span>
                    }
                },
            },
        },
        Kind {
            name: "markdown_heading",
            value: KindValue::Struct {
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
                parser: |v: &str| vec![Ok("#".to_string())],
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
        },
        Kind {
            name: "markdown_list",
            value: KindValue::Struct {
                fields: &[Field {
                    name: "items",
                    kind: &["markdown_paragraph"],
                    multiplicity: Multiplicity::Repeated,
                }],
                inner: Some("items"),
                parser: |v: &str| vec![Ok("-".to_string())],
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
        },
    ],
};

// Generate valid values.
type Parser = fn(&str) -> Vec<Result<String, String>>;
type Renderer = fn(&Model, &Node, &Path) -> Html;

// Generators either have logic to generate suggestions, or can delegate to other kinds.
// For instance, rust_expression may delegate to rust_identifier and rust_field_access.
// Suggestions from those may show up as `rust_expression > rust_identifier` when presented
// (hierarchy of delegations).

// Options such as pub or async for fns should be attributes of the fn block rather than children?
// Also:
// - mutability for references.
// - markdown heading level
// - markdown code block language
// These can probably be represented as enums / variants stored as value and validated when parsed
// (even for bools and ints).
// Maybe we can use :field to navigate to "hidden" fields of a block and then edit them as children?
// But it seems quite noisy to treat them as children, they are more like attributes.

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
    /* pub aliases: &'static [&'static str],
     * TODO: create list of elements, and then have validator function to filter them and
     * provide feedback if not matching, without hiding the entry. Or return one of three values
     * from parse: ok, hide, invalid */
    value: KindValue,
}

pub enum KindValue {
    Struct {
        fields: &'static [Field],
        inner: Option<&'static str>,
        renderer: Renderer,
        parser: Parser,
    },
    Enum {
        variants: &'static [&'static str],
    },
}

impl Kind {
    pub fn get_field(&self, field: &str) -> Option<Field> {
        self.get_fields().into_iter().find(|f| f.name == field)
    }

    pub fn get_fields(&self) -> Vec<Field> {
        match self.value {
            KindValue::Struct { fields, .. } => fields.iter().cloned().collect(),
            KindValue::Enum { variants } => variants
                .iter()
                .filter_map(|n| SCHEMA.get_kind(n))
                .flat_map(|k| k.get_fields())
                .collect(),
        }
    }

    pub fn inner(&self) -> Option<&'static str> {
        match self.value {
            KindValue::Struct { inner, .. } => inner,
            KindValue::Enum { .. } => {
                // XXX
                None
            }
        }
    }

    pub fn render(&self, model: &Model, node: &Node, path: &Path) -> Html {
        match self.value {
            KindValue::Struct { renderer, .. } => renderer(model, node, path),
            KindValue::Enum { .. } => {
                // XXX
                html! {
                    <span>{"enum rendering error"}</span>
                }
            }
        }
    }

    pub fn parse(&self, v: &str) -> Vec<Result<String, String>> {
        match self.value {
            KindValue::Struct { parser, .. } => parser(v),
            KindValue::Enum { variants } => variants
                .iter()
                .filter_map(|n| SCHEMA.get_kind(n))
                .flat_map(|k| k.parse(v))
                .collect(),
        }
    }
}

#[derive(Clone)]
pub struct Field {
    pub name: &'static str,
    pub kind: &'static [&'static str],
    pub multiplicity: Multiplicity,
}

#[derive(Clone)]
pub enum Multiplicity {
    // Required -- show hole if not present
    // Optional -- hide if not present
    Single,
    Repeated,
}
