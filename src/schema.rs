use crate::{
    node::{NodeComponent, FIELD_CLASSES},
    types::{append, get_value_from_input_event, File, Model, Msg, Node, Path, Selector},
    view,
};
use std::{collections::BTreeMap, rc::Rc};
use yew::{html, prelude::*, Html};

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

// https://doc.rust-lang.org/stable/reference/types.html#type-expressions
const RUST_TYPE: &[FieldValidator] = &[
    FieldValidator::Kind("rust_type_path"),
    FieldValidator::Kind("rust_tuple_type"),
    FieldValidator::Kind("rust_never_type"),
    FieldValidator::Kind("rust_raw_pointer_type"),
    FieldValidator::Kind("rust_reference_type"),
    FieldValidator::Kind("rust_array_type"),
    FieldValidator::Kind("rust_slice_type"),
    FieldValidator::Kind("rust_primitive_type"),
    FieldValidator::Kind("rust_inferred_type"),
];

// https://doc.rust-lang.org/stable/reference/patterns.html
const RUST_PATTERN: &[FieldValidator] = &[
    FieldValidator::Kind("rust_literal_pattern"),
    FieldValidator::Kind("rust_wildcard_pattern"),
    FieldValidator::Kind("rust_rest_pattern"),
    FieldValidator::Kind("rust_reference_pattern"),
    FieldValidator::Kind("rust_struct_pattern"),
    FieldValidator::Kind("rust_tuple_struct_pattern"),
    FieldValidator::Kind("rust_tuple_pattern"),
    FieldValidator::Kind("rust_grouped_pattern"),
    FieldValidator::Kind("rust_path_pattern"),
    FieldValidator::Kind("rust_macro_invocation"),
    FieldValidator::Kind("rust_identifier_pattern"),
];

// https://doc.rust-lang.org/stable/reference/expressions.html
const RUST_EXPRESSION: &[FieldValidator] = &[
    FieldValidator::Kind("rust_field_access"),
    FieldValidator::Kind("rust_function_call"),
    FieldValidator::Kind("rust_tuple_expression"),
    FieldValidator::Kind("rust_struct_expr_struct"),
    FieldValidator::Kind("rust_if"),
    FieldValidator::Kind("rust_match"),
    FieldValidator::Kind("rust_operator"),
    FieldValidator::Kind("rust_comparison_expression"),
    FieldValidator::Kind("rust_bool_literal"),
    FieldValidator::Kind("rust_identifier"),
    FieldValidator::Kind("rust_string_literal"),
];

// https://doc.rust-lang.org/stable/reference/expressions.html
const RUST_STATEMENT: &[FieldValidator] = &[
    FieldValidator::Kind("rust_field_access"),
    FieldValidator::Kind("rust_function_call"),
    FieldValidator::Kind("rust_tuple_expression"),
    FieldValidator::Kind("rust_struct_expr_struct"),
    FieldValidator::Kind("rust_if"),
    FieldValidator::Kind("rust_match"),
    FieldValidator::Kind("rust_operator"),
    FieldValidator::Kind("rust_comparison_expression"),
    FieldValidator::Kind("rust_bool_literal"),
    FieldValidator::Kind("rust_identifier"),
    FieldValidator::Kind("rust_string_literal"),
    FieldValidator::Kind("rust_item"),
    FieldValidator::Kind("rust_let"),
];

// https://doc.rust-lang.org/stable/reference/items.html
const RUST_ITEM: &[FieldValidator] = &[
    FieldValidator::Kind("rust_vis_item"),
    FieldValidator::Kind("rust_macro_item"),
];

const RUST_IDENTIFIER: FieldValidator = FieldValidator::Literal(|v: &str| {
    if v.is_empty() {
        vec![ValidationError {
            path: vec![].into(),
            message: "empty identifier".to_string(),
        }]
    } else if v.contains(' ') {
        vec![ValidationError {
            path: vec![].into(),
            message: "contains whitespace".to_string(),
        }]
    } else if !v.starts_with(|c: char| c.is_alphabetic()) {
        vec![ValidationError {
            path: vec![].into(),
            message: "must start with alphabetic character".to_string(),
        }]
    } else {
        vec![]
    }
});

const MARKDOWN_ITEM: &[FieldValidator] = &[
    MARKDOWN_PARAGRAPH,
    FieldValidator::Kind("markdown_heading"),
    FieldValidator::Kind("markdown_code"),
    FieldValidator::Kind("markdown_quote"),
    FieldValidator::Kind("markdown_list"),
];

const MARKDOWN_PARAGRAPH: FieldValidator = FieldValidator::Literal(|_v: &str| vec![]);

fn any(_v: &str) -> Vec<ValidationError> {
    vec![]
}

