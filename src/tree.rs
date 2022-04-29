use crate::{
    command_line::{CommandLine, Entry},
    model::{GlobalState, Model, Msg},
    schema::{
        default_renderer, Field, Kind, Schema, ValidatorContext, RUST_FUNCTION_CALL, SCHEMA, *,
    },
    types::{
        hash_node, parent, Cursor, Hash, Link, LinkTarget, LinkType, Mode, Node, NodeStore, Path,
        Selector,
    },
};
use std::{collections::BTreeMap, rc::Rc};
use web_sys::HtmlInputElement;
use yew::prelude::*;

pub struct TreeComponent {
    pub selected_path: Path,
    // pub hover_path: Path,
}

#[derive(Properties, PartialEq, Clone)]
pub struct TreeProperties {
    pub global_state: Rc<GlobalState>,
    pub root: Hash,
    pub updatemodel: Callback<Msg>,
    pub updateroot: Callback<Hash>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum TreeMsg {
    Select(Path),
    Hover(Path),

    Prev,
    Next,
    Parent,

    AddItem,
    DeleteItem,

    ReplaceNode(Path, Node, bool),
    AddField(Path, usize),

    SetNodeValue(Path, Vec<u8>),
}

impl Component for TreeComponent {
    type Message = TreeMsg;
    type Properties = TreeProperties;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            selected_path: Vec::new(),
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        todo!()
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let props = ctx.props();
        let global_state = props.global_state.as_ref();
        let node_store = &global_state.node_store;
        match msg {
            TreeMsg::Select(path) => self.selected_path = path,
            TreeMsg::Hover(path) => {}
            // TODO: sibling vs inner
            TreeMsg::Prev => {
                let new_selected_path = path(node_store, cursor(props.root), &self.selected_path)
                    .unwrap()
                    .prev(node_store)
                    .unwrap();
                self.selected_path = new_selected_path.path();
            }
            // Preorder tree traversal.
            TreeMsg::Next => self.next(node_store),
            TreeMsg::Parent => self.parent(node_store),
            TreeMsg::AddItem => {
                let selected_path = self.selected_path.clone();
                log::info!("AddItem {:?}", selected_path);
                let new_ref = node_store.put_parsed(&Node {
                    kind: "invalid".to_string(),
                    links: BTreeMap::new(),
                });
                if selected_path.is_empty() {
                    props.updateroot.emit(new_ref);
                } else {
                    let (selector, parent_path) = selected_path.split_last().unwrap();
                    let mut parent = self
                        .path(&node_store, parent_path)
                        .unwrap()
                        .link
                        .get(&node_store)
                        .unwrap()
                        .as_parsed()
                        .unwrap()
                        .clone();
                    // If the field does not exist, create a default one.
                    let children = parent.links.entry(selector.field_id).or_default();
                    let new_index = selector.index + 1;
                    children.insert(
                        new_index,
                        Link {
                            // TODO: Or should this be raw?
                            type_: LinkType::Parsed,
                            hash: new_ref,
                        },
                    );
                    tree_state.replace_node(node_store, parent_path, &parent);
                    // Select newly created element.
                    tree_state.selected_path.last_mut().unwrap().index = new_index;
                    tree_state.next(node_store);
                }
            }
            TreeMsg::DeleteItem => {
                let node_store = self.global_state_mut().node_store_mut();
                let selected_path = tree_state.selected_path.clone();
                if selected_path.is_empty() {
                    let node = Node {
                        kind: crate::schema::ROOT.to_string(),
                        links: BTreeMap::new(),
                    };
                    tree_state.replace_node(node_store, &[], &node);
                } else {
                    let (selector, parent_path) = selected_path.split_last().unwrap();
                    let mut parent = tree_state
                        .path(node_store, parent_path)
                        .unwrap()
                        .link
                        .get(node_store)
                        .unwrap()
                        .as_parsed()
                        .unwrap()
                        .clone();
                    // If the field does not exist, create a default one.
                    let children = parent.links.entry(selector.field_id).or_default();
                    children.remove(selector.index);
                    tree_state.replace_node(node_store, parent_path, &parent);
                    // Select parent.
                    tree_state.selected_path =
                        tree_state.selected_path[..tree_state.selected_path.len() - 1].to_vec();
                }
            }
            TreeMsg::ReplaceNode(path, node, mv) => {
                let node_store = self.global_state_mut().node_store_mut();
                log::info!("replace node {:?} {:?}", path, node);
                tree_state.replace_node(node_store, &path, &node);
                if mv {
                    tree_state.next(node_store);
                } else {
                    tree_state.selected_path = path;
                }
                set_location_hash(&tree_state.root);
            }
            TreeMsg::AddField(path, field_id) => {
                let node_store = self.global_state_mut().node_store_mut();
                let mut node = tree_state
                    .path(&node_store, &path)
                    .unwrap()
                    .link
                    .get(&node_store)
                    .unwrap()
                    .as_parsed()
                    .unwrap()
                    .clone();
                node.links
                    .entry(field_id)
                    .or_insert_with(Vec::new)
                    .push(Link {
                        type_: LinkType::Parsed,
                        hash: "".into(),
                    });
                let n = node.links[&field_id].len();
                tree_state.replace_node(node_store, &path, &node);
                tree_state.selected_path = append(
                    &path,
                    Selector {
                        field_id,
                        index: n - 1,
                    },
                );
                set_location_hash(&tree_state.root);
            }
            TreeMsg::SetNodeValue(path, value) => {
                let node_store = self.global_state_mut().node_store_mut();
                tree_state.selected_path = path.clone();
                tree_state.set_node_value(node_store, &path, &value);
                set_location_hash(&tree_state.root);
            }
        }
        *self.tree_state_mut(tree_id) = tree_state;
    }
}

