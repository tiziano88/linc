use crate::{
    model::{Model, Msg},
    node::{NodeComponent, KIND_CLASSES},
    types::{append, display_selector_text, Node, Path, Selector},
};
use maplit::hashmap;
use std::{collections::HashMap, rc::Rc};
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

pub static SCHEMA: std::lazy::SyncLazy<Schema> = std::lazy::SyncLazy::new(create_schema);

static RUST_TYPE: &[&'static str] = &[RUST_ARRAY_TYPE];
static RUST_VISIBILITY: &[&'static str] = &[
    RUST_VISIBILITY_PUB,
    RUST_VISIBILITY_PUB_CRATE,
    RUST_VISIBILITY_PUB_SELF,
];
static RUST_EXPRESSION: &[&'static str] = &[RUST_IF, RUST_MATCH, RUST_STRING_LITERAL];
static GO_STATEMENT: &[&'static str] = &[GO_ASSIGNMENT, GO_VARIABLE_DECLARATION];

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
            let (items_head, items) = c.view_children(0);
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
            let (_items_head, items) = c.view_children(3);
            let items = items.into_iter().map(|v| {
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
                ..Default::default()
            },
            2 => Field {
                name: "false_body",
                types: RUST_EXPRESSION,
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
            let (match_arms_head, match_arms) = c.view_children(1);
            let match_arms = match_arms.into_iter().map(|v| {
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
                    { match_arms_head }
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
            let (patterns_head, patterns) = c.view_children(0);
            let patterns = patterns.into_iter().intersperse(html! {
                    <span>{ "|" }</span>
            });
            html! {
                <span>
                    <span>{ for patterns }{ patterns_head }</span>
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
            let (_, arguments) = c.view_children(1);
            let arguments = arguments.into_iter().intersperse(html!{
                <span>{ "," }</span>
            });
            let (_, body) = c.view_children(3);
            let body = body.into_iter().map(|v| {
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
}

pub struct ValidatorContext {
    pub model: Rc<Model>,
    pub path: Vec<Selector>,
    pub node: Node,
    pub onselect: Callback<Vec<Selector>>,
    pub updatemodel: Callback<Msg>,
}

impl ValidatorContext {
    pub fn view_child(&self, field_id: usize) -> Html {
        self.view_child_index(field_id, 0)
    }
    pub fn view_child_index(&self, field_id: usize, index: usize) -> Html {
        log::debug!("view_child: {:?}", field_id);
        if self.node.links.get(&field_id).is_none() {
            return html! {};
        }
        if self.node.links.get(&field_id).unwrap().is_empty() {
            return html! {};
        }
        let h = &self.node.links.get(&field_id).unwrap()[index];
        let child_path = append(&self.path, Selector { field_id, index });
        let kind = SCHEMA.get_kind(&self.node.kind);
        let field = kind.and_then(|k| k.get_field(field_id));
        // TODO: validators.
        // let validators = field_schema.map(|v| v.validators).unwrap_or_default();
        let allowed_kinds = field.map(|v| v.types).unwrap_or_default();
        html! {
            // <div>
            //   { format!("{:?} {:?}", h, child_path) }
            // </div>
            <NodeComponent
                model={ self.model.clone() }
                hash={ h.clone() }
                onselect={ self.onselect.clone() }
                path={ child_path }
                updatemodel={ self.updatemodel.clone() }
                allowed_kinds={ allowed_kinds }
            />
        }
    }
    pub fn view_children(&self, field_id: usize) -> (Html, Vec<Html>) {
        log::debug!("view_child: {:?}", field_id);
        if self.node.links.get(&field_id).is_none() {
            return (html! {}, vec![]);
        }
        if self.node.links.get(&field_id).unwrap().is_empty() {
            return (html! {}, vec![]);
        }
        (
            html! {},
            self.node
                .links
                .get(&field_id)
                .unwrap()
                .iter()
                .enumerate()
                .map(|(i, _h)| self.view_child_index(field_id, i))
                .collect(),
        )
        // self.model
        //     .view_children(self.ctx, self.node, field_name, self.path)
    }
    // TODO: field / child.
}

// Generate valid values.
type Renderer = fn(&ValidatorContext) -> Html;

pub fn default_renderer(c: &ValidatorContext) -> Html {
    let node = &c.node;
    let path = &c.path;
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
                        { c.view_child_index(*field_id, i) }
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
