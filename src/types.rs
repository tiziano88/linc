use crate::schema::{
    FieldValidator, KindValue, ParsedValue, ValidationError, ValidatorContext, SCHEMA,
};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::{collections::HashMap, convert::TryInto};
use wasm_bindgen::JsCast;
use yew::{
    html,
    prelude::*,
    services::{
        keyboard::{KeyListenerHandle, KeyboardService},
        storage::Area,
        StorageService,
    },
    web_sys::HtmlElement,
    Component, ComponentLink, Html, KeyboardEvent, ShouldRender,
};

pub type Ref = String;

pub type Hash = String;
// pub const EMPTY_HASH: Hash = "".to_string();
// pub type Value = Vec<u8>;

#[derive(Debug, PartialEq, Eq, Clone, Hash, Serialize, Deserialize)]
pub struct Link {
    root: Option<Hash>,
    path: Path,
}

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

pub struct Model {
    pub file: File,

    pub store: StorageService,
    pub key_listener: KeyListenerHandle,

    pub cursor: Path,
    pub hover: Path,
    pub mode: Mode,

    pub link: ComponentLink<Self>,

    pub node_state: HashMap<Path, NodeState>,

    pub parsed_commands: Vec<ParsedValue>,
    pub selected_command_index: usize,

    pub errors: Vec<ValidationError>,
}

#[derive(Default)]
pub struct NodeState {
    // TODO: Errors.
}

pub fn parent(path: &[Selector]) -> &[Selector] {
    if path.is_empty() {
        path
    } else {
        path.split_last().unwrap().1
    }
}

impl Model {
    fn parse_commands(&self, path: &Path) -> Vec<ParsedValue> {
        self.field(path)
            .map(|field| {
                field
                    .validators
                    .iter()
                    .filter_map(|validator| match validator {
                        FieldValidator::Kind(kind) => SCHEMA.get_kind(kind),
                        FieldValidator::Literal(_valid) => None,
                    })
                    .map(|kind| ParsedValue {
                        kind_hierarchy: vec![kind.name.to_string()],
                        label: match kind.value {
                            KindValue::Struct { constructors, .. } => constructors[0].to_string(),
                            _ => kind.name.to_string(),
                        },
                        value: "".to_string(),
                    })
                    .collect::<Vec<_>>()
                // TODO: Ranking.
            })
            .unwrap_or_default()
    }

    fn prev(&mut self) {
        let flattened_paths = self.flatten_paths(&[]);
        log::info!("paths: {:?}", flattened_paths);
        let current_path_index = flattened_paths.iter().position(|x| *x == self.cursor);
        log::info!("current: {:?}", current_path_index);
        if let Some(current_path_index) = current_path_index {
            if current_path_index > 0 {
                if let Some(path) = flattened_paths.get(current_path_index - 1) {
                    self.cursor = path.clone();
                }
            }
        }
    }

    fn next(&mut self) {
        let flattened_paths = self.flatten_paths(&[]);
        log::info!("paths: {:?}", flattened_paths);
        let current_path_index = flattened_paths.iter().position(|x| *x == self.cursor);
        log::info!("current: {:?}", current_path_index);
        if let Some(current_path_index) = current_path_index {
            if let Some(path) = flattened_paths.get(current_path_index + 1) {
                log::info!("new path: {:?}", path);
                self.cursor = path.clone();
            }
        } else {
            if let Some(path) = flattened_paths.get(0) {
                self.cursor = path.clone();
            }
        }
    }

    /*
    fn set_node(&mut self, node: Node) {
        let mut node = node;

        let current_ref = self.current_ref();
        let node_kind = SCHEMA.get_kind(&node.kind);

        if let (Some(current_ref), Some(inner_field)) =
            (current_ref, node_kind.and_then(|k| k.inner()))
        {
            node.children
                .insert(inner_field.to_string(), vec![current_ref.clone()]);
        };

        let new_ref = self.file.add_node(node);

        let selector = self.cursor.back().unwrap().clone();
        let parent_ref = self.parent_ref().unwrap();
        log::info!("parent ref: {:?}", parent_ref);
        let mut parent = self.file.lookup(&parent_ref).unwrap().clone();
        log::info!("parent: {:?}", parent);

        // If the field does not exist, create a default one.
        let children = parent.children.entry(selector.field).or_default();
        match children.get_mut(selector.index) {
            Some(c) => *c = new_ref,
            None => children.push(new_ref),
        };
        self.file.replace_node(&parent_ref, parent);
    }
    */

