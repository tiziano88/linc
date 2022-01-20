use crate::{
    model::{GlobalState, Model, Msg},
    node::{NodeComponent, KIND_CLASSES},
    types::{append, display_selector_text, Cursor, Node, Path, Selector},
};
use maplit::hashmap;
use std::{
    collections::{BTreeMap, HashMap},
    rc::Rc,
};
use yew::prelude::*;

type UUID = String;

// Schema of the schema.
pub struct Schema {
    kinds: HashMap<&'static str, Kind>,
}

impl Schema {
    pub fn get_kind(&self, kind: &str) -> Option<&Kind> {
        self.kinds.get(kind)
    }
}

#[derive(Default)]
pub struct Kind {
    pub name: &'static str,
    pub fields: HashMap<usize, Field>,
    pub renderer: Option<Renderer>,
}

impl Kind {
    pub fn get_fields(&self) -> Vec<(&usize, &Field)> {
        self.fields.iter().collect()
    }
    pub fn get_field(&self, field_id: usize) -> Option<&Field> {
        self.fields.get(&field_id)
    }
}

#[derive(Default)]
pub struct Field {
    pub name: &'static str,
    pub repeated: bool,
    pub required: bool,
    pub raw: bool,
    pub types: &'static [&'static str],
}

fn comma() -> Html {
    html! {
        <span>{ "," }</span>
    }
}

fn semicolon() -> Html {
    html! {
        <span>{ ";" }</span>
    }
}

macro_rules! primitive {
    ($prefix:ident, $value:tt) => {
        Kind {
            name: concat!(stringify!($prefix), "_", stringify!($value)),
            fields: hashmap! {},
            renderer: Some(|_c| {
                html! {
                    <span class="type">
                        { stringify!($value) }
                    </span>
                }
            }),
        }
    };
}

