use crate::{model::Msg, node::FIELD_CLASSES};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::{
    collections::{BTreeMap, HashMap},
    convert::TryInto,
    ops::Deref,
    rc::Rc,
};
use wasm_bindgen::{JsCast, UnwrapThrowExt};
use web_sys::{HtmlInputElement, HtmlTextAreaElement, InputEvent};
use yew::{html, prelude::*, Html};

pub type Ref = String;

pub type Hash = String;

pub fn new_ref() -> Ref {
    uuid::Uuid::new_v4().to_hyphenated().to_string()
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, Serialize, Deserialize)]
pub struct Selector {
    pub field_id: usize,
    pub index: usize,
}

pub type Path = Vec<Selector>;

pub fn append(path: &[Selector], selector: Selector) -> Path {
    let mut new_path = path.to_vec();
    new_path.push(selector);
    new_path
}

pub fn parent(path: &[Selector]) -> &[Selector] {
    if path.is_empty() {
        path
    } else {
        path.split_last().unwrap().1
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Mode {
    Normal,
    Edit,
}

pub fn hash(value: &[u8]) -> Hash {
    let bytes: [u8; 32] = Sha256::digest(&value).try_into().unwrap();
    hex::encode(bytes)
}

pub fn hash_node(node: &Node) -> Hash {
    let node_json = serde_json::to_string_pretty(node).unwrap();
    let node_bytes = node_json.as_bytes();
    hash(node_bytes)
}

#[derive(Default, PartialEq, Clone)]
pub struct NodeState {
    // TODO: Errors.
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct NodeStore {
    nodes: HashMap<Hash, Node>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Cursor {
    pub parent: Option<(Box<Cursor>, Selector)>,
    pub hash: Hash,
}

impl Cursor {
    pub fn next(&self, node_store: &NodeStore) -> Option<Cursor> {
        // This must work even if the current hash / reference is invalid.
        self.node(node_store)
            .and_then(|node| {
                node.links.iter().next().map(|(field_id, children)| {
                    // Depth first.
                    Cursor {
                        parent: Some((
                            Box::new(self.clone()),
                            Selector {
                                field_id: *field_id,
                                index: 0,
                            },
                        )),
                        hash: children[0].clone(),
                    }
                })
            })
            .or_else(||
                // Try one level up.
                self.parent
                .as_ref()
                    .and_then(|(parent, selector)| parent.child_after(node_store, &selector)))
    }
    fn child_after(&self, node_store: &NodeStore, selector: &Selector) -> Option<Cursor> {
        self.node(node_store).and_then(|node| {
            let children = node.links.get(&selector.field_id).unwrap();
            if (selector.index + 1) < children.len() {
                //  Next index.
                let next_selector = Selector {
                    field_id: selector.field_id,
                    index: selector.index + 1,
                };
                self.traverse(node_store, &[next_selector])
            } else if let Some((next_field_id, _next_children)) = node
                .links
                .range((
                    std::ops::Bound::Excluded(selector.field_id),
                    std::ops::Bound::Unbounded,
                ))
                .next()
            {
                // Next field.
                let next_selector = Selector {
                    field_id: *next_field_id,
                    index: 0,
                };
                self.traverse(node_store, &[next_selector])
            } else {
                // Go up.
                self.parent
                    .as_ref()
                    .and_then(|(parent, selector)| parent.child_after(node_store, &selector))
            }
        })
    }
    pub fn prev(&self, node_store: &NodeStore) -> Option<Cursor> {
        todo!()
    }
    pub fn parent(&self, _node_store: &NodeStore) -> Option<Cursor> {
        self.parent
            .as_ref()
            .map(|(parent_cursor, _selector)| (**parent_cursor).clone())
    }
    pub fn traverse(&self, node_store: &NodeStore, path: &[Selector]) -> Option<Cursor> {
        match path.split_first() {
            Some((selector, rest)) => {
                // child_hash may or may not be valid at this point.
                let child_hash = self
                    .node(node_store)?
                    .links
                    .get(&selector.field_id)?
                    .get(selector.index)?;
                let child = Cursor {
                    parent: Some((Box::new(self.clone()), selector.clone())),
                    hash: child_hash.clone(),
                };
                child.traverse(node_store, rest)
            }
            None => Some(self.clone()),
        }
    }
    pub fn node<'a>(&self, node_store: &'a NodeStore) -> Option<&'a Node> {
        node_store.get(&self.hash)
    }
    pub fn path(&self) -> Vec<Selector> {
        match &self.parent {
            Some((parent_cursor, selector)) => {
                let mut path = parent_cursor.path();
                path.push(selector.clone());
                path
            }
            None => vec![],
        }
    }
}

impl PartialEq for NodeStore {
    fn eq(&self, other: &Self) -> bool {
        // Only compare the size of the hashmap, since it is effectively append-only.
        self.nodes.len() == other.nodes.len()
    }
}

impl NodeStore {
    // TODO: remove.
    pub fn get(&self, hash: &Hash) -> Option<&Node> {
        self.nodes.get(hash)
    }

    #[must_use]
    pub fn put(&mut self, node: &Node) -> Hash {
        let h = hash_node(node);
        self.nodes.insert(h.clone(), node.clone());
        h
    }
}

// TODO: Navigate to children directly, but use :var to navigate to variables, otherwise skip them
// when navigating.
#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
pub struct Node {
    // UUID.
    pub kind: String,
    pub value: String,
    // Keyed by field id.
    pub links: BTreeMap<usize, Vec<Hash>>,
}

pub fn display_selector(selector: &Selector) -> Html {
    display_selector_text(&format!("{}", selector.field_id), selector.index)
}

pub fn display_selector_text(field_name: &str, index: usize) -> Html {
    // TODO: lookup.
    html! {
        <div class={ FIELD_CLASSES.join(" ") }>
          <span class="border-r border-black pr-1">{ field_name }</span>
          <span class="pl-1">{ format!("{}", index) }</span>
        </div>
    }
}

pub fn display_cursor(cursor: &Path) -> Html {
    let segments = cursor
        .iter()
        .map(display_selector)
        .intersperse(html! { <span>{ ">" }</span>});
    html! {
        <div>{ for segments }</div>
    }
}

pub struct Action {
    pub image: Option<String>,
    pub text: String,
    pub msg: Msg,
}

pub fn get_value_from_input_event(e: InputEvent) -> String {
    let event: Event = e.dyn_into().unwrap_throw();
    let event_target = event.target().unwrap_throw();
    if let Ok(target) = event_target.clone().dyn_into::<HtmlInputElement>() {
        return target.value();
    }
    if let Ok(target) = event_target.dyn_into::<HtmlTextAreaElement>() {
        return target.value();
    }
    panic!("unexpected input event");
    // let target: HtmlTextareaElement = event_target.dyn_into().unwrap_throw();
    // web_sys::console::log_1(&target.value().into());
    // target.value()
}
