use crate::{model::Msg, node::FIELD_CLASSES};
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use sha2::{Digest, Sha256};
use std::{
    collections::{hash_map::Entry, BTreeMap, HashMap},
    convert::TryInto,
    ops::Deref,
    rc::Rc,
    sync::{Arc, Mutex},
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

#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
pub enum Mode {
    Normal,
    Edit,
}

pub fn hash(value: &[u8]) -> Hash {
    let bytes: [u8; 32] = Sha256::digest(value).try_into().unwrap();
    "sha256:".to_string() + &hex::encode(bytes)
}

pub fn serialize_node(node: &Node) -> Vec<u8> {
    serde_json::to_string_pretty(node)
        .unwrap()
        .as_bytes()
        .to_vec()
}

pub fn deserialize_node(raw: &[u8]) -> Option<Node> {
    serde_json::from_slice(raw).ok()
}

pub fn hash_node(node: &Node) -> Hash {
    let node_bytes = serialize_node(node);
    hash(&node_bytes)
}

#[derive(Default, PartialEq, Clone)]
pub struct NodeState {
    // TODO: Errors.
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct NodeStore {
    raw_nodes: HashMap<Hash, Vec<u8>>,
    parsed_nodes: Arc<Mutex<HashMap<Hash, Node>>>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Cursor {
    pub parent: Option<(Box<Cursor>, Selector)>,
    pub link: Link,
}

impl Cursor {
    pub fn next(&self, node_store: &NodeStore) -> Option<Cursor> {
        // This must work even if the current hash / reference is invalid.
        self.link
            .get(node_store)
            .and_then(|link_target| {
                match link_target {
                    LinkTarget::Raw(_) => None,
                    LinkTarget::Parsed(node) => {
                        node.links.iter().next().map(|(field_id, links)| {
                            // Depth first.
                            Cursor {
                                parent: Some((
                                    Box::new(self.clone()),
                                    Selector {
                                        field_id: *field_id,
                                        index: 0,
                                    },
                                )),
                                link: links[0].clone(),
                            }
                        })
                    }
                }
            })
            .or_else(||
                // Try one level up.
                self.parent
                .as_ref()
                    .and_then(|(parent, selector)| parent.child_after(node_store, &selector)))
    }
    fn child_after(&self, node_store: &NodeStore, selector: &Selector) -> Option<Cursor> {
        self.link
            .get(node_store)
            .and_then(|node| node.as_parsed().cloned())
            .and_then(|node| {
                let children = node.links.get(&selector.field_id).unwrap();
                if selector.index < (children.len() - 1) {
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
        // TODO: not working
        self.parent
            .as_ref()
            .and_then(|(parent, selector)| parent.child_before(node_store, &selector))
            .or_else(|| {
                self.parent
                    .as_ref()
                    .map(|(parent, _selector)| (**parent).clone())
            })
    }
    fn child_before(&self, node_store: &NodeStore, selector: &Selector) -> Option<Cursor> {
        self.link.get(node_store).and_then(|node| {
            if selector.index >= (0 + 1) {
                //  Prev index.
                let prev_selector = Selector {
                    field_id: selector.field_id,
                    index: selector.index - 1,
                };
                self.traverse(node_store, &[prev_selector])
            } else if let Some((prev_field_id, _prev_children)) = node
                .as_parsed()
                .map(|node| node.links.clone())
                .unwrap_or_default()
                .range((
                    std::ops::Bound::Unbounded,
                    std::ops::Bound::Excluded(selector.field_id),
                ))
                .next_back()
            {
                // Next field.
                let prev_selector = Selector {
                    field_id: *prev_field_id,
                    index: 0,
                };
                self.traverse(node_store, &[prev_selector])
            } else {
                None
            }
        })
    }
    pub fn parent(&self, _node_store: &NodeStore) -> Option<Cursor> {
        self.parent
            .as_ref()
            .map(|(parent_cursor, _selector)| (**parent_cursor).clone())
    }
    pub fn traverse(&self, node_store: &NodeStore, path: &[Selector]) -> Option<Cursor> {
        match path.split_first() {
            Some((selector, rest)) => {
                match self.link.get(node_store)? {
                    LinkTarget::Raw(_) => None,
                    LinkTarget::Parsed(node) => {
                        // child_hash may or may not be valid at this point.
                        let child_link = node.links.get(&selector.field_id)?.get(selector.index)?;
                        let child = Cursor {
                            parent: Some((Box::new(self.clone()), selector.clone())),
                            link: child_link.clone(),
                        };
                        child.traverse(node_store, rest)
                    }
                }
            }
            None => Some(self.clone()),
        }
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
        self.raw_nodes.len() == other.raw_nodes.len()
    }
}

impl NodeStore {
    // TODO: remove.
    pub fn get_raw(&self, hash: &Hash) -> Option<&Vec<u8>> {
        self.raw_nodes.get(hash)
    }

    pub fn get_parsed(&self, hash: &Hash) -> Option<Node> {
        let mut n = self.parsed_nodes.lock().unwrap();
        let entry = n.entry(hash.clone());
        match entry {
            Entry::Occupied(o) => Some(o.get().clone()),
            Entry::Vacant(v) => {
                let raw_node = self.raw_nodes.get(hash)?;
                // TODO: Put into cache.
                let node = crate::types::deserialize_node(raw_node)?;
                let r = v.insert(node.clone());
                Some(node)
            }
        }
    }

    pub fn has_raw_node(&self, hash: &Hash) -> bool {
        self.raw_nodes.contains_key(hash)
    }

    pub fn iter(&self) -> impl Iterator<Item = (&Hash, &Vec<u8>)> {
        self.raw_nodes.iter()
    }

    pub fn len(&self) -> usize {
        self.raw_nodes.len()
    }

    #[must_use]
    pub fn put_parsed(&mut self, node: &Node) -> Hash {
        let h = hash_node(node);
        self.parsed_nodes
            .lock()
            .unwrap()
            .insert(h.clone(), node.clone());
        self.raw_nodes
            .insert(h.clone(), crate::types::serialize_node(node));
        h
    }

    #[must_use]
    pub fn put_raw(&mut self, value: &[u8]) -> Hash {
        let h = hash(value);
        self.raw_nodes.insert(h.clone(), value.to_vec());
        h
    }

    pub fn put_many(&mut self, nodes: &[Node]) {
        for node in nodes {
            self.put_parsed(node);
        }
    }

    pub fn put_many_raw(&mut self, nodes: &[Vec<u8>]) {
        for node in nodes {
            self.put_raw(node);
        }
    }
}

// TODO: Navigate to children directly, but use :var to navigate to variables, otherwise skip them
// when navigating.
#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
pub struct Node {
    // UUID.
    pub kind: String,
    // Keyed by field id.
    pub links: BTreeMap<usize, Vec<Link>>,
}

impl Node {
    pub fn get_link(&self, selector: &Selector) -> Option<&Link> {
        self.links
            .get(&selector.field_id)
            .and_then(|links| links.get(selector.index))
    }
    pub fn get_link_mut(&mut self, selector: &Selector) -> Option<&mut Link> {
        self.links
            .get_mut(&selector.field_id)
            .and_then(|links| links.get_mut(selector.index))
    }
}

#[derive(Serialize_repr, Deserialize_repr, Debug, Clone, PartialEq)]
#[repr(u8)]
pub enum LinkType {
    Raw = 0,
    Parsed = 1,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Link {
    // 0: raw
    // 1: parsed
    #[serde(rename = "type")]
    pub type_: LinkType,
    pub hash: Hash,
}

impl Link {
    pub fn get<'a>(&self, node_store: &'a NodeStore) -> Option<LinkTarget<'a>> {
        match self.type_ {
            LinkType::Raw => node_store.get_raw(&self.hash).map(LinkTarget::Raw),
            LinkType::Parsed => node_store.get_parsed(&self.hash).map(LinkTarget::Parsed),
        }
    }
}

#[derive(Debug, Clone)]
pub enum LinkTarget<'a> {
    Raw(&'a Vec<u8>),
    Parsed(Node),
}

impl<'a> LinkTarget<'a> {
    pub fn as_parsed(&self) -> Option<&Node> {
        match self {
            LinkTarget::Parsed(node) => Some(node),
            _ => None,
        }
    }
}

pub fn display_selector(selector: &Selector) -> Html {
    display_selector_text(&format!("{}", selector.field_id), selector.index)
}

pub fn display_selector_text(field_name: &str, index: usize) -> Html {
    // TODO: lookup.
    html! {
        <div class={ FIELD_CLASSES.join(" ") }>
          <span class="border-r border-black pr-1">{ field_name }</span>
          <span class="pl-1">{ format!("[{}]", index) }</span>
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