macro_rules! schema {
    ( $n:ident,
        $($type_uuid:literal => $type_ident:ident @ $it:expr ,)*
    ) => {
        $(pub const $type_ident : &'static str = $type_uuid;)*
        fn $n() -> Schema {
            Schema {
                kinds: hashmap! {
                    $( $type_ident => $it ),*
                },
            }
        }
    }
}

// TODO: transformations between nodes, e.g. type -> array.

pub static SCHEMA: std::lazy::SyncLazy<Schema> = std::lazy::SyncLazy::new(create_schema);

static RUST_TYPE: &[&'static str] = &[
    RUST_ARRAY_TYPE,
    RUST_PRIMITIVE_TYPE_BOOL,
    RUST_PRIMITIVE_TYPE_CHAR,
    RUST_PRIMITIVE_TYPE_STR,
    RUST_PRIMITIVE_TYPE_U8,
    RUST_PRIMITIVE_TYPE_I8,
    RUST_PRIMITIVE_TYPE_U32,
    RUST_PRIMITIVE_TYPE_I32,
    RUST_PRIMITIVE_TYPE_U64,
    RUST_PRIMITIVE_TYPE_I64,
    RUST_PRIMITIVE_TYPE_USIZE,
    RUST_PRIMITIVE_TYPE_ISIZE,
    RUST_PRIMITIVE_TYPE_F32,
    RUST_PRIMITIVE_TYPE_F64,
];
static RUST_VISIBILITY: &[&'static str] = &[
    RUST_VISIBILITY_PUB,
    RUST_VISIBILITY_PUB_CRATE,
    RUST_VISIBILITY_PUB_SELF,
];
static RUST_EXPRESSION: &[&'static str] = &[
    RUST_IF,
    RUST_MATCH,
    RUST_STRING_LITERAL,
    RUST_FUNCTION_CALL,
    RUST_BINARY_OPERATOR,
];
static RUST_OPERATOR: &[&'static str] = &[
    RUST_OPERATOR_PLUS,
    RUST_OPERATOR_MINUS,
    RUST_OPERATOR_MUL,
    RUST_OPERATOR_DIV,
    RUST_OPERATOR_MOD,
    RUST_OPERATOR_AND,
    RUST_OPERATOR_OR,
    RUST_OPERATOR_XOR,
    RUST_OPERATOR_EQ,
    RUST_OPERATOR_NE,
    RUST_OPERATOR_LT,
    RUST_OPERATOR_GT,
    RUST_OPERATOR_LE,
    RUST_OPERATOR_GE,
    RUST_OPERATOR_BOOL_OR,
    RUST_OPERATOR_BOOL_AND,
];

static GO_STATEMENT: &[&'static str] = &[
    GO_ASSIGNMENT,
    GO_VARIABLE_DECLARATION,
    GO_FUNCTION_CALL,
    GO_IF,
];
static GO_EXPRESSION: &[&'static str] = &[GO_STRING_LITERAL, GO_FUNCTION_CALL];

schema! {
    create_schema,
    "d76d88c5-2094-48b4-b4ed-dbf8df15fa59" => ROOT @ Kind {
        name: "root",
        fields: hashmap!{
            0 => Field {
                name: "item",
                types: &[
                    GIT,
                    DOCKER,
                    RUST_FRAGMENT,
                    GO_FRAGMENT,
                    MARKDOWN_FRAGMENT,
                    RUST_CARGO_TOML,
                ],
                ..Default::default()
            },
        },
        ..Default::default()
    },

    "7bd45e4c-3c25-48b7-b247-b7bd2c67c6cc" => DOCKER @ Kind {
        name: "docker",
        fields: hashmap!{
            0 => Field {
                name: "command",
                types: &[DOCKER_BUILD, DOCKER_RUN],
                ..Default::default()
            },
        },
        ..Default::default()
    },
    "33ac449e-bbae-44f1-bcae-fa85f1b93e67" => DOCKER_BUILD @ Kind {
        name: "docker_build",
        fields: hashmap!{
            0 => Field {
                name: "add-host",
                raw: true,
                ..Default::default()
            },
            1 => Field {
                name: "build-arg",
                raw: true,
                repeated: true,
                ..Default::default()
            },
            2 => Field {
                name: "cache-from",
                raw: true,
                repeated: true,
                ..Default::default()
            },
            3 => Field {
                name: "compress",
                raw: true,
                ..Default::default()
            },
            4 => Field {
                name: "file",
                raw: true,
                ..Default::default()
            },
            5 => Field {
                name: "label",
                raw: true,
                ..Default::default()
            },
        },
        ..Default::default()
    },
    "688b91b7-c99c-4d5e-b996-6c5df6e01c14" => DOCKER_RUN @ Kind {
        name: "docker_run",
        fields: hashmap!{
            0 => Field {
                name: "attach",
                raw: true,
                ..Default::default()
            },
            1 => Field {
                name: "cap-add",
                raw: true,
                repeated: true,
                ..Default::default()
            },
            2 => Field {
                name: "cap-drop",
                raw: true,
                repeated: true,
                ..Default::default()
            },
            2 => Field {
                name: "detach",
                raw: true,
                ..Default::default()
            },
        },
        ..Default::default()
    },

    "7505f498-c04f-4180-a99d-c39a1abb1590" => GIT @ Kind {
        name: "git",
        fields: hashmap!{
            0 => Field {
                name: "command",
                types: &[GIT_COMMIT],
                ..Default::default()
            },
        },
        ..Default::default()
    },
    "aed19b7c-c92c-4a29-acef-4a813de0cd4d" => GIT_COMMIT @ Kind {
        name: "git_commit",
        fields: hashmap!{
            0 => Field {
                name: "message",
                raw: true,
                ..Default::default()
            },
            1 => Field {
                name: "author",
                raw: true,
                ..Default::default()
            },
            2 => Field {
                name: "date",
                raw: true,
                ..Default::default()
            },
            3 => Field {
                name: "interactive",
                raw: true,
                ..Default::default()
            },
            4 => Field {
                name: "amend",
                raw: true,
                ..Default::default()
            },
            5 => Field {
                name: "squash",
                raw: true,
                ..Default::default()
            },
            6 => Field {
                name: "fixup",
                raw: true,
                ..Default::default()
            },
        },
        ..Default::default()
    },
    "23fe18b7-f36d-4fd4-abcd-c605c927ca93" => GIT_STATUS @ Kind {
        name: "git_status",
        fields: hashmap!{
            0 => Field {
                name: "short",
                raw: true,
                ..Default::default()
            },
            1 => Field {
                name: "branch",
                raw: true,
                ..Default::default()
            },
            2 => Field {
                name: "porcelain",
                raw: true,
                ..Default::default()
            },
            3 => Field {
                name: "long",
                raw: true,
                ..Default::default()
            },
            4 => Field {
                name: "verbose",
                raw: true,
                ..Default::default()
            },
            5 => Field {
                name: "ignore-submodules",
                raw: true,
                ..Default::default()
            },
        },
        ..Default::default()
    },

    "e9687f8a-f22c-4650-a3d6-d075428ee648" => RUST_FRAGMENT @ Kind {
        name: "rust_fragment",
        fields: hashmap!{
            0 => Field {
                name: "items",
                repeated: true,
                types: &[RUST_VIS_ITEM],
                ..Default::default()
            },
        },
        renderer: Some(|c| {
            let items = c.view_children(0).into_iter().map(|b| {
                html! {
                    <div>{ b }</div>
                }
            });
            html! {
                <div>
                <div class="fragment-type">{ "rust" }</div>
                { for items }
                </div>
            }
        }),
        ..Default::default()
    },
    "a3aac07e-c452-4e52-887a-530b1677cd13" => RUST_STRING_LITERAL @ Kind {
        name: "rust_string_literal",
        fields: hashmap!{
            0 => Field {
                name: "value",
                raw: true,
                ..Default::default()
            },
        },
        renderer: Some(|c| {
            html! {
                <span>
                    { "\"" }{ c.view_child(0) }{ "\"" }
                </span>
            }
        }),
        ..Default::default()
    },
    "4f837305-9e07-402b-a98f-563e34e29125" => RUST_VIS_ITEM @ Kind {
        name: "rust_vis_item",
        fields: hashmap!{
            0 => Field {
                name: "visibility",
                types: RUST_VISIBILITY,
                ..Default::default()
            },
            1 => Field {
                name: "item",
                types: &[
                    RUST_CONSTANT,
                    RUST_ENUM,
                    RUST_FUNCTION,
                ],
                ..Default::default()
            },
        },
        renderer: Some(|c| {
            html! {
                <div>{ c.view_child(0) }{ c.view_child(1) }</div>
            }
        }),
        ..Default::default()
    },
    "5599fb59-61bf-4216-87c3-e38aa4f6b109" => RUST_ENUM @ Kind {
        name: "rust_enum",
        fields: hashmap!{
            0 => Field {
                name: "identifier",
                raw: true,
                ..Default::default()
            },
            1 => Field {
                name: "generic",
                types: RUST_TYPE,
                ..Default::default()
            },
            2 => Field {
                name: "where",
                types: &[],
                ..Default::default()
            },
            3 => Field {
                name: "items",
                repeated: true,
                types: &[RUST_ENUM_ITEM],
                ..Default::default()
            },
        },
        renderer: Some(|c| {
            let items = c.view_children(3).into_iter().map(|v| {
                html! {
                    <div class="indent">{ v }{ "," }</div>
                }
            });

            html! {
                <div>
                    <span class="keyword">{ "enum" }</span>{ c.view_child(0) }{ c.view_child(1) }{ c.view_child(2) }
                    { "{" }{ for items }{ "}" }
                </div>
            }
        }),
        ..Default::default()
    },
    "10ad3193-9f92-4dbc-baf5-e785440d4ca0" => RUST_ENUM_ITEM @ Kind {
        name: "rust_enum_item",
        fields: hashmap!{
            0 => Field {
                name: "visibility",
                raw: true,
                types: RUST_VISIBILITY,
                ..Default::default()
            },
            1 => Field {
                name: "identifier",
                raw: true,
                ..Default::default()
            },
            2 => Field {
                name: "inner",
                ..Default::default()
            },
        },
        renderer: Some(|c| {
            html! {
                <span>
                    { c.view_child(0) }{ c.view_child(1) }{ c.view_child(2) }
                </span>
            }
        }),
        ..Default::default()
    },
    "9cbb3d22-1a64-4c71-9675-e1d8231222f1" => RUST_PRIMITIVE_TYPE_BOOL @ primitive!(rust_primitive_type, bool),
    "8c829b5a-52e1-45ca-978a-3e0fa0d5663d" => RUST_PRIMITIVE_TYPE_CHAR @ primitive!(rust_primitive_type, char),
    "f3c3af21-8cb4-4c2e-9076-a926b4fab56d" => RUST_PRIMITIVE_TYPE_STR @ primitive!(rust_primitive_type, str),
    "df6d27ac-9d32-46e5-b8dd-bb3c43243a55" => RUST_PRIMITIVE_TYPE_U8 @ primitive!(rust_primitive_type, u8),
    "110d9227-5ae2-4be1-bda7-30c20eee18b9" => RUST_PRIMITIVE_TYPE_I8 @ primitive!(rust_primitive_type, i8),
    "80cfb5fe-d296-48da-9b9e-b3c8a2cdce68" => RUST_PRIMITIVE_TYPE_U32 @ primitive!(rust_primitive_type, u32),
    "3e1a7508-a56c-4e1f-8c38-60864815752a" => RUST_PRIMITIVE_TYPE_I32 @ primitive!(rust_primitive_type, i32),
    "4f5d623c-fdcf-4818-9dc3-d0e651968200" => RUST_PRIMITIVE_TYPE_U64 @ primitive!(rust_primitive_type, u64),
    "c3032ed2-d82d-4cc2-8964-d128394b4185" => RUST_PRIMITIVE_TYPE_I64 @ primitive!(rust_primitive_type, i64),
    "2a92015e-2790-413a-a595-6e61ea0e9441" => RUST_PRIMITIVE_TYPE_USIZE @ primitive!(rust_primitive_type, usize),
    "f37bbd39-06ff-4734-8095-cde929558f16" => RUST_PRIMITIVE_TYPE_ISIZE @ primitive!(rust_primitive_type, isize),
    "18f4617a-1744-408c-86f8-eef3b0056917" => RUST_PRIMITIVE_TYPE_F32 @ primitive!(rust_primitive_type, f32),
    "a1e3b48f-b712-424a-af0a-8c07c7524181" => RUST_PRIMITIVE_TYPE_F64 @ primitive!(rust_primitive_type, f64),

    "3eb326f6-769b-4998-acbc-4d9184d34360" => RUST_OPERATOR_PLUS @ primitive!(rust_primitive_type, +),
    "3c0f8733-b295-4fbb-b573-7bb1ee2d4846" => RUST_OPERATOR_MINUS @ primitive!(rust_primitive_type, -),
    "f93f85b8-09ce-4e80-8f4f-67555708c6b6" => RUST_OPERATOR_MUL @ primitive!(rust_primitive_type, *),
    "919e365d-8599-461c-8dad-42506db2b1d9" => RUST_OPERATOR_DIV @ primitive!(rust_primitive_type, /),
    "b814f4d9-285b-4124-895a-7c36ab386811" => RUST_OPERATOR_MOD @ primitive!(rust_primitive_type, %),
    "8aa441cd-ab27-49c2-98d9-5a4144b1e7a9" => RUST_OPERATOR_AND @ primitive!(rust_primitive_type, &),
    "1aa0cc0f-611c-4c2a-951c-df2e08396764" => RUST_OPERATOR_OR @ primitive!(rust_primitive_type, |),
    "2bdc039e-2462-4c67-91fc-ba6634379764" => RUST_OPERATOR_XOR @ primitive!(rust_primitive_type, ^),
    "7d0a7a81-1ed1-47fd-96d9-b320e35eb654" => RUST_OPERATOR_EQ @ primitive!(rust_primitive_type, ==),
    "8150945f-2be0-4b92-9be9-f282af85b565" => RUST_OPERATOR_NE @ primitive!(rust_primitive_type, !=),
    "1e3d0a7b-e683-4ac7-bb25-be911e2f22be" => RUST_OPERATOR_LT @ primitive!(rust_primitive_type, <),
    "eddd8b9b-5e7b-40ca-9426-efb54ebfc9e5" => RUST_OPERATOR_GT @ primitive!(rust_primitive_type, >),
    "0c081d05-7c34-4e5e-9757-88261ce0c854" => RUST_OPERATOR_LE @ primitive!(rust_primitive_type, <=),
    "c8b054e8-51d7-4065-b9ab-a53f1a8ed815" => RUST_OPERATOR_GE @ primitive!(rust_primitive_type, >=),

    "19cb99f6-af8c-4a4b-bf56-c1ae8eadfe62" => RUST_OPERATOR_BOOL_OR @ primitive!(rust_primitive_type, ||),
    "a8e31571-4570-4b77-9adb-544987ae1940" => RUST_OPERATOR_BOOL_AND @ primitive!(rust_primitive_type, &&),

    "30b95f1c-6c5f-4877-8047-ec84b570f6cf" => RUST_FUNCTION @ Kind {
        name: "rust_function",
        fields: hashmap!{
            0 => Field {
                name: "comment",
                raw: true,
                ..Default::default()
            },
            1 => Field {
                name: "const",
                raw: true,
                ..Default::default()
            },
            2 => Field {
                name: "async",
                raw: true,
                ..Default::default()
            },
            3 => Field {
                name: "unsafe",
                raw: true,
                ..Default::default()
            },
            4 => Field {
                name: "extern",
                raw: true,
                ..Default::default()
            },
            5 => Field {
                name: "identifier",
                raw: true,
                ..Default::default()
            },
            6 => Field {
                name: "generic",
                raw: true,
                ..Default::default()
            },
            7 => Field {
                name: "parameters",
                types: &[RUST_FUNCTION_PARAMETER],
                repeated: true,
                ..Default::default()
            },
            8 => Field {
                name: "return_type",
                types: RUST_TYPE,
                raw: true,
                ..Default::default()
            },
            9 => Field {
                name: "body",
                types: RUST_EXPRESSION,
                repeated: true,
                ..Default::default()
            },
        },
        renderer: Some(|c| {
            html! {
                <span>
                    <span class="keyword">{ "fn" }</span>
                    { c.view_child(5) }
                    { "(" }{ for c.view_children(7).into_iter().intersperse(comma()) }{ ")" }
                    { "->" }{ c.view_child(8) }
                    { "{" }{ for c.view_children(9).into_iter().intersperse(semicolon()) }{ "}" }
                </span>
            }
        }),
        ..Default::default()
    },
    "895c624e-7308-4ebd-83c8-644076613e08" => RUST_FUNCTION_PARAMETER @ Kind {
        name: "rust_function_parameter",
        fields: hashmap!{
            0 => Field {
                name: "pattern",
                raw: true,
                ..Default::default()
            },
            1 => Field {
                name: "type",
                types: RUST_TYPE,
                ..Default::default()
            },
        },
        renderer: Some(|c| {
            html! {
                <span>
                    { c.view_child(0) }{ ":" }{ c.view_child(1) }
                </span>
            }
        }),
        ..Default::default()
    },
    "65e449f1-1ab8-4f5e-b3d8-064e7d9ed222" => RUST_CONSTANT @ Kind {
        name: "rust_constant",
        fields: hashmap!{
            0 => Field {
                name: "identifier",
                raw: true,
                ..Default::default()
            },
            1 => Field {
                name: "type",
                types: RUST_TYPE,
                ..Default::default()
            },
            2 => Field {
                name: "expression",
                types: RUST_EXPRESSION,
                ..Default::default()
            },
        },
        renderer: Some(|c| {
            html! {
                <span>
                    <span class="keyword">{ "const" }</span>
                    { c.view_child(0) }
                    { ":" }
                    { c.view_child(1) }
                    { "=" }
                    { c.view_child(2) }
                    { ";" }
                </span>
            }
        }),
        ..Default::default()
    },
    "476d88e5-5b6b-496e-86b4-480a688450f9" => RUST_ARRAY_TYPE @ Kind {
        name: "rust_array_type",
        fields: hashmap!{
            0 => Field {
                name: "type",
                types: RUST_TYPE,
                ..Default::default()
            },
        },
        renderer: Some(|c| {
            html! {
                <span>
                    { "[" }{ c.view_child(0) }{ "]" }
                </span>
            }
        }),
        ..Default::default()
    },
    "9b74572d-17ec-4b76-9166-3e1916e3289a" => RUST_FUNCTION_CALL @ Kind {
        name: "rust_function_call",
        fields: hashmap!{
            0 => Field {
                name: "function",
                types: RUST_EXPRESSION,
                ..Default::default()
            },
            1 => Field {
                name: "arguments",
                types: RUST_EXPRESSION,
                repeated: true,
                ..Default::default()
            },
        },
        renderer: Some(|c| {
            html! {
                <span>
                    { c.view_child(0) }
                    { "(" }{ for c.view_children(1).into_iter().intersperse(comma()) }{ ")" }
                </span>
            }
        }),
        ..Default::default()
    },
    "c4578237-eb44-47a9-a2c2-b451575fc660" => RUST_BINARY_OPERATOR @ Kind {
        name: "rust_binary_operator",
        fields: hashmap!{
            0 => Field {
                name: "operator",
                types: RUST_OPERATOR,
                ..Default::default()
            },
            1 => Field {
                name: "left",
                types: RUST_EXPRESSION,
                ..Default::default()
            },
            2 => Field {
                name: "right",
                types: RUST_EXPRESSION,
                ..Default::default()
            },
        },
        renderer: Some(|c| {
            html! {
                <span>
                    { c.view_child(1) }{ c.view_child(0) }{ c.view_child(2) }
                </span>
            }
        }),
        ..Default::default()
    },
    "e7c7dcd0-28b1-4efd-a0ce-1d18aa60919d" => RUST_IF @ Kind {
        name: "rust_if",
        fields: hashmap!{
            0 => Field {
                name: "condition",
                types: RUST_EXPRESSION,
                ..Default::default()
            },
            1 => Field {
                name: "true_body",
                types: RUST_EXPRESSION,
                repeated: true,
                ..Default::default()
            },
            2 => Field {
                name: "false_body",
                types: RUST_EXPRESSION,
                repeated: true,
                ..Default::default()
            },
        },
        renderer: Some(|c| {
            html! {
                <span>
                    <div>
                        <span class="keyword">{ "if" }</span>{ c.view_child(0) }{ "{" }
                    </div>
                    <div class="indent">
                        { c.view_child(1) }
                    </div>
                    <div>
                        { "}" }<span class="keyword">{ "else" }</span>{ "{" }
                    </div>
                    <div class="indent">
                        { c.view_child(2) }
                    </div>
                    <div>
                        { "}" }
                    </div>
                </span>
            }
        }),
        ..Default::default()
    },
    "5bfb45f3-df68-4f7c-a218-4dbd9bcc000a" => RUST_MATCH @ Kind {
        name: "rust_match",
        fields: hashmap!{
            0 => Field {
                name: "expression",
                types: RUST_EXPRESSION,
                ..Default::default()
            },
            1 => Field {
                name: "match_arms",
                types: &[RUST_MATCH_ARM],
                repeated: true,
                ..Default::default()
            },
        },
        renderer: Some(|c| {
            let match_arms = c.view_children(1).into_iter().map(|v| {
                html! {
                    <div class="indent">{ v }{ "," }</div>
                }
            });
            html! {
                <span>
                    <div>
                        <span class="keyword">{ "match" }</span>{ c.view_child(0) }{ "{" }
                    </div>
                    { for match_arms }
                    <div>
                        { "}" }
                    </div>
                </span>
            }
        }),
        ..Default::default()
    },
    "097de557-15f1-4341-aa0a-b92cfa01002f" => RUST_MATCH_ARM @ Kind {
        name: "rust_match_arm",
        fields: hashmap!{
            0 => Field {
                name: "patterns",
                repeated: true,
                ..Default::default()
            },
            1 => Field {
                name: "guard",
                types: RUST_EXPRESSION,
                ..Default::default()
            },
            2 => Field {
                name: "expression",
                types: RUST_EXPRESSION,
                ..Default::default()
            },
        },
        renderer: Some(|c| {
            let patterns = c.view_children(0).into_iter().intersperse(html! {
                    <span>{ "|" }</span>
            });
            html! {
                <span>
                    <span>{ for patterns }</span>
                    <span>{ "if" }{ c.view_child(1) }</span>
                    <span>{ "=>" }</span>
                    <span>{ c.view_child(2) }</span>
                </span>
            }
        }),
        ..Default::default()
    },
    "003830a2-d0e4-4828-9d05-156cce62ef7a" => RUST_VISIBILITY_PUB @ Kind {
        name: "rust_visibility_pub",
        fields: hashmap!{},
        renderer: Some(|_c| {
            html! {
                <span class="keyword">{ "pub" }</span>
            }
        }),
        ..Default::default()
    },
    "dbcea439-1234-40d4-bd80-20ec35974168" => RUST_VISIBILITY_PUB_CRATE @ Kind {
        name: "rust_visibility_pub_crate",
        fields: hashmap!{},
        renderer: Some(|_c| {
            html! {
                <span class="keyword">{ "pub(crate)" }</span>
            }
        }),
        ..Default::default()
    },
    "52468e4a-f333-46b8-b814-ea2efc4d25c0" => RUST_VISIBILITY_PUB_SELF @ Kind {
        name: "rust_visibility_pub_self",
        fields: hashmap!{},
        renderer: Some(|_c| {
            html! {
                <span class="keyword">{ "pub(self)" }</span>
            }
        }),
        ..Default::default()
    },

    "5996fcf2-2277-40c8-8081-db9d5ca12be8" => GO_FRAGMENT @ Kind {
        name: "go_fragment",
        fields: hashmap!{
            0 => Field {
                name: "items",
                types: &[GO_FUNCTION],
                repeated: true,
                ..Default::default()
            },
        },
        ..Default::default()
    },
    "a67915dd-68e8-4028-b165-ecb9abd9ea76" => GO_STRING_LITERAL @ Kind {
        name: "go_string_literal",
        fields: hashmap!{
            0 => Field {
                name: "value",
                raw: true,
                ..Default::default()
            },
        },
        renderer: Some(|c| {
            html! {
                <span>
                    { "\"" }{ c.view_child(0) }{ "\"" }
                </span>
            }
        }),
        ..Default::default()
    },
    "ed98e2e6-6422-4737-a70f-22a1935b007f" => GO_FUNCTION @ Kind {
        name: "go_function",
        fields: hashmap!{
            0 => Field {
                name: "identifier",
                raw: true,
                ..Default::default()
            },
            1 => Field {
                name: "arguments",
                ..Default::default()
            },
            2 => Field {
                name: "return_type",
                ..Default::default()
            },
            3 => Field {
                name: "body",
                types: GO_STATEMENT,
                repeated: true,
                ..Default::default()
            },
        },
        renderer: Some(|c| {
            let arguments = c.view_children(1).into_iter().intersperse(html!{
                <span>{ "," }</span>
            });
            let body = c.view_children(3).into_iter().map(|v| {
                html! {
                    <div class="indent">{ v }</div>
                }
            });
            html! {
                <div>
                    <span class="keyword">{ "func" }</span>
                    { c.view_child(0) }
                    { "(" }{ for arguments }{ ")" }
                    { c.view_child(2) }
                    { "{" }
                    { for body }
                    { "}" }
                </div>
            }
        }),
        ..Default::default()
    },
    "1c91fa1d-2c46-459e-8dcd-897bf931df25" => GO_ASSIGNMENT @ Kind {
        name: "go_assignment",
        fields: hashmap!{
            0 => Field {
                name: "left",
                ..Default::default()
            },
            1 => Field {
                name: "right",
                ..Default::default()
            },
        },
        renderer: Some(|c| {
            html! {
                <div>
                    { c.view_child(0) }{ ":=" }{ c.view_child(1) }
                </div>
            }
        }),
        ..Default::default()
    },
    "c3befc7b-f618-4567-b629-863910646c3d" => GO_IF @ Kind {
        name: "go_if",
        fields: hashmap!{
            0 => Field {
                name: "condition",
                types: GO_EXPRESSION,
                repeated: true,
                ..Default::default()
            },
            1 => Field {
                name: "true_body",
                types: GO_EXPRESSION,
                repeated: true,
                ..Default::default()
            },
            2 => Field {
                name: "false_body",
                types: GO_EXPRESSION,
                repeated: true,
                ..Default::default()
            },
        },
        renderer: Some(|c| {
            let conditions = c.view_children(0).into_iter().intersperse(comma());
            let true_body = c.view_children(1).into_iter().map(|v| html!{
                <div class="indent">{ v }</div>
            });
            let false_body = c.view_children(2).into_iter().map(|v| html!{
                <div class="indent">{ v }</div>
            });
            html! {
                <div>
                    <span class="keyword">{ "if" }</span>
                    { for conditions }
                    { "{" }
                    { for true_body }
                    { "}" }<span class="keyword">{ "else" }</span>{ "{" }
                    { for false_body }
                    { "}" }
                </div>
            }
        }),
        ..Default::default()
    },
    "122c3140-5104-4977-9ca9-dcca25b27394" => GO_FUNCTION_CALL @ Kind {
        name: "go_function_call",
        fields: hashmap!{
            0 => Field {
                name: "function",
                types: GO_EXPRESSION,
                ..Default::default()
            },
            1 => Field {
                name: "arguments",
                types: GO_EXPRESSION,
                repeated: true,
                ..Default::default()
            },
        },
        renderer: Some(|c| {
            let arguments = c.view_children(1).into_iter().intersperse(comma());
            html! {
                <div>
                    { c.view_child(0) }{ "(" }{ for arguments }{ ")" }
                </div>
            }
        }),
        ..Default::default()
    },
    "4ca0de48-75df-42e1-823b-33e70d445a5a" => GO_VARIABLE_DECLARATION @ Kind {
        name: "go_variable_declaration",
        fields: hashmap!{
            0 => Field {
                name: "identifier",
                ..Default::default()
            },
            1 => Field {
                name: "type",
                ..Default::default()
            },
            2 => Field {
                name: "value",
                ..Default::default()
            },
        },
        renderer: Some(|c| {
            html! {
                <div>
                    <span class="keyword">{ "var" }</span>{ c.view_child(0) }{ c.view_child(1) }{ "=" }{ c.view_child(2) }
                </div>
            }
        }),
        ..Default::default()
    },

    "72e31cba-ff86-4311-95f7-fe4d418c1bd3" => MARKDOWN_FRAGMENT @ Kind {
        name: "markdown_fragment",
        fields: hashmap!{
            0 => Field {
                name: "items",
                types: RUST_TYPE,
                ..Default::default()
            },
        },
        ..Default::default()
    },
    "fe5c5d00-7d24-428a-8bd4-28ccd605e7d4" => MARKDOWN_HEADING @ Kind {
        name: "markdown_heading",
        fields: hashmap!{
            0 => Field {
                name: "level",
                raw: true,
                ..Default::default()
            },
            1 => Field {
                name: "text",
                raw: true,
                ..Default::default()
            },
        },
        ..Default::default()
    },
    "b9a858e7-f511-4de4-bdf7-58d95073d03e" => MARKDOWN_CODE_BLOCK @ Kind {
        name: "markdown_code_block",
        fields: hashmap!{
            0 => Field {
                name: "lang",
                raw: true,
                ..Default::default()
            },
            1 => Field {
                name: "code",
                raw: true,
                ..Default::default()
            },
        },
        ..Default::default()
    },
    "e2a79822-f044-418a-998a-9bee7c5965bc" => RUST_CARGO_TOML @ Kind {
        name: "rust_cargo_toml",
        fields: hashmap!{
            0 => Field {
                name: "package",
                types: &[RUST_CARGO_TOML_PACKAGE],
                ..Default::default()
            },
            1 => Field {
                name: "dependencies",
                types: &[RUST_CARGO_TOML_DEP],
                ..Default::default()
            },
        },
        ..Default::default()
    },
    "26cd8d0d-512a-4734-a902-6e3cf65e92e4" => RUST_CARGO_TOML_PACKAGE @ Kind {
        name: "rust_cargo_toml_package",
        fields: hashmap!{
            0 => Field {
                name: "name",
                raw: true,
                ..Default::default()
            },
            1 => Field {
                name: "version",
                raw: true,
                ..Default::default()
            },
            2 => Field {
                name: "authors",
                raw: true,
                ..Default::default()
            },
            3 => Field {
                name: "edition",
                raw: true,
                ..Default::default()
            },
        },
        ..Default::default()
    },
    "e7b77123-bcfd-40b5-8ca9-bf96c7f34aad" => RUST_CARGO_TOML_DEP @ Kind {
        name: "rust_cargo_toml_dep",
        fields: hashmap!{
            0 => Field {
                name: "name",
                raw: true,
                ..Default::default()
            },
            1 => Field {
                name: "version",
                raw: true,
                ..Default::default()
            },
            2 => Field {
                name: "features",
                raw: true,
                ..Default::default()
            },
        },
        ..Default::default()
    },
}

pub fn create_node(kind_id: &str) -> Node {
    SCHEMA
        .get_kind(kind_id)
        .map(|kind| Node {
            kind: kind_id.to_string(),
            value: "".to_string(),
            links: BTreeMap::new(),
        })
        .unwrap_or_else(|| panic!("Unknown kind: {}", kind_id))
}

pub struct ValidatorContext {
    pub global_state: Rc<GlobalState>,
    pub selected_path: Vec<Selector>,
    pub cursor: Cursor,
    pub onselect: Callback<Vec<Selector>>,
    pub updatemodel: Callback<Msg>,
}

impl ValidatorContext {
    pub fn node(&self) -> Option<&Node> {
        self.cursor.node(&self.global_state.node_store)
    }

    pub fn view_child(&self, field_id: usize) -> Html {
        self.view_child_index(field_id, 0, true).unwrap_or_default()
    }
    pub fn view_child_with_placeholder(&self, field_id: usize) -> Html {
        self.view_child_index(field_id, 0, true).unwrap_or_default()
    }
    fn view_child_index(&self, field_id: usize, index: usize, placeholder: bool) -> Option<Html> {
        log::debug!("view_child: {:?}", field_id);
        log::debug!("cursor: {:?}", self.cursor);
        let node = &self.node().unwrap();
        let hash = node
            .links
            .get(&field_id)
            .and_then(|fields| fields.get(index))
            .cloned();
        if hash.is_none() && !placeholder {
            return None;
        }
        let child_cursor = self
            .cursor
            .traverse(
                &self.global_state.node_store,
                &[Selector { field_id, index }],
            )
            .unwrap();
        let kind = SCHEMA.get_kind(&node.kind);
        let field = kind.and_then(|k| k.get_field(field_id));
        let allowed_kinds = field.map(|v| v.types).unwrap_or_default();
        Some(html! {
            <NodeComponent
                global_state={ self.global_state.clone() }
                cursor={ child_cursor }
                selected_path={ self.selected_path.clone() }
                onselect={ self.onselect.clone() }
                updatemodel={ self.updatemodel.clone() }
                allowed_kinds={ allowed_kinds }
            />
        })
    }
    pub fn view_children(&self, field_id: usize) -> Vec<Html> {
        log::debug!("view_child: {:?}", field_id);
        let node = &self.node().unwrap();
        if node.links.get(&field_id).is_none() {
            return vec![];
        }
        if node.links.get(&field_id).unwrap().is_empty() {
            return vec![];
        }
        node.links
            .get(&field_id)
            .unwrap()
            .iter()
            .enumerate()
            // TODO: placeholder for invalid ones?
            .filter_map(|(i, _h)| self.view_child_index(field_id, i, true))
            .collect()
    }
    // TODO: field / child.
}

// Generate valid values.
type Renderer = fn(&ValidatorContext) -> Html;

pub fn default_renderer(c: &ValidatorContext) -> Html {
    let cursor = &c.cursor;
    let node = c.node().unwrap();
    let path = cursor.path();
    log::debug!("default_renderer: {:?}", path);
    let kind = SCHEMA.get_kind(&node.kind);
    let _hash = "xxx";
    let header = html! {
        <div>
            <div class={ KIND_CLASSES.join(" ") }>
                { kind.map(|k| k.name).unwrap_or_default() }
            </div>
            // <div class="inline-block text-xs border border-black">
            //     { hash.clone() }
            // </div>
        </div>
    };
    // Node.
    // https://codepen.io/xotonic/pen/JRLAOR
    let children: Vec<_> = node
        .links
        .iter()
        .flat_map(|(field_id, hashes)| {
            let field = kind.and_then(|k| k.get_field(*field_id));
            let field_name = field.map(|f| f.name).unwrap_or("INVALID");
            // let _validators = field_schema.map(|v| v.validators).unwrap_or_default();
            let path = path.clone();
            hashes.iter().enumerate().map(move |(i, _h)| {
                let selector = Selector {
                    field_id: *field_id,
                    index: i,
                };
                let child_path = append(&path, selector.clone());
                let updatemodel = c.updatemodel.clone();
                let onclick = Callback::from(move |e: MouseEvent| {
                    e.stop_propagation();
                    updatemodel.emit(Msg::Select(child_path.clone()))
                });
                // TODO: Sticky field headers.
                html! {
                    <div class="pl-3 flex items-start">
                        <div onclick={ onclick } >
                            { display_selector_text(field_name, selector.index) }
                        </div>
                        <div class="">
                            { ":" }
                        </div>
                        { c.view_child_index(*field_id, i, true).unwrap_or_default() }
                    </div>
                }
            })
        })
        .collect();
    html! {
        // <div class="divide-y divide-black border-t border-b border-black border-solid">
        <>
            { header }
            <div class="space-y-1 my-1">
                { for children }
            </div>
        </>
    }
}