    /*
    fn replace_node(&mut self, node: Node) {
        let path = self.cursor.clone();
        self.replace_node_path(&path, node)
    }

    fn replace_node_path(&mut self, path: &[Selector], node: Node) {
        match self.file.lookup(path) {
            Some(current_ref) => {
                self.file.replace_node(&current_ref, node);
            }
            None => {
                let new_ref = self.file.add_node(node);
                let (selector, parent_path) = path.split_last().unwrap().clone();
                let parent_ref = self.lookup_path(&self.file.root, &parent_path).unwrap();
                log::info!("parent ref: {:?}", parent_ref);
                let mut parent = self.file.lookup(&parent_ref).unwrap().clone();
                log::info!("parent: {:?}", parent);
                // If the field does not exist, create a default one.
                let children = parent.children.entry(selector.field).or_default();
                match children.get_mut(selector.index) {
                    Some(c) => *c = new_ref,
                    None => children.push(new_ref),
                };
                self.file.replace_node(&parent_ref, parent);
            }
        }
    }
    */

    pub fn focus_command_line(&self) {
        Self::focus_element("#command-line");
    }

    pub fn focus_element(selector: &str) {
        log::info!("select {}", selector);
        if let Ok(Some(element)) = yew::utils::document().query_selector(selector) {
            element.dyn_into::<HtmlElement>().unwrap().focus().unwrap();
        } else {
            log::warn!("not found");
        }
    }

    pub fn scroll_into_view(&self, selector: &str) {
        if let Some(element) = yew::utils::document().query_selector(selector).unwrap() {
            element
                .dyn_into::<HtmlElement>()
                .unwrap()
                .scroll_into_view_with_bool(false)
        }
    }

    pub fn update_errors(&mut self) {
        self.update_errors_node(&self.cursor.clone());
    }

    pub fn update_errors_node(&mut self, path: &[Selector]) {
        let node = match self.file.lookup(path) {
            Some(node) => node.clone(),
            None => return,
        };
        let kind = &node.kind;

        if let Some(kind) = SCHEMA.get_kind(kind) {
            if let crate::schema::KindValue::Struct { validator, .. } = kind.value {
                let errors = validator(&ValidatorContext {
                    model: self,
                    node: &node,
                    path,
                });
                log::info!("errors: {:?} {:?}", path, errors);
            }
        }
        /*
        for (_, children) in node.children.iter() {
            for child in children {
                // TODO
                // self.update_errors_node(child);
            }
        }
        */
    }
}

#[derive(Clone, Debug)]
pub enum Msg {
    Noop,

    Select(Path),
    Hover(Path),

    Store,
    Load,

    Prev,
    Next,
    Parent,

    AddItem,
    DeleteItem,

    SetMode(Mode),

    ReplaceNode(Path, Node, bool),

    SetNodeCommand(Path, String),
    CommandKey(KeyboardEvent),
    PrevCommand,
    NextCommand,
    /* EnterCommand,
     * EscapeCommand,
     */
}

