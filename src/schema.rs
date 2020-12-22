use crate::types::{Inner, Model, Path, Value};
use itertools::Itertools;
use yew::{html, Html};

// https://doc.rust-lang.org/stable/reference/expressions.html
const RUST_EXPRESSION: Type = Type::Any(&[
    Type::Bool,
    Type::Int,
    Type::Inner("if"),
    Type::Inner("string"),
    Type::Inner("field_access"),
    Type::Inner("function_call"),
    Type::Inner("operator"),
    Type::Inner("match"),
]);

// https://doc.rust-lang.org/stable/reference/items.html
const RUST_ITEM: Type = Type::Any(&[
    Type::Inner("function_definition"),
    Type::Inner("struct"),
    Type::Inner("enum"),
]);

// https://doc.rust-lang.org/stable/reference/types.html#type-expressions
const RUST_TYPE: Type = Type::Any(&[
    Type::String,
    Type::Inner("tuple_type"),
    Type::Inner("reference_type"),
    Type::Inner("array_type"),
    Type::Inner("slice_type"),
    Type::Inner("simple_path"),
]);

// Alternative implementation: distinct structs implementing a parse_from method that only looks at
//the kind field of Inner, and we then try to parse each element with all of them until one
// matches.

pub const SCHEMA: Schema = Schema {
    kinds: &[
        Kind {
            name: "document",
            fields: &[Field {
                name: "bindings",
                type_: RUST_ITEM,
                multiplicity: Multiplicity::Repeated,
                validator: whatever,
            }],
            inner: None,
            renderer: |model: &Model, value: &Inner, path: &Path| {
                let (bindings_head, bindings) = model.view_children(&value, "bindings", &path);
                let bindings = bindings.into_iter().map(|b| {
                    html! {
                        <div>{ b }</div>
                    }
                });
                html! {
                    <div>
                    { bindings_head }
                    { for bindings }
                    </div>
                }
            },
        },
        Kind {
            name: "tuple_type",
            fields: &[Field {
                name: "components",
                type_: RUST_TYPE,
                multiplicity: Multiplicity::Repeated,
                validator: whatever,
            }],
            inner: Some("components"),
            renderer: |model: &Model, value: &Inner, path: &Path| todo!(),
        },
        Kind {
            name: "reference_type",
            fields: &[
                Field {
                    name: "type",
                    type_: RUST_TYPE,
                    multiplicity: Multiplicity::Single,
                    validator: whatever,
                },
                Field {
                    name: "mutable",
                    type_: Type::Bool,
                    multiplicity: Multiplicity::Single,
                    validator: whatever,
                },
                Field {
                    name: "lifetime",
                    type_: Type::Star,
                    multiplicity: Multiplicity::Single,
                    validator: whatever,
                },
            ],
            inner: None,
            renderer: |model: &Model, value: &Inner, path: &Path| {
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
                    name: "name",
                    type_: Type::String,
                    multiplicity: Multiplicity::Single,
                    validator: whatever,
                },
                Field {
                    name: "type",
                    type_: RUST_TYPE,
                    multiplicity: Multiplicity::Single,
                    validator: whatever,
                },
                Field {
                    name: "value",
                    type_: RUST_EXPRESSION,
                    multiplicity: Multiplicity::Single,
                    validator: whatever,
                },
            ],
            inner: Some("statements"),
            renderer: |model: &Model, value: &Inner, path: &Path| todo!(),
        },
        Kind {
            name: "block",
            fields: &[Field {
                name: "statements",
                type_: Type::Star,
                multiplicity: Multiplicity::Repeated,
                validator: whatever,
            }],
            inner: Some("statements"),
            renderer: |model: &Model, value: &Inner, path: &Path| {
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
                    type_: RUST_EXPRESSION,
                    multiplicity: Multiplicity::Single,
                    validator: whatever,
                },
                Field {
                    name: "match_arms",
                    type_: RUST_EXPRESSION,
                    multiplicity: Multiplicity::Single,
                    validator: whatever,
                },
            ],
            inner: Some("expression"),
            renderer: |model: &Model, value: &Inner, path: &Path| {
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
                    type_: RUST_EXPRESSION,
                    multiplicity: Multiplicity::Single,
                    validator: whatever,
                },
                Field {
                    name: "true_body", // Expression
                    type_: RUST_EXPRESSION,
                    multiplicity: Multiplicity::Single,
                    validator: whatever,
                },
                Field {
                    name: "false_body", // Expression
                    type_: RUST_EXPRESSION,
                    multiplicity: Multiplicity::Single,
                    validator: whatever,
                },
            ],
            inner: Some("true_body"),
            renderer: |model: &Model, value: &Inner, path: &Path| {
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
            name: "string",
            fields: &[Field {
                name: "value",
                type_: Type::String,
                multiplicity: Multiplicity::Single,
                validator: whatever,
            }],
            inner: Some("value"),
            renderer: |model: &Model, value: &Inner, path: &Path| {
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
                    type_: RUST_EXPRESSION,
                    multiplicity: Multiplicity::Single,
                    validator: whatever,
                },
                Field {
                    name: "field",
                    type_: Type::String,
                    multiplicity: Multiplicity::Single,
                    validator: whatever,
                },
            ],
            inner: Some("object"),
            renderer: |model: &Model, value: &Inner, path: &Path| {
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
                type_: Type::Star,
                multiplicity: Multiplicity::Repeated,
                validator: whatever,
            }],
            inner: Some("segments"),
            renderer: |model: &Model, value: &Inner, path: &Path| {
                let (segments_head, segments) = model.view_children(value, "segments", &path);
                let segments = segments.into_iter().intersperse(html! {{ "::" }});
                html! {
                    <span>{ segments_head }{ for segments }</span>
                }
            },
        },
        Kind {
            name: "crate",
            fields: &[],
            inner: None,
            renderer: |model: &Model, value: &Inner, path: &Path| {
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
                    type_: Type::String,
                    multiplicity: Multiplicity::Single,
                    validator: operator,
                },
                Field {
                    name: "left",
                    type_: RUST_EXPRESSION,
                    multiplicity: Multiplicity::Single,
                    validator: whatever,
                },
                Field {
                    name: "right",
                    type_: RUST_EXPRESSION,
                    multiplicity: Multiplicity::Single,
                    validator: whatever,
                },
            ],
            inner: Some("left"),
            renderer: |model: &Model, value: &Inner, path: &Path| {
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
                    type_: Type::Inner("markdown_document"),
                    multiplicity: Multiplicity::Single,
                    validator: whatever,
                },
                Field {
                    name: "name",
                    type_: Type::String,
                    multiplicity: Multiplicity::Single,
                    validator: identifier,
                },
                Field {
                    name: "arguments", // Pattern
                    type_: Type::Star,
                    multiplicity: Multiplicity::Repeated,
                    validator: whatever,
                },
                Field {
                    name: "return_type", // Type
                    type_: RUST_TYPE,
                    multiplicity: Multiplicity::Single,
                    validator: whatever,
                },
                Field {
                    name: "body", // Expression
                    type_: RUST_EXPRESSION,
                    multiplicity: Multiplicity::Single,
                    validator: whatever,
                },
            ],
            inner: None,
            renderer: |model: &Model, value: &Inner, path: &Path| {
                let comment = model.view_child(&value, "comment", &path);
                let label = model.view_child(&value, "name", &path);
                let (args_head, args) = model.view_children(&value, "arguments", &path);
                let args = args.into_iter().intersperse(html! {{ "," }});
                let body = model.view_child(&value, "body", &path);
                let return_type = model.view_child(&value, "return_type", &path);
                // let async_ = self.view_child(&v, "async", &path);
                // let pub_ = self.view_child(&v, "pub", &path);

                html! {
                    <span>
                        <div>{ "//" }{ comment }</div>
                        // { pub_ }
                        <div><span class="keyword">{ "fn" }</span>{ label }{ "(" }{ args_head }{ for args }{ ")" }{ "->" }{ return_type }{ "{" }</div>
                        <div class="indent">{ body }</div>{ "}" }
                    </span>
                }
            },
        },
        Kind {
            name: "pattern",
            fields: &[
                Field {
                    name: "name",
                    type_: Type::String,
                    multiplicity: Multiplicity::Single,
                    validator: identifier,
                },
                Field {
                    name: "type", // Type
                    type_: RUST_TYPE,
                    multiplicity: Multiplicity::Single,
                    validator: whatever,
                },
            ],
            inner: None,
            renderer: |model: &Model, value: &Inner, path: &Path| {
                let name = model.lookup(&value.children["name"][0]).unwrap();
                let name = match &name.value {
                    Value::String(v) => v.clone(),
                    _ => "error".to_string(),
                };
                html! {
                    <span>
                    { name }
                    </span>
                }
            },
        },
        Kind {
            name: "binding",
            fields: &[
                Field {
                    name: "name",
                    type_: Type::String,
                    multiplicity: Multiplicity::Single,
                    validator: identifier,
                },
                Field {
                    name: "type", // Type
                    type_: RUST_TYPE,
                    multiplicity: Multiplicity::Single,
                    validator: whatever,
                },
                Field {
                    name: "value", // Expression
                    type_: RUST_EXPRESSION,
                    multiplicity: Multiplicity::Single,
                    validator: whatever,
                },
            ],
            inner: Some("value"),
            renderer: |model: &Model, value: &Inner, path: &Path| {
                let name = model.view_child(value, "name", &path);
                let value = model.view_child(value, "value", &path);
                html! {
                    <span>{ "let" }{ name }{ "=" }{ value }</span>
                }
            },
        },
        Kind {
            name: "type",
            fields: &[
                Field {
                    name: "name",
                    type_: Type::String,
                    multiplicity: Multiplicity::Single,
                    validator: whatever,
                },
                // Generic type parameters.
                Field {
                    name: "arguments",
                    type_: Type::Star,
                    multiplicity: Multiplicity::Repeated,
                    validator: whatever,
                },
            ],
            inner: None,
            renderer: |model: &Model, value: &Inner, path: &Path| todo!(),
        },
        Kind {
            name: "function_call",
            fields: &[
                Field {
                    name: "expression",
                    type_: RUST_EXPRESSION,
                    multiplicity: Multiplicity::Single,
                    validator: whatever,
                },
                Field {
                    name: "arguments",
                    type_: RUST_EXPRESSION,
                    multiplicity: Multiplicity::Repeated,
                    validator: whatever,
                },
            ],
            inner: Some("expression"),
            renderer: |model: &Model, value: &Inner, path: &Path| {
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
                    type_: Type::String,
                    multiplicity: Multiplicity::Single,
                    validator: identifier,
                },
                Field {
                    name: "fields",
                    type_: Type::Inner("struct_field"),
                    multiplicity: Multiplicity::Repeated,
                    validator: whatever,
                },
            ],
            inner: None,
            renderer: |model: &Model, value: &Inner, path: &Path| {
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
                    type_: Type::String,
                    multiplicity: Multiplicity::Single,
                    validator: identifier,
                },
                Field {
                    name: "type", // Type
                    type_: RUST_TYPE,
                    multiplicity: Multiplicity::Single,
                    validator: whatever,
                },
            ],
            inner: None,
            renderer: |model: &Model, value: &Inner, path: &Path| {
                let name = model.view_child(value, "name", &path);
                let type_ = model.view_child(value, "type", &path);
                html! {
                    <span>
                    { name }{ ":" }{ type_ }
                    </span>
                }
            },
        },
        Kind {
            name: "enum",
            fields: &[
                Field {
                    name: "name",
                    type_: Type::String,
                    multiplicity: Multiplicity::Single,
                    validator: identifier,
                },
                Field {
                    name: "variants",
                    type_: Type::Inner("enum_variant"),
                    multiplicity: Multiplicity::Repeated,
                    validator: whatever,
                },
            ],
            inner: None,
            renderer: |model: &Model, value: &Inner, path: &Path| {
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
            name: "markdown_document",
            fields: &[Field {
                name: "items",
                type_: Type::Any(&[
                    Type::String,
                    Type::Inner("markdown_heading"),
                    Type::Inner("markdown_code"),
                    Type::Inner("markdown_quote"),
                    Type::Inner("markdown_list"),
                ]),
                multiplicity: Multiplicity::Repeated,
                validator: whatever,
            }],
            inner: Some("paragraphs"),
            renderer: |model: &Model, value: &Inner, path: &Path| {
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
                    type_: Type::Int,
                    multiplicity: Multiplicity::Single,
                    validator: whatever,
                },
                Field {
                    name: "text",
                    type_: Type::String,
                    multiplicity: Multiplicity::Single,
                    validator: whatever,
                },
            ],
            inner: Some("text"),
            renderer: |model: &Model, value: &Inner, path: &Path| {
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
                type_: Type::Inner("markdown_paragraph"),
                multiplicity: Multiplicity::Repeated,
                validator: whatever,
            }],
            inner: Some("items"),
            renderer: |model: &Model, value: &Inner, path: &Path| {
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

type Validator = fn(&Value) -> bool;
type Renderer = fn(&Model, &Inner, &Path) -> Html;

fn whatever(_: &Value) -> bool {
    true
}

fn identifier(v: &Value) -> bool {
    match v {
        Value::String(v) => !v.contains(' '),
        _ => false,
    }
}

fn operator(v: &Value) -> bool {
    match v {
        Value::String(v) => v == "==",
        _ => false,
    }
}

pub struct Schema {
    pub kinds: &'static [Kind],
}

pub struct Kind {
    pub name: &'static str,
    pub fields: &'static [Field],
    pub inner: Option<&'static str>,
    pub renderer: Renderer,
    // pub aliases: &'static [&'static str],
}

pub struct Field {
    pub name: &'static str,
    pub type_: Type,
    pub multiplicity: Multiplicity,
    pub validator: Validator,
}

#[derive(Debug)]
pub enum Type {
    Star,
    Bool,
    String,
    Int,
    Inner(&'static str),
    Any(&'static [Type]), // Choice between other types.
}

impl Type {
    pub fn valid(&self, value: &Value) -> bool {
        match (self, value) {
            (Type::Star, _) => true,
            (Type::Bool, Value::Bool(_)) => true,
            (Type::Int, Value::Int(_)) => true,
            (Type::String, Value::String(_)) => true,
            (Type::Inner(k), Value::Inner(v)) => k == &v.kind,
            (Type::Any(k), _) => k.iter().any(|t| t.valid(value)),
            _ => false,
        }
    }

    pub fn prefixes(&self) -> Vec<String> {
        match self {
            Type::Star => vec![],
            Type::Bool => vec!["true".to_string(), "false".to_string()],
            Type::String => vec!["\"".to_string()],
            Type::Int => vec![],
            Type::Inner(v) => vec![v.to_string()],
            Type::Any(vv) => vv.iter().flat_map(|v| v.prefixes()).collect(),
        }
    }
}

pub enum Multiplicity {
    // Required -- show hole if not present
    // Optional -- hide if not present
    Single,
    Repeated,
}
