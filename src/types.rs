use crate::{model::Msg, node::FIELD_CLASSES};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::{
    collections::{BTreeMap, HashMap},
    convert::TryInto,
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
    pub field: String,
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
pub struct File {
    pub nodes: HashMap<Hash, Node>,
    pub root: Hash,
    pub log: Vec<(Ref, Node)>,
}

impl PartialEq for File {
    fn eq(&self, other: &Self) -> bool {
        // Only compare the size of the hashmap, since it is effectively append-only.
        self.nodes.len() == other.nodes.len()
            && self.root == other.root
            && self.log.len() == other.log.len()
    }
}

impl File {
    pub fn lookup(&self, path: &[Selector]) -> Option<&Node> {
        self.lookup_from(&self.root, path)
    }

    fn lookup_from(&self, base: &Hash, path: &[Selector]) -> Option<&Node> {
        let base = self.nodes.get(base)?;
        if path.is_empty() {
            Some(base)
        } else {
            let (selector, rest) = path.split_first().unwrap();
            let children = base.links.get(&selector.field)?;
            let child = children.get(selector.index)?;
            self.lookup_from(child, rest)
        }
    }

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
    pub fn replace_node(&mut self, path: &[Selector], node: Node) -> Option<Hash> {
        self.replace_node_from(&self.root.clone(), path, node)
    }

    #[must_use]
    fn replace_node_from(&mut self, base: &Hash, path: &[Selector], node: Node) -> Option<Hash> {
        if path.is_empty() {
            Some(self.add_node(&node))
        } else {
            let mut base = self.nodes.get(base)?.clone();
            let selector = path[0].clone();
            match base
                .links
                .get(&selector.field)
                .and_then(|v| v.get(selector.index))
            {
                Some(old_child_hash) => {
                    let new_child_hash =
                        self.replace_node_from(old_child_hash, &path[1..], node)?;
                    base.links.get_mut(&selector.field)?[selector.index] = new_child_hash;
                }
                None => {
                    // WARN: Only works for one level of children.
                    let new_child_hash = self.add_node(&node);
                    base.links
                        .entry(selector.field)
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
    pub kind: String,
    pub value: String,
    pub links: BTreeMap<String, Vec<Hash>>,
}

pub fn display_selector(selector: &Selector) -> Html {
    html! {
        <div class={ FIELD_CLASSES.join(" ") }>
          <span class="border-r border-black pr-1">{ selector.field.clone() }</span>
          <span class="pl-1">{ format!("{}", selector.index) }</span>
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