#[derive(Serialize, Deserialize, Debug)]
pub struct File {
    pub nodes: HashMap<Hash, Node>,
    pub root: Hash,
    pub log: Vec<(Ref, Node)>,
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
            let (selector, rest) = path.split_first().unwrap().clone();
            let children = base.children.get(&selector.field)?;
            let child = children.get(selector.index)?;
            self.lookup_from(child, &rest)
        }
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
                .children
                .get(&selector.field)
                .and_then(|v| v.get(selector.index))
            {
                Some(old_child_hash) => {
                    let new_child_hash =
                        self.replace_node_from(old_child_hash, &path[1..], node)?;
                    base.children.get_mut(&selector.field)?[selector.index] = new_child_hash;
                }
                None => {
                    // WARN: Only works for one level of children.
                    let new_child_hash = self.add_node(&node);
                    base.children
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
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Node {
    pub kind: String,
    pub value: String,
    pub children: HashMap<String, Vec<Hash>>,
}

fn display_selector(selector: &Selector) -> Vec<Html> {
    let mut segments = Vec::new();
    segments.push(html! {
        <span>
          { selector.field.clone() }
              { format!("[{}]", selector.index) }
        </span>
    });
    segments.push(html! {
        <span>
          { format!("[{}]", selector.index) }
        </span>
    });
    segments
}

fn display_cursor(cursor: &Path) -> Html {
    let segments = cursor
        .iter()
        .flat_map(|s| display_selector(s))
        .intersperse(html! { <span>{ ">" }</span>});
    html! {
        <div>{ for segments }</div>
    }
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn view(&self) -> Html {
        let onmouseover = self.link.callback(move |e: MouseEvent| {
            e.stop_propagation();
            Msg::Hover(vec![].into())
        });
        html! {
            <div
            //   onkeydown=onkeypress
              onmouseover=onmouseover
              >
                <div>{ "LINC" }</div>
                <div>{ "click on an empty node to see list of possible completions" }</div>
                <div class="">
                    <div class="column">{ self.view_file(&self.file) }</div>

                    <div class="column">
                        <div>{ "Mode: " }{ format!("{:?}", self.mode) }</div>
                        <div>{ display_cursor(&self.cursor) }</div>
                        <div>{ format!("Ref: {:?}", self.file.lookup(&self.cursor)) }</div>
                    </div>

                    <div>{ self.view_actions() }</div>
                    <div class="h-40">
                        <div class="column">{ self.view_file_json(&self.file) }</div>
                    </div>
                </div>
            </div>
        }
    }

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let key_listener = KeyboardService::register_key_down(
            &yew::utils::window(),
            link.callback(move |e: KeyboardEvent| {
                // e.stop_propagation();
                // e.stop_immediate_propagation();
                // e.prevent_default();
                Msg::CommandKey(e)
            }),
        );
        Model {
            store: StorageService::new(Area::Local).expect("could not create storage service"),
            key_listener,
            file: super::initial::initial(),
            mode: Mode::Normal,
            cursor: vec![].into(),
            hover: vec![].into(),
            link,
            node_state: HashMap::new(),
            parsed_commands: Vec::new(),
            selected_command_index: 0,
            errors: vec![],
        }
    }

    fn change(&mut self, _props: ()) -> ShouldRender {
        false
    }

    fn rendered(&mut self, _first_render: bool) {}

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        if let Msg::Hover(_) = msg {
            return false;
        }
        log::info!("update {:?}", msg);
        fn update_from_selected(model: &mut Model) {
            let current_node = model.file.lookup(&model.cursor);
            let _current_kind = current_node.clone().map(|n| n.kind.clone());

            let parsed_commands = model.parse_commands(&model.cursor);
            log::debug!("parsed commands {:?}", parsed_commands);
            model.parsed_commands = parsed_commands;

            let command_input_id = crate::view::command_input_id(&model.cursor);
            Model::focus_element(&format!("#{}", command_input_id));
        }

        const KEY: &str = "linc_file";
        match msg {
            Msg::Noop => {}
            Msg::Select(path) => {
                self.cursor = path.clone();
                update_from_selected(self);
            }
            Msg::Hover(path) => {
                self.hover = path;
            }
            // TODO: sibling vs inner
            Msg::Prev => {
                self.prev();
                update_from_selected(self);
            }
            // Preorder tree traversal.
            Msg::Next => {
                self.next();
                update_from_selected(self);
            }
            Msg::Parent => {
                self.cursor = self.cursor[..self.cursor.len() - 1].to_vec();
                update_from_selected(self);
            }
            Msg::Store => {
                self.store.store(KEY, yew::format::Json(&self.file));
            }
            Msg::Load => {
                if let yew::format::Json(Ok(file)) = self.store.restore(KEY) {
                    self.file = file;
                }
            }
            Msg::SetMode(mode) => {
                self.mode = mode;
            }
            Msg::ReplaceNode(path, node, mv) => {
                log::info!("replace node {:?} {:?}", path, node);
                let new_root = self.file.replace_node(&path, node);
                log::info!("new root: {:?}", new_root);
                if let Some(new_root) = new_root {
                    self.file.root = new_root;
                }
                if mv {
                    self.link.send_message(Msg::Next);
                    self.parsed_commands = vec![];
                    self.selected_command_index = 0;
                }
            }
            Msg::SetNodeCommand(path, raw_command) => {
                let mut node = self.file.lookup(&path).cloned().unwrap_or_default();
                node.value = raw_command.clone();
                let new_root = self.file.replace_node(&path, node);
                let parsed_commands = self
                    .parse_commands(&path)
                    .into_iter()
                    // TODO: Fuzzy match.
                    .filter(|c| c.label.starts_with(&raw_command))
                    .collect();
                self.parsed_commands = parsed_commands;
                self.selected_command_index = 0;
                if let Some(new_root) = new_root {
                    self.file.root = new_root;
                }
            }
            Msg::AddItem => {
                let (selector, parent_path) = self.cursor.split_last().unwrap().clone();
                let new_ref = self.file.add_node(&Node {
                    kind: "invalid".to_string(),
                    value: "invalid".to_string(),
                    children: HashMap::new(),
                });
                let mut parent = self.file.lookup(parent_path).unwrap().clone();
                // If the field does not exist, create a default one.
                let children = parent.children.entry(selector.field.clone()).or_default();
                let new_index = selector.index + 1;
                children.insert(new_index, new_ref);
                self.file.replace_node(parent_path, parent);
                // Select newly created element.
                self.cursor.last_mut().unwrap().index = new_index;
                self.next();
            }
            Msg::DeleteItem => {
                let (selector, parent_path) = self.cursor.split_last().unwrap().clone();
                let mut parent = self.file.lookup(parent_path).unwrap().clone();
                // If the field does not exist, create a default one.
                let children = parent.children.entry(selector.field.clone()).or_default();
                children.remove(selector.index);
                if let Some(new_root) = self.file.replace_node(parent_path, parent) {
                    self.file.root = new_root;
                }
            }
            Msg::PrevCommand => {
                self.selected_command_index = if self.selected_command_index > 0 {
                    self.selected_command_index - 1
                } else {
                    self.parsed_commands.len() - 1
                };
            }
            Msg::NextCommand => {
                if self.parsed_commands.len() > 0 {
                    self.selected_command_index =
                        (self.selected_command_index + 1) % self.parsed_commands.len();
                }
            }
            Msg::CommandKey(e) => {
                log::info!("key: {}", e.key());
                let selection = yew::utils::window().get_selection().unwrap().unwrap();
                let anchor_node = selection.anchor_node().unwrap();
                let _anchor_offset = selection.anchor_offset();
                let anchor_node_value = anchor_node.node_value().unwrap_or_default();
                log::info!(
                    "selection: {:?} {} {}",
                    selection,
                    selection.anchor_offset(),
                    anchor_node_value
                );

                // See https://developer.mozilla.org/en-US/docs/Web/API/KeyboardEvent/code
                match e.key().as_ref() {
                    "Enter" => {
                        e.prevent_default();
                        if let Some(selected_command) =
                            self.parsed_commands.get(self.selected_command_index)
                        {
                            let node = selected_command.to_node();
                            self.link.send_message(Msg::ReplaceNode(
                                self.cursor.clone(),
                                node,
                                true,
                            ));
                        }
                    }
                    // "Enter" if self.mode == Mode::Edit =>
                    // self.link.send_message(Msg::EnterCommand), "Escape" =>
                    // self.link.send_message(Msg::EscapeCommand),
                    "ArrowUp" => self.link.send_message(Msg::PrevCommand),
                    "ArrowDown" => self.link.send_message(Msg::NextCommand),
                    "ArrowLeft" if self.mode == Mode::Normal => self.link.send_message(Msg::Prev),
                    "ArrowRight" if self.mode == Mode::Normal => self.link.send_message(Msg::Next),
                    /*
                    "i" if self.mode == Mode::Normal => {
                        e.prevent_default();
                        self.link.send_message(Msg::SetMode(Mode::Edit))
                    }
                    "c" if self.mode == Mode::Normal => {
                        e.prevent_default();
                        self.link.send_message(Msg::SetMode(Mode::Edit))
                    }
                    "e" if self.mode == Mode::Normal => {
                        e.prevent_default();
                        self.link.send_message(Msg::SetMode(Mode::Edit))
                    }
                    */
                    _ => {}
                }
            }
        };
        // self.focus_command_line();
        self.update_errors();
        true
    }
}

pub struct Action {
    pub image: Option<String>,
    pub text: String,
    pub msg: Msg,
}