fn cursor(root: Hash) -> Cursor {
    Cursor {
        parent: None,
        link: Link {
            type_: LinkType::Parsed,
            hash: root.clone(),
        },
    }
}

pub fn path(node_store: &NodeStore, base: Cursor, path: &[Selector]) -> Option<Cursor> {
    base.traverse(node_store, path)
}

pub fn set_node_value(&mut self, node_store: &mut NodeStore, path: &[Selector], value: &[u8]) {
    let target_hash = node_store.put_raw(value);
    if let Some(root) = self.replace_node_from(
        node_store,
        &self.root.clone(),
        path,
        &Link {
            type_: LinkType::Raw,
            hash: target_hash,
        },
    ) {
        self.root = root.hash;
    }
}

pub fn replace_node(node_store: &NodeStore, base: &Hash, path: &[Selector], node: &Node) {
    let target_hash = node_store.put_parsed(node);
    if let Some(root) = replace_node_from(
        node_store,
        base,
        path,
        &Link {
            type_: LinkType::Parsed,
            hash: target_hash,
        },
    ) {
        self.root = root.hash;
    }
}

// Returns link and nodes to add.
#[must_use]
fn replace_node_from(
    node_store: &NodeStore,
    base: &Hash,
    path: &[Selector],
    link: &Link,
) -> Option<(Link, Vec<Vec<u8>>)> {
    if path.is_empty() {
        Some((link.clone(), vec![]))
    } else {
        let mut new_node = node_store.get_parsed(base)?.clone();
        let selector = path[0].clone();
        let mut new_nodes = match new_node.get_link_mut(&selector) {
            Some(mut old_child_link) => {
                let (new_child_link, new_nodes) =
                    replace_node_from(node_store, &old_child_link.hash, &path[1..], link)?;
                *old_child_link = new_child_link;
                new_nodes
            }
            None => {
                // WARN: Only works for one level of children.
                new_node
                    .links
                    .entry(selector.field_id)
                    .or_default()
                    .push(link.clone());
                vec![]
            }
        };
        new_nodes.push(crate::types::serialize_node(&new_node));
        let new_node_hash = hash_node(&new_node);
        Some((
            Link {
                type_: LinkType::Parsed,
                hash: new_node_hash,
            },
            new_nodes,
        ))
    }
}

/*
fn parent(&mut self, node_store: &NodeStore) {
    if let Some(current) = self.path(node_store, &self.selected_path) {
        log::debug!("current: {:?}", current);
        if let Some(parent) = current.parent(node_store) {
            self.selected_path = parent.path();
        }
    }
}

fn prev(&mut self, node_store: &NodeStore) {
    if let Some(cursor) = self.path(node_store, &self.selected_path) {
        if let Some(prev) = cursor.prev(node_store) {
            self.selected_path = prev.path();
        }
    }
}

fn next(&mut self, node_store: &NodeStore) {
    log::warn!("old selected_path: {:?}", self.selected_path);
    if let Some(cursor) = self.path(node_store, &self.selected_path) {
        if let Some(next) = cursor.next(node_store) {
            log::warn!("new selected_path: {:?}", next.path());
            self.selected_path = next.path();
        }
    }
}
*/
