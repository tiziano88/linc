use crate::{
    model::{GlobalState, Model, Msg},
    node::{NodeComponent, KIND_CLASSES},
    types::{append, display_selector_text, Cursor, LinkTarget, Node, Path, Selector},
};
use maplit::hashmap;
use serde::{Deserialize, Serialize};
use std::{
    collections::{BTreeMap, HashMap},
    rc::Rc,
};
use yew::prelude::*;

type UUID = String;

// Schema of the schema.
#[derive(PartialEq, Clone, Serialize, Deserialize, Default)]
pub struct Schema {
    pub kinds: Vec<Kind>,
}

impl Schema {
    pub fn get_kind(&self, kind_id: u64) -> Option<&Kind> {
        self.kinds.iter().find(|k| k.kind_id == kind_id)
    }

    pub fn root_kind(&self) -> Option<&Kind> {
        self.kinds.get(0)
    }
}

#[derive(PartialEq, Clone, Serialize, Deserialize, Default, Debug)]
pub struct Kind {
    pub kind_id: u64,
    pub name: String,
    pub fields: Vec<Field>,
}

impl Kind {
    pub fn get_field(&self, field_id: u64) -> Option<&Field> {
        self.fields.iter().find(|f| f.field_id == field_id)
    }
}

#[derive(PartialEq, Clone, Serialize, Deserialize, Default, Debug)]
pub struct Field {
    pub field_id: u64,
    pub name: String,
    // pub kind_id: u64,
    pub type_: FieldType,
    // TODO: Use type id.
    pub raw: u64,
}

#[derive(PartialEq, Clone, Serialize, Deserialize, Debug)]
pub enum FieldType {
    String,
    Bytes,
    Bool,
    Int,
    Float,
    Object { kind_id: u64 },
}

impl Default for FieldType {
    fn default() -> Self {
        FieldType::String
    }
}

pub enum FieldValue {
    String(String),
    Bytes(Vec<u8>),
    Bool(bool),
    Int(i64),
    Float(f64),
    Object(Object),
}

pub struct Object {
    pub kind_id: u64,
    pub fields: Vec<(u64, FieldValue)>,
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

pub struct ValidatorContext {
    pub global_state: Rc<GlobalState>,
    pub selected_path: Vec<Selector>,
    pub cursor: Cursor,
    pub onselect: Callback<Vec<Selector>>,
    pub updatemodel: Callback<Msg>,
}

impl ValidatorContext {
    pub fn node<'a>(&'a self) -> Option<LinkTarget<'a>> {
        self.cursor.link.get(&self.global_state.node_store)
    }

    pub fn view_child(&self, field_id: u64) -> Html {
        self.view_child_index(field_id, 0, true).unwrap_or_default()
    }
    pub fn view_child_with_placeholder(&self, field_id: u64) -> Html {
        self.view_child_index(field_id, 0, true).unwrap_or_default()
    }
    fn view_child_index(&self, field_id: u64, index: usize, placeholder: bool) -> Option<Html> {
        log::debug!("view_child: {:?}", field_id);
        log::debug!("cursor: {:?}", self.cursor);
        let kind_id = self.cursor.kind_id;
        let link_target = self.node()?;
        let node = link_target.as_parsed()?;
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
                &self.global_state.schema,
                &[Selector { field_id, index }],
            )
            .unwrap();
        let kind = self.global_state.schema.get_kind(kind_id);
        let field = kind.and_then(|k| k.get_field(field_id));
        Some(html! {
            <NodeComponent
                global_state={ self.global_state.clone() }
                cursor={ child_cursor }
                selected_path={ self.selected_path.clone() }
                onselect={ self.onselect.clone() }
                updatemodel={ self.updatemodel.clone() }
            />
        })
    }
    pub fn view_children(&self, field_id: u64) -> Vec<Html> {
        log::debug!("view_child: {:?}", field_id);
        match self.node() {
            None => Vec::new(),
            Some(LinkTarget::Raw(_)) => Vec::new(),
            Some(LinkTarget::Parsed(node)) => {
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
        }
    }
    // TODO: field / child.
}

// Generate valid values.
type Renderer = fn(&ValidatorContext) -> Html;

pub fn default_renderer(c: &ValidatorContext) -> Html {
    let cursor = &c.cursor;
    let kind_id = cursor.kind_id;
    let kind = c
        .global_state
        .schema
        .get_kind(kind_id)
        .cloned()
        .unwrap_or_default();
    let path = cursor.path();
    log::debug!("default_renderer: {:?}", path);
    match c.node().unwrap() {
        LinkTarget::Raw(value) => html! {
            <div>
                <span>{ "Raw" }</span>
            </div>
        },
        LinkTarget::Parsed(node) => {
            let header = html! {
                <div>
                    <div class={ KIND_CLASSES.join(" ") }>
                        { kind.name.clone() }
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
                    let field = kind.get_field(*field_id);
                    let field_name = field
                        .map(|f| f.name.clone())
                        .unwrap_or("INVALID".to_string());
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
                                    { display_selector_text(&field_name, selector.index) }
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
    }
}