pub const SCHEMA: Schema = Schema {
    kinds: &[
        Kind {
            name: "root",
            value: KindValue::Struct {
                fields: &[Field {
                    name: "value",
                    multiplicity: Multiplicity::Repeated,
                    validators: &[
                        FieldValidator::Kind("git"),
                        FieldValidator::Kind("rust_fragment"),
                        FieldValidator::Kind("markdown_fragment"),
                    ],
                }],
                inner: None,
                constructors: &["root"],
                validator: |_c: &ValidatorContext| vec![],
                renderer: default_renderer,
            },
        },
        Kind {
            name: "git",
            value: KindValue::Struct {
                fields: &[Field {
                    name: "command",
                    multiplicity: Multiplicity::Repeated,
                    validators: &[
                        FieldValidator::Kind("git_commit"),
                        FieldValidator::Kind("git_status"),
                    ],
                }],
                inner: None,
                constructors: &["git"],
                validator: |_c: &ValidatorContext| vec![],
                renderer: default_renderer,
            },
        },
        Kind {
            name: "git_commit",
            value: KindValue::Struct {
                fields: &[
                    Field {
                        name: "message",
                        multiplicity: Multiplicity::Single,
                        validators: &[FieldValidator::Literal(any)],
                    },
                    Field {
                        name: "author",
                        multiplicity: Multiplicity::Single,
                        validators: &[FieldValidator::Literal(any)],
                    },
                    Field {
                        name: "date",
                        multiplicity: Multiplicity::Single,
                        validators: &[FieldValidator::Literal(any)],
                    },
                    Field {
                        name: "interactive",
                        multiplicity: Multiplicity::Single,
                        validators: &[FieldValidator::Literal(any)],
                    },
                    Field {
                        name: "amend",
                        multiplicity: Multiplicity::Single,
                        validators: &[FieldValidator::Literal(any)],
                    },
                    Field {
                        name: "dry-run",
                        multiplicity: Multiplicity::Single,
                        validators: &[FieldValidator::Literal(any)],
                    },
                    Field {
                        name: "squash",
                        multiplicity: Multiplicity::Single,
                        validators: &[FieldValidator::Literal(any)],
                    },
                    Field {
                        name: "fixup",
                        multiplicity: Multiplicity::Single,
                        validators: &[FieldValidator::Literal(any)],
                    },
                    Field {
                        name: "reset-author",
                        multiplicity: Multiplicity::Single,
                        validators: &[FieldValidator::Literal(any)],
                    },
                ],
                inner: None,
                constructors: &["commit"],
                validator: |_c: &ValidatorContext| vec![],
                renderer: default_renderer,
            },
        },
        Kind {
            name: "git_status",
            value: KindValue::Struct {
                fields: &[
                    Field {
                        name: "short",
                        multiplicity: Multiplicity::Single,
                        validators: &[FieldValidator::Literal(any)],
                    },
                    Field {
                        name: "branch",
                        multiplicity: Multiplicity::Single,
                        validators: &[FieldValidator::Literal(any)],
                    },
                    Field {
                        name: "show-stash",
                        multiplicity: Multiplicity::Single,
                        validators: &[FieldValidator::Literal(any)],
                    },
                    Field {
                        name: "porcelain",
                        multiplicity: Multiplicity::Single,
                        validators: &[FieldValidator::Literal(any)],
                    },
                    Field {
                        name: "long",
                        multiplicity: Multiplicity::Single,
                        validators: &[FieldValidator::Literal(any)],
                    },
                    Field {
                        name: "verbose",
                        multiplicity: Multiplicity::Single,
                        validators: &[FieldValidator::Literal(any)],
                    },
                    Field {
                        name: "untracked-files",
                        multiplicity: Multiplicity::Single,
                        validators: &[FieldValidator::Literal(any)],
                    },
                    Field {
                        name: "ignore-submodules",
                        multiplicity: Multiplicity::Single,
                        validators: &[FieldValidator::Literal(any)],
                    },
                    Field {
                        name: "ignored",
                        multiplicity: Multiplicity::Single,
                        validators: &[FieldValidator::Literal(any)],
                    },
                ],
                inner: None,
                constructors: &["commit"],
                validator: |_c: &ValidatorContext| vec![],
                renderer: default_renderer,
            },
        },
        Kind {
            name: "rust_fragment",
            value: KindValue::Struct {
                fields: &[Field {
                    name: "items",
                    multiplicity: Multiplicity::Repeated,
                    validators: RUST_ITEM,
                }],
                inner: None,
                constructors: &["rust_fragment"],
                validator: |_c: &ValidatorContext| vec![],
                renderer: |c: &ValidatorContext| {
                    let (items_head, items) = c.view_children("items");
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
            name: "rust_vis_item",
            value: KindValue::Struct {
                fields: &[
                    Field {
                        name: "visibility",
                        multiplicity: Multiplicity::Single,
                        validators: &[
                            FieldValidator::Kind("rust_visibility_pub"),
                            FieldValidator::Kind("rust_visibility_pub_crate"),
                            FieldValidator::Kind("rust_visibility_pub_self"),
                            FieldValidator::Kind("rust_visibility_pub_super"),
                            FieldValidator::Kind("rust_visibility_pub_in"),
                        ],
                    },
                    Field {
                        name: "item",
                        multiplicity: Multiplicity::Single,
                        validators: &[
                            FieldValidator::Kind("rust_constant"),
                            FieldValidator::Kind("rust_enum"),
                            FieldValidator::Kind("rust_function"),
                            FieldValidator::Kind("rust_struct"),
                            FieldValidator::Kind("rust_impl"),
                        ],
                    },
                ],
                inner: None,
                constructors: &["vis_item"],
                validator: |_c: &ValidatorContext| vec![],
                renderer: |c: &ValidatorContext| {
                    let visibility = c.view_child("visibility");
                    let inner = c.view_child("item");
                    html! {
                        <div>{ visibility }{ inner }</div>
                    }
                },
            },
        },
        /*
        Kind {
            name: "rust_vis_item_inner",
            value: KindValue::Union {
                variants: &[
                    "rust_constant",
                    "rust_enum",
                    "rust_function",
                    "rust_struct",
                    "rust_impl",
                ],
            },
        },
        Kind {
            name: "rust_visibility",
            value: KindValue::Union {
                variants: &[
                    "rust_visibility_pub",
                    "rust_visibility_pub_crate",
                    "rust_visibility_pub_self",
                    "rust_visibility_pub_super",
                    "rust_visibility_pub_in",
                ],
            },
        },
        Kind {
            name: "rust_path_ident_segment",
            value: KindValue::Union {
                variants: &[
                    "rust_path_ident_segment_super",
                    "rust_path_ident_segment_self",
                    "rust_path_ident_segment_self_upper",
                    "rust_path_ident_segment_crate",
                    "rust_path_ident_segment_crate_dollar",
                    "rust_identifier",
                ],
            },
        },
        // https://doc.rust-lang.org/stable/reference/items/implementations.html
        Kind {
            name: "rust_impl",
            value: KindValue::Union {
                variants: &["rust_trait_impl", "rust_inherent_impl"],
            },
        },
        */
        Kind {
            name: "rust_trait_impl",
            value: KindValue::Struct {
                fields: &[
                    Field {
                        name: "generics",
                        multiplicity: Multiplicity::Repeated,
                        validators: RUST_TYPE,
                    },
                    Field {
                        name: "trait",
                        multiplicity: Multiplicity::Single,
                        validators: RUST_TYPE,
                    },
                    Field {
                        name: "type",
                        multiplicity: Multiplicity::Single,
                        validators: RUST_TYPE,
                    },
                    Field {
                        name: "items",
                        multiplicity: Multiplicity::Repeated,
                        validators: &[FieldValidator::Kind("rust_trait_impl_item")],
                    },
                ],
                inner: None,
                constructors: &["impl_trait"],
                validator: |_c: &ValidatorContext| vec![],
                renderer: |c: &ValidatorContext| {
                    let trait_ = c.view_child("trait");
                    let type_ = c.view_child("type");

                    let (items_head, items) = c.view_children("items");
                    let items = items.into_iter().map(|b| {
                        html! {
                            <div>{ b }</div>
                        }
                    });

                    html! {
                        <span>
                            <div>
                              <span class="keyword">{ "impl" }</span>
                              { trait_ }
                              <span class="keyword">{ "for" }</span>
                              { type_ }
                              { "{" }
                            </div>
                            <div class="indent">
                              { for items }
                              { items_head }
                            </div>
                            <div>
                              { "}" }
                            </div>
                        </span>
                    }
                },
            },
        },
        /*
        Kind {
            name: "rust_trait_impl_item",
            value: KindValue::Union {
                variants: &[
                    "rust_type_alias",
                    "rust_constant",
                    "rust_function",
                    "rust_method",
                ],
            },
        },
        // https://doc.rust-lang.org/stable/reference/patterns.html
        Kind {
            name: "rust_pattern",
            value: KindValue::Union {
                variants: &[
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
                ],
            },
        },
        // https://doc.rust-lang.org/stable/reference/patterns.html#literal-patterns
        Kind {
            name: "rust_literal_pattern",
            value: KindValue::Union {
                variants: &[
                    "rust_bool_literal",
                    "rust_char_literal",
                    "rust_byte_literal",
                    "rust_string_literal",
                    "rust_number_literal",
                ],
            },
        },
        */
        // https://doc.rust-lang.org/stable/reference/patterns.html#wildcard-pattern
        Kind {
            name: "rust_wildcard_pattern",
            value: KindValue::Struct {
                fields: &[],
                inner: None,
                constructors: &["wildcard"],
                validator: |_c: &ValidatorContext| vec![],
                renderer: |_c: &ValidatorContext| {
                    html! {
                      <span>{ "_" }</span>
                    }
                },
            },
        },
        // https://doc.rust-lang.org/stable/reference/patterns.html#identifier-patterns
        Kind {
            name: "rust_identifier_pattern",
            value: KindValue::Struct {
                fields: &[
                    Field {
                        name: "ref",
                        multiplicity: Multiplicity::Single,
                        validators: &[FieldValidator::Kind("rust_ref")],
                    },
                    Field {
                        name: "mut",
                        multiplicity: Multiplicity::Single,
                        validators: &[FieldValidator::Kind("rust_mut")],
                    },
                    Field {
                        name: "identifier",
                        multiplicity: Multiplicity::Single,
                        validators: &[FieldValidator::Kind("rust_identifier")],
                    },
                    Field {
                        name: "pattern",
                        multiplicity: Multiplicity::Single,
                        validators: RUST_PATTERN,
                    },
                ],
                inner: None,
                constructors: &["identifier"],
                validator: |_c: &ValidatorContext| vec![],
                renderer: |c: &ValidatorContext| {
                    let ref_ = c.view_child("ref");
                    let mut_ = c.view_child("mut");
                    let identifier = c.view_child("identifier");
                    let pattern = c.view_child("pattern");
                    html! {
                      <span>{ ref_ }{ mut_ }{ identifier }{ "@" }{ pattern }</span>
                    }
                },
            },
        },
        /*
        Kind {
            name: "rust_comparison_operator",
            value: KindValue::Union {
                variants: &[
                    "rust_comparison_operator_==",
                    "rust_comparison_operator_!=",
                    "rust_comparison_operator_>",
                    "rust_comparison_operator_<",
                    "rust_comparison_operator_>=",
                    "rust_comparison_operator_<=",
                ],
            },
        },
        // https://doc.rust-lang.org/stable/reference/expressions.html
        Kind {
            name: "rust_expression",
            value: KindValue::Union {
                variants: &[
                    "rust_field_access",
                    "rust_function_call",
                    "rust_tuple_expression",
                    "rust_struct_expr_struct",
                    "rust_if",
                    "rust_match",
                    "rust_operator",
                    "rust_comparison_expression",
                    "rust_bool_literal",
                    "rust_number_literal",
                    "rust_identifier",
                    "rust_string_literal",
                ],
            },
        },
        // https://doc.rust-lang.org/stable/reference/statements.html
        Kind {
            name: "rust_statement",
            value: KindValue::Union {
                variants: &["rust_item", "rust_let", "rust_expression"],
            },
        },
        // https://doc.rust-lang.org/stable/reference/types.html#type-expressions
        Kind {
            name: "rust_type",
            value: KindValue::Union {
                variants: &[
                    "rust_type_path",
                    "rust_tuple_type",
                    "rust_never_type",
                    "rust_raw_pointer_type",
                    "rust_reference_type",
                    "rust_array_type",
                    "rust_slice_type",
                    "rust_primitive_type",
                    "rust_inferred_type",
                ],
            },
        },
        */
        Kind {
            name: "rust_tuple_type",
            value: KindValue::Struct {
                fields: &[Field {
                    name: "components",
                    multiplicity: Multiplicity::Repeated,
                    validators: RUST_TYPE,
                }],
                inner: Some("components"),
                constructors: &["tuple"],
                validator: |_c: &ValidatorContext| vec![],
                renderer: |c: &ValidatorContext| {
                    let (components_head, components) = c.view_children("components");
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
                constructors: &[
                    "bool", "char", "str", "u8", "u16", "u32", "u64", "u128", "i8", "i16", "i32",
                    "i64", "i128", "f32", "f64", "usize", "isize",
                ],
                validator: |_c: &ValidatorContext| vec![],
                renderer: |c: &ValidatorContext| {
                    html! {
                        <span class="type">{ c.node.value.clone() }</span>
                    }
                },
            },
        },
        Kind {
            name: "rust_path_ident_segment_super",
            value: KindValue::Struct {
                fields: &[],
                inner: None,
                constructors: &["super"],
                validator: |_c: &ValidatorContext| vec![],
                renderer: |_c: &ValidatorContext| {
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
                constructors: &["self"],
                validator: |_c: &ValidatorContext| vec![],
                renderer: |_c: &ValidatorContext| {
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
                constructors: &["Self"],
                validator: |_c: &ValidatorContext| vec![],
                renderer: |_c: &ValidatorContext| {
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
                constructors: &["crate"],
                validator: |_c: &ValidatorContext| vec![],
                renderer: |_c: &ValidatorContext| {
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
                constructors: &["$crate"],
                validator: |_c: &ValidatorContext| vec![],
                renderer: |_c: &ValidatorContext| {
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
                constructors: &["pub"],
                validator: |_c: &ValidatorContext| vec![],
                renderer: |_c: &ValidatorContext| {
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
                constructors: &["pub_crate"],
                validator: |_c: &ValidatorContext| vec![],
                renderer: |_c: &ValidatorContext| {
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
                constructors: &["pub_self"],
                validator: |_c: &ValidatorContext| vec![],
                renderer: |_c: &ValidatorContext| {
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
                    multiplicity: Multiplicity::Single,
                    validators: &[FieldValidator::Kind("rust_simple_path")],
                }],
                inner: None,
                constructors: &["pub_in"],
                validator: |_c: &ValidatorContext| vec![],
                renderer: |c: &ValidatorContext| {
                    let path = c.view_child("path");
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
                    multiplicity: Multiplicity::Repeated,
                    validators: &[FieldValidator::Kind("rust_path_ident_segment")],
                }],
                inner: Some("segments"),
                constructors: &["type_path"],
                validator: |_c: &ValidatorContext| vec![],
                renderer: |c: &ValidatorContext| {
                    let (segments_head, segments) = c.view_children("segments");
                    let segments = segments
                        .into_iter()
                        .intersperse(html! { <span>{ "::" }</span>});
                    html! {
                        <span>
                        { "::" }{ for segments }{ segments_head }
                        </span>
                    }
                },
            },
        },
        // https://doc.rust-lang.org/nightly/reference/types/pointer.html#references--and-mut
        Kind {
            name: "rust_reference_type",
            value: KindValue::Struct {
                fields: &[
                    Field {
                        name: "lifetime",
                        multiplicity: Multiplicity::Single,
                        validators: &[FieldValidator::Kind("rust_lifetime")],
                    },
                    Field {
                        name: "mutable",
                        multiplicity: Multiplicity::Single,
                        validators: &[FieldValidator::Kind("rust_bool")],
                    },
                    Field {
                        name: "type",
                        multiplicity: Multiplicity::Single,
                        validators: RUST_TYPE,
                    },
                ],
                inner: Some("type"),
                constructors: &["reference"],
                validator: |_c: &ValidatorContext| vec![],
                renderer: |c: &ValidatorContext| {
                    let lifetime = c.view_child("lifetime");
                    let mutable = c.view_child("mutable");
                    let type_ = c.view_child("type");
                    html! {
                        <span>
                        { "&" }{ lifetime }{ mutable }{ type_ }
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
                        multiplicity: Multiplicity::Single,
                        validators: &[RUST_IDENTIFIER],
                    },
                    Field {
                        name: "type",
                        multiplicity: Multiplicity::Single,
                        validators: RUST_TYPE,
                    },
                    Field {
                        name: "expression",
                        multiplicity: Multiplicity::Single,
                        validators: RUST_EXPRESSION,
                    },
                ],
                inner: Some("statements"),
                constructors: &["const"],
                validator: |_c: &ValidatorContext| vec![],
                renderer: |c: &ValidatorContext| {
                    let identifier = c.view_child("identifier");
                    let type_ = c.view_child("type");
                    let expression = c.view_child("expression");
                    html! {
                        <span>
                            <span class="keyword">{ "const" }</span>
                            { identifier }
                            { ":" }
                            { type_ }
                            { "=" }
                            { expression }
                            { ";" }
                        </span>
                    }
                },
            },
        },
        Kind {
            name: "rust_block",
            value: KindValue::Struct {
                fields: &[
                    Field {
                        name: "statements",
                        multiplicity: Multiplicity::Repeated,
                        validators: RUST_STATEMENT,
                    },
                    Field {
                        name: "expression",
                        multiplicity: Multiplicity::Single,
                        validators: RUST_EXPRESSION,
                    },
                ],
                inner: Some("statements"),
                constructors: &["block"],
                validator: |_c: &ValidatorContext| vec![],
                renderer: |c: &ValidatorContext| {
                    let (_statements_head, statements) = c.view_children("statements");
                    let statements = statements.into_iter().map(|v| {
                        html! {
                            <div class="indent">{ v }{ ";" }</div>
                        }
                    });
                    let expression = c.view_child("expression");

                    html! {
                        <span>
                        { "{" }
                          { for statements }
                          <div class="indent">{ expression }</div>
                        { "}" }
                        </span>
                    }
                },
            },
        },
        Kind {
            name: "rust_match",
            value: KindValue::Struct {
                fields: &[
                    Field {
                        name: "expression",
                        multiplicity: Multiplicity::Single,
                        validators: RUST_EXPRESSION,
                    },
                    Field {
                        name: "match_arms",
                        multiplicity: Multiplicity::Repeated,
                        validators: &[FieldValidator::Kind("rust_match_arm")],
                    },
                ],
                inner: Some("match_arms"),
                constructors: &["match"],
                validator: |_c: &ValidatorContext| vec![],
                renderer: |c: &ValidatorContext| {
                    let expression = c.view_child("expression");
                    let (match_arms_head, match_arms) = c.view_children("match_arms");
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
                        multiplicity: Multiplicity::Repeated,
                        validators: RUST_PATTERN,
                    },
                    Field {
                        name: "guard",
                        multiplicity: Multiplicity::Single,
                        validators: RUST_EXPRESSION,
                    },
                    Field {
                        name: "expression",
                        multiplicity: Multiplicity::Single,
                        validators: RUST_EXPRESSION,
                    },
                ],
                inner: Some("match_arms"),
                constructors: &["match_arm"],
                validator: |_c: &ValidatorContext| vec![],
                renderer: |c: &ValidatorContext| {
                    let (patterns_head, patterns) = c.view_children("patterns");
                    let patterns = patterns.into_iter().intersperse(html! {
                            <span>{ "|" }</span>

                    });
                    let guard = c.view_child("guard");
                    let expression = c.view_child("expression");
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
                        name: "condition",
                        multiplicity: Multiplicity::Single,
                        validators: RUST_EXPRESSION,
                    },
                    Field {
                        name: "true_body",
                        multiplicity: Multiplicity::Single,
                        validators: RUST_EXPRESSION,
                    },
                    Field {
                        name: "false_body",
                        multiplicity: Multiplicity::Single,
                        validators: RUST_EXPRESSION,
                    },
                ],
                inner: Some("true_body"),
                constructors: &["if"],
                validator: |_c: &ValidatorContext| vec![],
                renderer: |c: &ValidatorContext| {
                    let condition = c.view_child("condition");
                    let true_body = c.view_child("true_body");
                    let false_body = c.view_child("false_body");
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
                fields: &[Field {
                    name: "value",
                    multiplicity: Multiplicity::Single,
                    validators: &[FieldValidator::Kind("literal")],
                }],
                inner: None,
                constructors: &["string"],
                validator: |_c: &ValidatorContext| vec![],
                renderer: |c: &ValidatorContext| {
                    let value = c.view_child("value");
                    html! {
                        <span>
                        { "\"" }{ value }{ "\"" }
                        </span>
                    }
                },
            },
        },
        Kind {
            name: "rust_number_literal",
            value: KindValue::Struct {
                fields: &[Field {
                    name: "value",
                    multiplicity: Multiplicity::Single,
                    validators: &[FieldValidator::Kind("literal")],
                }],
                inner: None,
                constructors: &["number"],
                validator: |c: &ValidatorContext| {
                    let node = &c.node;
                    // TODO: child.
                    if node.value.parse::<i32>().is_ok() {
                        vec![]
                    } else {
                        vec![ValidationError {
                            path: vec![].into(),
                            message: "invalid number".to_string(),
                        }]
                    }
                },
                renderer: |c: &ValidatorContext| {
                    let value = c.view_child("value");
                    html! {
                        <span>
                        { "#" }
                        { value }
                        </span>
                    }
                },
            },
        },
        /*
        Kind {
            name: "rust_bool_literal",
            value: KindValue::Union {
                variants: &["rust_bool_literal_false", "rust_bool_literal_true"],
            },
        },
        */
        Kind {
            name: "rust_bool_literal_false",
            value: KindValue::Struct {
                fields: &[],
                inner: None,
                constructors: &["false"],
                validator: |_c: &ValidatorContext| vec![],
                renderer: |_c: &ValidatorContext| {
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
                constructors: &["true"],
                validator: |_c: &ValidatorContext| vec![],
                renderer: |_c: &ValidatorContext| {
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
                        name: "container",
                        multiplicity: Multiplicity::Single,
                        validators: RUST_EXPRESSION,
                    },
                    Field {
                        name: "field",
                        multiplicity: Multiplicity::Single,
                        validators: &[FieldValidator::Kind("rust_identifier")],
                    },
                ],
                inner: Some("object"),
                constructors: &["field_access"],
                validator: |_c: &ValidatorContext| vec![],
                renderer: |c: &ValidatorContext| {
                    let container = c.view_child("container");
                    let field = c.view_child("field");
                    html! {
                        <span>
                        { container }
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
                    multiplicity: Multiplicity::Repeated,
                    validators: &[FieldValidator::Kind("rust_path_ident_segment")],
                }],
                inner: Some("segments"),
                constructors: &["::"],
                validator: |_c: &ValidatorContext| vec![],
                renderer: |c: &ValidatorContext| {
                    let (segments_head, segments) = c.view_children("segments");
                    let segments = segments.into_iter().intersperse(html! {{ "::" }});
                    html! {
                        <span>{ for segments }{ segments_head }</span>
                    }
                },
            },
        },
        Kind {
            name: "rust_crate",
            value: KindValue::Struct {
                fields: &[],
                inner: None,
                constructors: &["crate"],
                validator: |_c: &ValidatorContext| vec![],
                renderer: |_c: &ValidatorContext| {
                    html! {
                        <span class="keyword">{ "crate" }</span>
                    }
                },
            },
        },
        Kind {
            name: "rust_lifetime_static",
            value: KindValue::Struct {
                fields: &[],
                inner: None,
                constructors: &["static"],
                validator: |_c: &ValidatorContext| vec![],
                renderer: |_c: &ValidatorContext| {
                    html! {
                        <span>{ "'static" }</span>
                    }
                },
            },
        },
        Kind {
            name: "rust_lifetime_underscore",
            value: KindValue::Struct {
                fields: &[],
                inner: None,
                constructors: &["_"],
                validator: |_c: &ValidatorContext| vec![],
                renderer: |_c: &ValidatorContext| {
                    html! {
                        <span>{ "'_" }</span>
                    }
                },
            },
        },
        // https://doc.rust-lang.org/nightly/reference/tokens.html#lifetimes-and-loop-labels
        Kind {
            name: "rust_lifetime_or_label",
            value: KindValue::Struct {
                fields: &[Field {
                    name: "identifier",
                    multiplicity: Multiplicity::Single,
                    validators: &[RUST_IDENTIFIER],
                }],
                inner: None,
                constructors: &["'"],
                validator: |_c: &ValidatorContext| vec![],
                renderer: |c: &ValidatorContext| {
                    let identifier = c.view_child("identifier");
                    html! {
                        <span>{ "'" }{ identifier }</span>
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
                        name: "left",
                        validators: &[FieldValidator::Kind("rust_expression")],
                        multiplicity: Multiplicity::Single,
                    },
                    Field {
                        name: "operator",
                        validators: &[FieldValidator::Kind("rust_comparison_operator")],
                        multiplicity: Multiplicity::Single,
                    },
                    Field {
                        name: "right",
                        validators: &[FieldValidator::Kind("rust_expression")],
                        multiplicity: Multiplicity::Single,
                    },
                ],
                inner: Some("left"),
                constructors: &["==", "!=", ">", "<", ">=", "<="],
                validator: |_c: &ValidatorContext| vec![],
                renderer: |c: &ValidatorContext| {
                    let operator = c.view_child("operator");
                    let left = c.view_child("left");
                    let right = c.view_child("right");
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
            name: "rust_comparison_operator_==",
            value: KindValue::Struct {
                fields: &[],
                inner: None,
                constructors: &["=="],
                validator: |_c: &ValidatorContext| vec![],
                renderer: |_c: &ValidatorContext| {
                    html! {
                        <span class="keyword">{ "==" }</span>
                    }
                },
            },
        },
        Kind {
            name: "rust_comparison_operator_!=",
            value: KindValue::Struct {
                fields: &[],
                inner: None,
                constructors: &["!="],
                validator: |_c: &ValidatorContext| vec![],
                renderer: |_c: &ValidatorContext| {
                    html! {
                        <span class="keyword">{ "!=" }</span>
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
                        validators: &[FieldValidator::Kind("rust_expression")],
                        multiplicity: Multiplicity::Single,
                    },
                    Field {
                        name: "left",
                        validators: &[FieldValidator::Kind("rust_expression")],
                        multiplicity: Multiplicity::Single,
                    },
                    Field {
                        name: "right",
                        validators: &[FieldValidator::Kind("rust_expression")],
                        multiplicity: Multiplicity::Single,
                    },
                ],
                inner: Some("left"),
                constructors: &[
                    "==", "+", "+=", "-", "-=", "<<", ">>", "<", ">", "&&", "||", "&", "|", "^",
                ],
                validator: |_c: &ValidatorContext| vec![],
                renderer: |c: &ValidatorContext| {
                    let operator = c.view_child("operator");
                    let left = c.view_child("left");
                    let right = c.view_child("right");
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
            name: "rust_function",
            value: KindValue::Struct {
                fields: &[
                    Field {
                        name: "comment",
                        multiplicity: Multiplicity::Single,
                        validators: &[FieldValidator::Kind("markdown_fragment")],
                    },
                    Field {
                        name: "const",
                        multiplicity: Multiplicity::Single,
                        validators: &[FieldValidator::Kind("rust_function_qualifier_const")],
                    },
                    Field {
                        name: "async",
                        multiplicity: Multiplicity::Single,
                        validators: &[FieldValidator::Kind("rust_function_qualifier_async")],
                    },
                    Field {
                        name: "unsafe",
                        multiplicity: Multiplicity::Single,
                        validators: &[FieldValidator::Kind("rust_function_qualifier_unsafe")],
                    },
                    Field {
                        name: "extern",
                        multiplicity: Multiplicity::Single,
                        validators: &[FieldValidator::Kind("rust_function_qualifier_extern")],
                    },
                    Field {
                        name: "identifier",
                        multiplicity: Multiplicity::Single,
                        validators: &[FieldValidator::Kind("rust_identifier_literal")],
                    },
                    Field {
                        name: "generic",
                        multiplicity: Multiplicity::Single,
                        validators: &[FieldValidator::Kind("rust_generic_params")],
                    },
                    Field {
                        name: "parameters",
                        multiplicity: Multiplicity::Repeated,
                        validators: &[FieldValidator::Kind("rust_function_parameter")],
                    },
                    Field {
                        name: "return_type",
                        multiplicity: Multiplicity::Single,
                        validators: RUST_TYPE,
                    },
                    Field {
                        name: "body",
                        multiplicity: Multiplicity::Single,
                        validators: &[FieldValidator::Kind("rust_block")],
                    },
                ],
                inner: None,
                constructors: &["fn"],
                validator: |_c: &ValidatorContext| vec![],
                renderer: |c: &ValidatorContext| {
                    let comment = c.view_child("comment");

                    let const_ = c.view_child("const");
                    let async_ = c.view_child("async");
                    let unsafe_ = c.view_child("unsafe");
                    let extern_ = c.view_child("extern");

                    let identifier = c.view_child("identifier");
                    let generic = c.view_child("generic");
                    let (_parameters_head, parameters) = c.view_children("parameters");
                    let parameters = parameters.into_iter().intersperse(html! {{ "," }});
                    let body = c.view_child("body");
                    let return_type = c.view_child("return_type");

                    html! {
                        <span>
                            <div>{ "//" }{ comment }</div>
                            <div>
                              { const_ }{ async_ }{ unsafe_ }{ extern_ }
                              <span class="keyword">{ "fn" }</span>{ identifier }{ generic }
                              { "(" }{ for parameters }{ ")" }
                              { "->" }{ return_type }
                              <div class="indent">{ body }</div>
                            </div>
                        </span>
                    }
                },
            },
        },
        Kind {
            name: "rust_function_qualifier_const",
            value: KindValue::Struct {
                fields: &[],
                inner: None,
                constructors: &["const"],
                validator: |_c: &ValidatorContext| vec![],
                renderer: |_c: &ValidatorContext| {
                    html! {
                        <span>{ "const" }</span>
                    }
                },
            },
        },
        Kind {
            name: "rust_function_qualifier_async",
            value: KindValue::Struct {
                fields: &[],
                inner: None,
                constructors: &["async"],
                validator: |_c: &ValidatorContext| vec![],
                renderer: |_c: &ValidatorContext| {
                    html! {
                        <span>{ "async" }</span>
                    }
                },
            },
        },
        Kind {
            name: "rust_function_qualifier_unsafe",
            value: KindValue::Struct {
                fields: &[],
                inner: None,
                constructors: &["unsafe"],
                validator: |_c: &ValidatorContext| vec![],
                renderer: |_c: &ValidatorContext| {
                    html! {
                        <span>{ "unsafe" }</span>
                    }
                },
            },
        },
        Kind {
            name: "rust_function_qualifier_extern",
            value: KindValue::Struct {
                fields: &[Field {
                    name: "abi",
                    validators: &[FieldValidator::Kind("rust_string_literal")],
                    multiplicity: Multiplicity::Single,
                }],
                inner: None,
                constructors: &["extern"],
                validator: |_c: &ValidatorContext| vec![],
                renderer: |c: &ValidatorContext| {
                    let abi = c.view_child("abi");
                    html! {
                        <span>{ "extern" }{ abi }</span>
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
                        multiplicity: Multiplicity::Single,
                        validators: RUST_PATTERN,
                    },
                    Field {
                        name: "type",
                        multiplicity: Multiplicity::Single,
                        validators: RUST_TYPE,
                    },
                ],
                inner: None,
                constructors: &["param"],
                validator: |_c: &ValidatorContext| vec![],
                renderer: |c: &ValidatorContext| {
                    let pattern = c.view_child("pattern");
                    let type_ = c.view_child("type");
                    html! {
                        <span>
                        { pattern }{ ":" }{ type_ }
                        </span>
                    }
                },
            },
        },
        Kind {
            name: "rust_generic_params",
            value: KindValue::Struct {
                fields: &[Field {
                    name: "parameters",
                    validators: &[FieldValidator::Kind("rust_generic_param")],
                    multiplicity: Multiplicity::Repeated,
                }],
                inner: None,
                constructors: &["generic"],
                validator: |_c: &ValidatorContext| vec![],
                renderer: |c: &ValidatorContext| {
                    let (_parameters_head, parameters) = c.view_children("parameters");
                    let parameters = parameters.into_iter().intersperse(html! {{ "," }});
                    html! {
                        <span>
                        { "<" }{ for parameters }{ ">" }
                        </span>
                    }
                },
            },
        },
        /*
        Kind {
            name: "rust_generic_param",
            value: KindValue::Union {
                variants: &["rust_lifetime_param", "rust_type_param", "rust_const_param"],
            },
        },
        */
        Kind {
            name: "rust_lifetime_param",
            value: KindValue::Struct {
                fields: &[
                    Field {
                        name: "lifetime",
                        multiplicity: Multiplicity::Single,
                        validators: &[FieldValidator::Kind("rust_lifetime_or_label")],
                    },
                    Field {
                        name: "bounds",
                        multiplicity: Multiplicity::Single,
                        validators: &[FieldValidator::Kind("rust_lifetime_bounds")],
                    },
                ],
                inner: None,
                constructors: &["'"],
                validator: |_c: &ValidatorContext| vec![],
                renderer: |c: &ValidatorContext| {
                    let lifetime = c.view_child("lifetime");
                    let bounds = c.view_child("bounds");
                    html! {
                        <span>
                        { lifetime }{ ":" }{ bounds }
                        </span>
                    }
                },
            },
        },
        Kind {
            name: "rust_type_param",
            value: KindValue::Struct {
                fields: &[
                    Field {
                        name: "identifier",
                        multiplicity: Multiplicity::Single,
                        validators: &[RUST_IDENTIFIER],
                    },
                    Field {
                        name: "bounds",
                        multiplicity: Multiplicity::Single,
                        validators: &[RUST_IDENTIFIER],
                    },
                    Field {
                        name: "type",
                        multiplicity: Multiplicity::Single,
                        validators: RUST_TYPE,
                    },
                ],
                inner: None,
                constructors: &["type"],
                validator: |_c: &ValidatorContext| vec![],
                renderer: |c: &ValidatorContext| {
                    let identifier = c.view_child("identifier");
                    let bounds = c.view_child("bounds");
                    let type_ = c.view_child("type");
                    html! {
                        <span>
                        { identifier }{ ":" }{ bounds }{ "=" }{ type_ }
                        </span>
                    }
                },
            },
        },
        Kind {
            name: "rust_const_param",
            value: KindValue::Struct {
                fields: &[
                    Field {
                        name: "identifier",
                        multiplicity: Multiplicity::Single,
                        validators: &[RUST_IDENTIFIER],
                    },
                    Field {
                        name: "type",
                        multiplicity: Multiplicity::Single,
                        validators: RUST_TYPE,
                    },
                ],
                inner: None,
                constructors: &["const"],
                validator: |_c: &ValidatorContext| vec![],
                renderer: |c: &ValidatorContext| {
                    let identifier = c.view_child("identifier");
                    let type_ = c.view_child("type");
                    html! {
                        <span>
                        { "const" }{ identifier }{ ":" }{ type_ }
                        </span>
                    }
                },
            },
        },
        // https://doc.rust-lang.org/nightly/reference/items/generics.html#where-clauses
        Kind {
            name: "rust_where_clause",
            value: KindValue::Struct {
                fields: &[Field {
                    name: "items",
                    validators: &[
                        FieldValidator::Kind("rust_lifetime_where_clause_item"),
                        FieldValidator::Kind("rust_type_bound_where_clause_item"),
                    ],
                    multiplicity: Multiplicity::Repeated,
                }],
                inner: None,
                constructors: &["where"],
                validator: |_c: &ValidatorContext| vec![],
                renderer: |c: &ValidatorContext| {
                    let (_items_head, items) = c.view_children("items");
                    let items = items.into_iter().intersperse(html! {{ "," }});
                    html! {
                        <span>
                        { "where" }{ for items }
                        </span>
                    }
                },
            },
        },
        Kind {
            name: "rust_lifetime_where_clause_item",
            value: KindValue::Struct {
                fields: &[
                    Field {
                        name: "lifetime",
                        validators: &[FieldValidator::Kind("rust_lifetime")],
                        multiplicity: Multiplicity::Single,
                    },
                    Field {
                        name: "bounds",
                        validators: &[FieldValidator::Kind("rust_lifetime_bounds")],
                        multiplicity: Multiplicity::Single,
                    },
                ],
                inner: None,
                constructors: &["'"],
                validator: |_c: &ValidatorContext| vec![],
                renderer: |c: &ValidatorContext| {
                    let lifetime = c.view_child("lifetime");
                    let bounds = c.view_child("bounds");
                    html! {
                        <span>
                        { lifetime }{ ":" }{ bounds }
                        </span>
                    }
                },
            },
        },
        Kind {
            name: "rust_type_bound_where_clause_item",
            value: KindValue::Struct {
                fields: &[
                    Field {
                        name: "type",
                        validators: &[FieldValidator::Kind("rust_type")],
                        multiplicity: Multiplicity::Single,
                    },
                    Field {
                        name: "bounds",
                        validators: &[FieldValidator::Kind("rust_type_param_bounds")],
                        multiplicity: Multiplicity::Single,
                    },
                ],
                inner: None,
                constructors: &["where"],
                validator: |_c: &ValidatorContext| vec![],
                renderer: |c: &ValidatorContext| {
                    let type_ = c.view_child("type");
                    let bounds = c.view_child("bounds");
                    html! {
                        <span>
                        { type_ }{ ":" }{ bounds }
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
                        multiplicity: Multiplicity::Single,
                        validators: &[FieldValidator::Kind("rust_pattern")],
                    },
                    Field {
                        name: "type",
                        multiplicity: Multiplicity::Single,
                        validators: RUST_TYPE,
                    },
                    Field {
                        name: "value", // Expression
                        multiplicity: Multiplicity::Single,
                        validators: &[FieldValidator::Kind("rust_expression")],
                    },
                ],
                inner: Some("value"),
                constructors: &["let"],
                validator: |_c: &ValidatorContext| vec![],
                renderer: |c: &ValidatorContext| {
                    let pattern = c.view_child("pattern");
                    let type_ = c.view_child("type");
                    let value = c.view_child("value");
                    html! {
                        <span>{ "let" }{ pattern }{ ":" }{ type_ }{ "=" }{ value }</span>
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
                        multiplicity: Multiplicity::Single,
                        validators: RUST_EXPRESSION,
                    },
                    Field {
                        name: "arguments",
                        multiplicity: Multiplicity::Repeated,
                        validators: RUST_EXPRESSION,
                    },
                ],
                inner: Some("expression"),
                constructors: &["function_call"],
                validator: |_c: &ValidatorContext| vec![],
                renderer: |c: &ValidatorContext| {
                    let expression = c.view_child("expression");
                    let (_args_head, args) = c.view_children("arguments");
                    let args = args.into_iter().intersperse(html! {{ "," }});
                    html! {
                        <span>
                        { expression }
                        { "(" }{ for args }{ ")" }
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
                    validators: &[FieldValidator::Kind("rust_expression")],
                    multiplicity: Multiplicity::Repeated,
                }],
                inner: Some("elements"),
                constructors: &["tuple"],
                validator: |_c: &ValidatorContext| vec![],
                renderer: |c: &ValidatorContext| {
                    let (_elements_head, elements) = c.view_children("elements");
                    let elements = elements.into_iter().intersperse(html! {{ "," }});
                    html! {
                        <span>
                        { "(" }{ for elements }{ ")" }
                        </span>
                    }
                },
            },
        },
        // https://doc.rust-lang.org/stable/reference/expressions/struct-expr.html
        Kind {
            name: "rust_struct_expr_struct",
            value: KindValue::Struct {
                fields: &[
                    Field {
                        name: "type",
                        multiplicity: Multiplicity::Repeated,
                        validators: RUST_TYPE,
                    },
                    Field {
                        name: "fields",
                        validators: &[FieldValidator::Kind("rust_struct_expr_field")],
                        multiplicity: Multiplicity::Repeated,
                    },
                ],
                inner: Some("elements"),
                constructors: &["struct_expr_struct"],
                validator: |_c: &ValidatorContext| vec![],
                renderer: |c: &ValidatorContext| {
                    let identifier = c.view_child("type");
                    let (_fields_head, fields) = c.view_children("fields");
                    html! {
                        <span>
                        { identifier }{ "{" }{ for fields }{ "}" }
                        </span>
                    }
                },
            },
        },
        Kind {
            name: "rust_struct_expr_field",
            value: KindValue::Struct {
                fields: &[
                    Field {
                        name: "identifier",
                        multiplicity: Multiplicity::Single,
                        validators: &[RUST_IDENTIFIER],
                    },
                    Field {
                        name: "value",
                        multiplicity: Multiplicity::Single,
                        validators: RUST_EXPRESSION,
                    },
                ],
                inner: Some("elements"),
                constructors: &["struct_expr_field"],
                validator: |_c: &ValidatorContext| vec![],
                renderer: |c: &ValidatorContext| {
                    let identifier = c.view_child("type");
                    let value = c.view_child("value");
                    html! {
                        <span>
                        { identifier }{ ":" }{ value }
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
                        multiplicity: Multiplicity::Single,
                        validators: &[RUST_IDENTIFIER],
                    },
                    Field {
                        name: "fields",
                        multiplicity: Multiplicity::Repeated,
                        validators: &[FieldValidator::Kind("rust_struct_field")],
                    },
                ],
                inner: None,
                constructors: &["struct"],
                validator: |_c: &ValidatorContext| vec![],
                renderer: |c: &ValidatorContext| {
                    let identifier = c.view_child("identifier");
                    let (_fields_head, fields) = c.view_children("fields");
                    let fields = fields.into_iter().map(|v| {
                        html! {
                            <div class="indent">{ v }{ "," }</div>
                        }
                    });

                    html! {
                        <span>
                        <span class="keyword">{ "struct" }</span>{ identifier }
                        { "{" }{ for fields }{ "}" }
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
                        multiplicity: Multiplicity::Single,
                        validators: &[FieldValidator::Kind("rust_visibility")],
                    },
                    Field {
                        name: "identifier",
                        multiplicity: Multiplicity::Single,
                        validators: &[RUST_IDENTIFIER],
                    },
                    Field {
                        name: "type", // Type
                        multiplicity: Multiplicity::Single,
                        validators: RUST_TYPE,
                    },
                ],
                inner: None,
                constructors: &["struct_field"],
                validator: |_c: &ValidatorContext| vec![],
                renderer: |c: &ValidatorContext| {
                    let visibility = c.view_child("visibility");
                    let identifier = c.view_child("identifier");
                    let type_ = c.view_child("type");
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
                        multiplicity: Multiplicity::Single,
                        validators: &[RUST_IDENTIFIER],
                    },
                    Field {
                        name: "generic",
                        multiplicity: Multiplicity::Single,
                        validators: &[FieldValidator::Kind("rust_generic_params")],
                    },
                    Field {
                        name: "where",
                        multiplicity: Multiplicity::Single,
                        validators: &[FieldValidator::Kind("rust_where_clause")],
                    },
                    Field {
                        name: "items",
                        multiplicity: Multiplicity::Repeated,
                        validators: &[FieldValidator::Kind("rust_enum_item")],
                    },
                ],
                inner: None,
                constructors: &["enum"],
                validator: |_c: &ValidatorContext| vec![],
                renderer: |c: &ValidatorContext| {
                    let identifier = c.view_child("identifier");
                    let generic = c.view_child("generic");
                    let where_ = c.view_child("where");

                    let (_items_head, items) = c.view_children("items");
                    let items = items.into_iter().map(|v| {
                        html! {
                            <div class="indent">{ v }{ "," }</div>
                        }
                    });

                    html! {
                        <span>
                        <span class="keyword">{ "enum" }</span>{ identifier }{ generic }{ where_ }
                        { "{" }{ for items }{ "}" }
                        </span>
                    }
                },
            },
        },
        Kind {
            name: "rust_enum_item",
            value: KindValue::Struct {
                fields: &[
                    Field {
                        name: "visibility",
                        multiplicity: Multiplicity::Single,
                        validators: &[FieldValidator::Kind("rust_visibility")],
                    },
                    Field {
                        name: "identifier",
                        multiplicity: Multiplicity::Single,
                        validators: &[RUST_IDENTIFIER],
                    },
                    Field {
                        name: "inner",
                        multiplicity: Multiplicity::Single,
                        validators: &[
                            FieldValidator::Kind("rust_enum_item_tuple"),
                            FieldValidator::Kind("rust_enum_item_struct"),
                            FieldValidator::Kind("rust_enum_discriminant"),
                        ],
                    },
                ],
                inner: None,
                constructors: &["enum_variant"],
                validator: |_c: &ValidatorContext| vec![],
                renderer: |c: &ValidatorContext| {
                    let visibility = c.view_child("visibility");
                    let identifier = c.view_child("identifier");
                    let inner = c.view_child("inner");

                    html! {
                        <span>
                        { visibility }{ identifier }{ inner }
                        </span>
                    }
                },
            },
        },
        /*
        Kind {
            name: "rust_enum_item_inner",
            value: KindValue::Union {
                variants: &[
                    "rust_enum_item_tuple",
                    "rust_enum_item_struct",
                    "rust_enum_item_discriminant",
                ],
            },
        },
        */
        Kind {
            name: "rust_enum_item_tuple",
            value: KindValue::Struct {
                fields: &[Field {
                    name: "fields",
                    multiplicity: Multiplicity::Repeated,
                    validators: &[FieldValidator::Kind("rust_tuple_field")],
                }],
                inner: None,
                constructors: &["tuple"],
                validator: |_c: &ValidatorContext| vec![],
                renderer: |c: &ValidatorContext| {
                    let (_fields_head, fields) = c.view_children("fields");
                    let fields = fields
                        .into_iter()
                        .intersperse(html! { <span>{ "," }</span>});

                    html! {
                        <span>
                        { "(" }{ for fields }{ ")" }
                        </span>
                    }
                },
            },
        },
        Kind {
            name: "rust_enum_item_struct",
            value: KindValue::Struct {
                fields: &[Field {
                    name: "fields",
                    multiplicity: Multiplicity::Repeated,
                    validators: &[FieldValidator::Kind("rust_struct_field")],
                }],
                inner: None,
                constructors: &["struct"],
                validator: |_c: &ValidatorContext| vec![],
                renderer: |c: &ValidatorContext| {
                    let (_fields_head, fields) = c.view_children("fields");
                    let fields = fields.into_iter().map(|v| {
                        html! {
                            <div class="indent">{ v }{ "," }</div>
                        }
                    });

                    html! {
                        <span>
                        { "{" }{ for fields }{ "}" }
                        </span>
                    }
                },
            },
        },
        Kind {
            name: "rust_enum_item_discriminant",
            value: KindValue::Struct {
                fields: &[Field {
                    name: "value",
                    multiplicity: Multiplicity::Single,
                    validators: RUST_EXPRESSION,
                }],
                inner: None,
                constructors: &["discriminant"],
                validator: |_c: &ValidatorContext| vec![],
                renderer: |c: &ValidatorContext| {
                    let value = c.view_child("value");
                    html! {
                        <span>
                        { "=" }{ value }
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
                    multiplicity: Multiplicity::Repeated,
                    validators: MARKDOWN_ITEM,
                }],
                inner: Some("items"),
                constructors: &["markdown_fragment"],
                validator: |_c: &ValidatorContext| vec![],
                renderer: |c: &ValidatorContext| {
                    let (_items_head, items) = c.view_children("items");
                    let items = items.into_iter().map(|v| {
                        html! {
                            <div>{ v }</div>
                        }
                    });
                    html! {
                        <div>
                            <div class="fragment-type">{ "markdown" }</div>
                            { for items }
                        </div>
                    }
                },
            },
        },
        /*
        Kind {
            name: "markdown_item",
            value: KindValue::Union {
                variants: &[
                    "markdown_paragraph",
                    "markdown_heading",
                    "markdown_code",
                    "markdown_quote",
                    "markdown_list",
                ],
            },
        },
        */
        Kind {
            name: "markdown_code",
            value: KindValue::Struct {
                fields: &[
                    Field {
                        name: "lang",
                        multiplicity: Multiplicity::Single,
                        validators: &[FieldValidator::Literal(any)],
                    },
                    Field {
                        name: "text",
                        multiplicity: Multiplicity::Single,
                        validators: &[FieldValidator::Literal(any)],
                    },
                ],
                inner: Some("text"),
                constructors: &["heading"],
                validator: |_c: &ValidatorContext| vec![],
                renderer: |c: &ValidatorContext| {
                    let level = c.view_child("level");
                    let text = c.view_child("text");
                    html! {
                        <span>
                        { "#" }{ level}{ text }
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
                        multiplicity: Multiplicity::Single,
                        // XXX
                        validators: &[],
                    },
                    Field {
                        name: "text",
                        multiplicity: Multiplicity::Single,
                        // XXX
                        validators: &[MARKDOWN_PARAGRAPH],
                    },
                ],
                inner: Some("text"),
                constructors: &["heading"],
                validator: |_c: &ValidatorContext| vec![],
                renderer: |c: &ValidatorContext| {
                    let level = c.view_child("level");
                    let text = c.view_child("text");
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
                    multiplicity: Multiplicity::Repeated,
                    validators: &[FieldValidator::Kind("markdown_paragraph")],
                }],
                inner: Some("items"),
                constructors: &["list"],
                validator: |_c: &ValidatorContext| vec![],
                renderer: |c: &ValidatorContext| {
                    let (_items_head, items) = c.view_children("items");
                    let items = items.into_iter().map(|v| {
                        html! {
                            <li>{ v }</li>
                        }
                    });
                    html! {
                        <span>
                            <ul class="list-disc">
                                { for items }
                            </ul>
                        </span>
                    }
                },
            },
        },
    ],
};

// Generate valid values.
type Renderer = fn(&ValidatorContext) -> Html;
type Validator = fn(&ValidatorContext) -> Vec<ValidationError>;

pub fn default_renderer(c: &ValidatorContext) -> Html {
    let node = &c.node;
    let path = &c.path;
    log::debug!("default_renderer: {:?}", path);
    let kind = SCHEMA.get_kind(&node.kind);
    // Node.
    // https://codepen.io/xotonic/pen/JRLAOR
    let children: Vec<_> = node
        .children
        .iter()
        .flat_map(|(field_name, hashes)| {
            let field_schema = kind.and_then(|k| k.get_field(field_name));
            let validators = field_schema
                .map(|v| v.validators.clone())
                .unwrap_or_default();
            let path = path.clone();
            hashes.iter().enumerate().map(move |(i, h)| {
                let child_path = append(
                    &path,
                    Selector {
                        field: field_name.clone(),
                        index: i,
                    },
                );
                // TODO: Sticky field headers.
                html! {
                    <div class="pl-3 flex items-start"
                    //   onclick={ onclick }
                    >
                        <div class={ FIELD_CLASSES.join(" ") }>
                            { field_name }
                        </div>
                        <div class="">
                            { ":" }
                        </div>
                        { c.view_child_index(field_name, i) }
                    </div>
                }
            })
        })
        .collect();
    html! {
        <div class="divide-y divide-black border-t border-b border-black border-solid">
            { for children }
        </div>
    }
}

pub struct ValidatorContext {
    pub path: Vec<Selector>,
    pub cursor: Vec<Selector>,
    pub file: Rc<File>,
    pub node: Node,
    pub onselect: Callback<Vec<Selector>>,
    pub updatemodel: Callback<Msg>,
}

impl ValidatorContext {
    pub fn view_child(&self, field_name: &str) -> Html {
        self.view_child_index(field_name, 0)
    }
    pub fn view_child_index(&self, field_name: &str, index: usize) -> Html {
        log::debug!("view_child: {:?}", field_name);
        if self.node.children.get(field_name).is_none() {
            return html! {};
        }
        if self.node.children.get(field_name).unwrap().is_empty() {
            return html! {};
        }
        let h = &self.node.children.get(field_name).unwrap()[index];
        let child_path = append(
            &self.path,
            Selector {
                field: field_name.to_string(),
                index: index,
            },
        );
        let kind = SCHEMA.get_kind(&self.node.kind);
        let field_schema = kind.and_then(|k| k.get_field(field_name));
        let validators = field_schema
            .map(|v| v.validators.clone())
            .unwrap_or_default();
        html! {
            // <div>
            //   { format!("{:?} {:?}", h, child_path) }
            // </div>
            <NodeComponent
                file={ self.file.clone() }
                hash={ h.clone() }
                cursor={ self.cursor.clone() }
                onselect={ self.onselect.clone() }
                path={ child_path }
                updatemodel={ self.updatemodel.clone() }
                validators={ validators }
            />
        }
    }
    pub fn view_children(&self, field_name: &str) -> (Html, Vec<Html>) {
        log::debug!("view_child: {:?}", field_name);
        if self.node.children.get(field_name).is_none() {
            return (html! {}, vec![]);
        }
        if self.node.children.get(field_name).unwrap().is_empty() {
            return (html! {}, vec![]);
        }
        (
            html! {},
            self.node
                .children
                .get(field_name)
                .unwrap()
                .iter()
                .enumerate()
                .map(|(i, h)| self.view_child_index(field_name, i))
                .collect(),
        )
        // self.model
        //     .view_children(self.ctx, self.node, field_name, self.path)
    }
    // TODO: field / child.
}

#[derive(Debug, PartialEq)]
pub struct ValidationError {
    // TODO: Path vs Ref?????
    path: Path,
    pub message: String,
}

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
    pub value: KindValue,
}

pub enum KindValue {
    Struct {
        fields: &'static [Field],
        inner: Option<&'static str>,
        constructors: &'static [&'static str],
        validator: Validator,
        renderer: Renderer,
    },
}

impl Kind {
    pub fn get_field(&self, field: &str) -> Option<Field> {
        self.get_fields().into_iter().find(|f| f.name == field)
    }

    pub fn get_fields(&self) -> Vec<Field> {
        match self.value {
            KindValue::Struct { fields, .. } => fields.iter().cloned().collect(),
        }
    }

    pub fn render(&self, context: &ValidatorContext) -> Html {
        match self.value {
            KindValue::Struct { renderer, .. } => renderer(context),
        }
    }

    pub fn validator(&self, context: &ValidatorContext) -> Vec<ValidationError> {
        match self.value {
            KindValue::Struct { validator, .. } => validator(context),
        }
    }

    /*
    pub fn constructors(&self) -> Vec<ParsedValue> {
        match self.value {
            KindValue::Struct { constructors, .. } => constructors
                .into_iter()
                .map(|value| ParsedValue {
                    kind_hierarchy: vec![self.name.to_string()],
                    label: value.to_string(),
                    value: value.to_string(),
                })
                .collect(),
        }
    }
    */
}

#[derive(Clone, Debug, PartialEq)]
pub struct ParsedValue {
    pub kind_hierarchy: Vec<String>,
    pub label: String,
    pub value: String,
}

impl ParsedValue {
    pub fn to_node(&self) -> Node {
        Node {
            kind: self.kind_hierarchy.last().unwrap().clone(),
            value: self.value.clone(),
            children: BTreeMap::new(),
        }
    }
}

#[derive(Clone)]
pub struct Field {
    pub name: &'static str,
    // pub kind: &'static [&'static str],
    pub multiplicity: Multiplicity,
    pub validators: &'static [FieldValidator],
}

#[derive(Clone)]
pub enum Multiplicity {
    // Required -- show hole if not present
    // Optional -- hide if not present
    Single,
    Repeated,
}

pub enum FieldValidator {
    Kind(&'static str),
    Literal(fn(&str) -> Vec<ValidationError>),
}

impl PartialEq for FieldValidator {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Kind(l0), Self::Kind(r0)) => l0 == r0,
            (Self::Literal(l0), Self::Literal(r0)) => true,
            (_, _) => false,
        }
    }
}

// TODO: Replace parser with validator fn that can return errors on the node itself or its children
// via relative paths. In this way we don't need to validate literals when parsing, but we can do
// another pass later and highlight errors (or in real time). For instance, fields with invalid
// identifiers, or fields in a struct with duplicate names may be highlighted in this way.
