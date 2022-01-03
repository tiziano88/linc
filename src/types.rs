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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NodeStore {
    pub nodes: HashMap<Hash, Node>,
    pub root: Hash,
    pub log: Vec<(Ref, Node)>,
}

#[derive(Clone, Debug)]
pub struct Cursor {
    pub path: Path,
    // In same order as path.
    pub parents: Vec<Ref>,
    pub hash: Hash,
}

impl Cursor {
    pub fn next(&self, node_store: &NodeStore) -> Option<Cursor> {
        todo!()
    }
    pub fn prev(&self, node_store: &NodeStore) -> Option<Cursor> {
        todo!()
    }
    pub fn parent(&self, _node_store: &NodeStore) -> Option<Cursor> {
        self.parents
            .split_last()
            .map(|(parent_hash, parent_parents)| Cursor {
                path: self.path[..self.path.len() - 1].to_vec(),
                parents: parent_parents.to_vec(),
                hash: parent_hash.clone(),
            })
    }
    pub fn traverse(&self, node_store: &NodeStore, path: &[Selector]) -> Option<Cursor> {
        path.first().and_then(|selector| {
            // child_hash may or may not be valid at this point.
            let child_hash = self
                .node(node_store)?
                .links
                .get(&selector.field_id)?
                .get(selector.index)?;
            let mut parents = self.parents.clone();
            parents.push(self.hash.clone());
            Some(Cursor {
                path: append(&self.path, selector.clone()),
                parents,
                hash: child_hash.clone(),
            })
        })
    }
    pub fn node<'a>(&self, node_store: &'a NodeStore) -> Option<&'a Node> {
        node_store.lookup_hash(&self.hash)
    }
}

impl PartialEq for NodeStore {
    fn eq(&self, other: &Self) -> bool {
        // Only compare the size of the hashmap, since it is effectively append-only.
        self.nodes.len() == other.nodes.len()
            && self.root == other.root
            && self.log.len() == other.log.len()
    }
}

impl NodeStore {
    pub fn root(&self) -> Cursor {
        Cursor {
            path: vec![],
            parents: vec![],
            hash: self.root.clone(),
        }
    }

    pub fn path(&self, path: &[Selector]) -> Option<Cursor> {
        self.root().traverse(self, path)
    }

    // TODO: remove.
    pub fn lookup_hash(&self, hash: &Hash) -> Option<&Node> {
        self.nodes.get(hash)
    }

    #[must_use]
    pub fn add_node(&mut self, node: &Node) -> Hash {
        let h = hash_node(node);
        self.nodes.insert(h.clone(), node.clone());
        h
    }

    #[must_use]
    pub fn replace_node(&mut self, path: &[Selector], node: &Node) -> Option<Hash> {
        self.replace_node_from(&self.root.clone(), path, node)
    }

    #[must_use]
    fn replace_node_from(&mut self, base: &Hash, path: &[Selector], node: &Node) -> Option<Hash> {
        if path.is_empty() {
            Some(self.add_node(node))
        } else {
            let mut base = self.nodes.get(base)?.clone();
            let selector = path[0].clone();
            match base
                .links
                .get(&selector.field_id)
                .and_then(|v| v.get(selector.index))
            {
                Some(old_child_hash) => {
                    let new_child_hash =
                        self.replace_node_from(old_child_hash, &path[1..], node)?;
                    base.links.get_mut(&selector.field_id)?[selector.index] = new_child_hash;
                }
                None => {
                    // WARN: Only works for one level of children.
                    let new_child_hash = self.add_node(node);
                    base.links
                        .entry(selector.field_id)
                        .or_default()
                        .push(new_child_hash);
                }
            };
            Some(self.add_node(&base))
        }
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
