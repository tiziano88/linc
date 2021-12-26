use crate::schema::{
    FieldValidator, KindValue, ParsedValue, ValidationError, ValidatorContext, SCHEMA,
};
use gloo_storage::{LocalStorage, Storage};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::{
    collections::{BTreeMap, HashMap},
    convert::TryInto,
};
use wasm_bindgen::{JsCast, UnwrapThrowExt};
use web_sys::{window, HtmlElement, HtmlInputElement, InputEvent};
use yew::{html, prelude::*, Component, Html, KeyboardEvent};

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

    pub cursor: Path,
    pub hover: Path,
    pub mode: Mode,

    pub node_state: HashMap<Path, NodeState>,

    pub parsed_commands: Vec<ParsedValue>,
    pub selected_command_index: usize,

    pub errors: Vec<ValidationError>,

    pub show_serialized: bool,
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
        if let Ok(Some(element)) = window()
            .unwrap()
            .document()
            .unwrap()
            .query_selector(selector)
        {
            element.dyn_into::<HtmlElement>().unwrap().focus().unwrap();
        } else {
            log::warn!("not found");
        }
    }

    pub fn scroll_into_view(&self, selector: &str) {
        if let Some(element) = window()
            .unwrap()
            .document()
            .unwrap()
            .query_selector(selector)
            .unwrap()
        {
            element
                .dyn_into::<HtmlElement>()
                .unwrap()
                .scroll_into_view_with_bool(false)
        }
    }

    pub fn update_errors(&mut self, ctx: &Context<Self>) {
        self.update_errors_node(ctx, &self.cursor.clone());
    }

    pub fn update_errors_node(&mut self, ctx: &Context<Self>, path: &[Selector]) {
        let node = match self.file.lookup(path) {
            Some(node) => node.clone(),
            None => return,
        };
        let kind = &node.kind;

        if let Some(kind) = SCHEMA.get_kind(kind) {
            let crate::schema::KindValue::Struct { validator, .. } = kind.value;
            let errors = validator(&ValidatorContext {
                model: self,
                ctx,
                node: &node,
                path,
                entries: &[],
                placeholder: "",
            });
            log::info!("errors: {:?} {:?}", path, errors);
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
    Parse(String),

    Prev,
    Next,
    Parent,

    AddItem,
    DeleteItem,

    SetMode(Mode),

    ReplaceNode(Path, Node, bool),
    AddField(Path, String),

    SetNodeValue(Path, String),

    CommandKey(Path, KeyboardEvent),
    PrevCommand,
    NextCommand,

    ToggleSerialized,
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
    pub children: BTreeMap<String, Vec<Hash>>,
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

    fn view(&self, ctx: &Context<Self>) -> Html {
        let onmouseover = ctx.link().callback(move |e: MouseEvent| {
            e.stop_propagation();
            Msg::Hover(vec![].into())
        });
        let parse = ctx
            .link()
            .callback(move |e: InputEvent| Msg::Parse(get_value_from_input_event(e)));

        let serialized = if self.show_serialized {
            html! {
                <div class="column">{ self.view_file_json(&self.file) }</div>
            }
        } else {
            html! {}
        };
        html! {
            <div
            //   onkeydown=onkeypress
              onmouseover={ onmouseover }
              >
                <div>{ "LINC" }</div>
                <div>{ "click on an empty node to see list of possible completions" }</div>
                <div class="">
                    <div class="column">{ self.view_file(ctx, &self.file) }</div>

                    <div class="column">
                        <div>{ "Mode: " }{ format!("{:?}", self.mode) }</div>
                        <div>{ display_cursor(&self.cursor) }</div>
                        <div>{ format!("Ref: {:?}", self.file.lookup(&self.cursor)) }</div>
                    </div>

                    <div>{ self.view_actions(ctx) }</div>
                    <div class="h-40">
                        <textarea type="text" class="border-solid border-black border" oninput={ parse } />
                        { serialized }
                    </div>
                </div>
            </div>
        }
    }

    fn create(_ctx: &Context<Self>) -> Self {
        // let key_listener = KeyboardService::register_key_down(
        //     &window().unwrap(),
        //     ctx.link().callback(move |e: KeyboardEvent| {
        //         // e.stop_propagation();
        //         // e.stop_immediate_propagation();
        //         // e.prevent_default();
        //         Msg::CommandKey(e)
        //     }),
        // );
        Model {
            file: super::initial::initial(),
            mode: Mode::Normal,
            cursor: vec![].into(),
            hover: vec![].into(),
            node_state: HashMap::new(),
            parsed_commands: Vec::new(),
            selected_command_index: 0,
            errors: vec![],
            show_serialized: false,
        }
    }

    fn changed(&mut self, _ctx: &Context<Self>) -> bool {
        false
    }

    fn rendered(&mut self, _ctx: &Context<Self>, _first_render: bool) {}

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        if let Msg::Hover(_) = msg {
            return false;
        }
        log::info!("update {:?}", msg);
        fn update_from_selected(model: &mut Model) {
            let current_node = model.file.lookup(&model.cursor);
            let _current_kind = current_node.clone().map(|n| n.kind.clone());

            let parsed_commands = model.parse_commands(&model.cursor);
            log::debug!("parsed commands {:?}", parsed_commands);
            let filtered_commands = match current_node {
                Some(node) => parsed_commands
                    .into_iter()
                    .filter(|c| c.label.starts_with(&node.value))
                    .collect(),
                None => parsed_commands,
            };
            log::debug!("filtered commands {:?}", filtered_commands);
            model.parsed_commands = filtered_commands;
            model.selected_command_index = 0;

            let command_input_id = crate::view::command_input_id(&model.cursor);
            Model::focus_element(&format!("#{}", command_input_id));
        }

        const KEY: &str = "linc_file";
        match msg {
            Msg::Noop => {}
            Msg::ToggleSerialized => {
                self.show_serialized = !self.show_serialized;
            }
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
                LocalStorage::set(KEY, &self.file).unwrap();
            }
            Msg::Load => {
                if let Ok(file) = LocalStorage::get(KEY) {
                    self.file = file;
                }
            }
            Msg::Parse(v) => {
                log::debug!("parse {:?}", v);
                let html = html_parser::Dom::parse(&v).unwrap();
                log::debug!("parsed {:?}", html);
                fn add_string(model: &mut Model, value: &str) -> Hash {
                    model.file.add_node(&Node {
                        kind: "string".into(),
                        value: value.into(),
                        children: BTreeMap::new(),
                    })
                }
                fn add_node(model: &mut Model, node: &html_parser::Node) -> Hash {
                    match node {
                        html_parser::Node::Element(e) => {
                            let mut children: BTreeMap<String, Vec<String>> = BTreeMap::new();
                            e.attributes.iter().for_each(|(k, v)| {
                                children.entry(k.clone()).or_insert_with(Vec::new).push(
                                    add_string(model, &v.as_ref().cloned().unwrap_or_default()),
                                );
                            });
                            e.children.iter().for_each(|v| {
                                children
                                    .entry("children".to_string())
                                    .or_insert_with(Vec::new)
                                    .push(add_node(model, v));
                            });
                            model.file.add_node(&Node {
                                kind: e.name.clone(),
                                value: "".into(),
                                children,
                            })
                        }
                        html_parser::Node::Text(t) => add_string(model, t),
                        html_parser::Node::Comment(c) => add_string(model, c),
                    }
                }
                fn add_dom(model: &mut Model, e: &html_parser::Dom) -> Hash {
                    let mut children: BTreeMap<String, Vec<String>> = BTreeMap::new();
                    e.children.iter().for_each(|v| {
                        children
                            .entry("children".to_string())
                            .or_insert_with(Vec::new)
                            .push(add_node(model, v));
                    });
                    model.file.add_node(&Node {
                        kind: "dom".into(),
                        value: "".to_string(),
                        children,
                    })
                }
                let h = add_dom(self, &html);
                self.file.root = h;
            }
            Msg::SetMode(mode) => {
                self.mode = mode;
            }
            Msg::AddField(path, field) => {
                let mut node = self.file.lookup(&path).cloned().unwrap();
                node.children
                    .entry(field.clone())
                    .or_insert_with(Vec::new)
                    .push("".into());
                let n = node.children[&field].len();
                let new_root = self.file.replace_node(&path, node);
                if let Some(new_root) = new_root {
                    self.file.root = new_root;
                }
                self.cursor = append(
                    &path,
                    Selector {
                        field,
                        index: n - 1,
                    },
                );
                update_from_selected(self);
            }
            Msg::ReplaceNode(path, node, mv) => {
                log::info!("replace node {:?} {:?}", path, node);
                let new_root = self.file.replace_node(&path, node);
                log::info!("new root: {:?}", new_root);
                if let Some(new_root) = new_root {
                    self.file.root = new_root;
                }
                if mv {
                    ctx.link().send_message(Msg::Next);
                    self.parsed_commands = vec![];
                    self.selected_command_index = 0;
                }
            }
            Msg::SetNodeValue(path, value) => {
                self.cursor = path.clone();
                let mut node = self.file.lookup(&path).cloned().unwrap_or_default();
                node.value = value.clone();
                let new_root = self.file.replace_node(&path, node);
                if let Some(new_root) = new_root {
                    self.file.root = new_root;
                }
                update_from_selected(self);
            }
            Msg::AddItem => {
                let (selector, parent_path) = self.cursor.split_last().unwrap().clone();
                let new_ref = self.file.add_node(&Node {
                    kind: "invalid".to_string(),
                    value: "invalid".to_string(),
                    children: BTreeMap::new(),
                });
                let mut parent = self.file.lookup(parent_path).unwrap().clone();
                // If the field does not exist, create a default one.
                let children = parent.children.entry(selector.field.clone()).or_default();
                let new_index = selector.index + 1;
                children.insert(new_index, new_ref);
                self.file.replace_node(parent_path, parent);
                // Select newly created element.
                self.cursor.last_mut().unwrap().index = new_index;
                // self.next();
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
                    if self.parsed_commands.len() > 0 {
                        self.parsed_commands.len() - 1
                    } else {
                        0
                    }
                };
            }
            Msg::NextCommand => {
                if self.parsed_commands.len() > 0 {
                    self.selected_command_index =
                        (self.selected_command_index + 1) % self.parsed_commands.len();
                }
            }
            Msg::CommandKey(path, e) => {
                log::info!("key: {}", e.key());
                self.cursor = path.clone();
                let node = self.file.lookup(&path).cloned().unwrap_or_default();

                let selection = window().unwrap().get_selection().unwrap().unwrap();
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
                        self.mode = Mode::Edit;
                        if let Some(selected_command) =
                            self.parsed_commands.get(self.selected_command_index)
                        {
                            let node = selected_command.to_node();
                            ctx.link().send_message(Msg::ReplaceNode(
                                self.cursor.clone(),
                                node,
                                true,
                            ));
                        }
                        // If it is a pure value, select the parent again so another field may be
                        // added.
                        if node.kind.is_empty() {
                            self.cursor = self.cursor[..self.cursor.len() - 1].to_vec();
                            update_from_selected(self);
                        }
                    }
                    "Escape" => {
                        self.mode = Mode::Normal;
                        // If it is a pure value, select the parent again so another field may be
                        // added.
                        if node.kind.is_empty() {
                            self.cursor = self.cursor[..self.cursor.len() - 1].to_vec();
                            update_from_selected(self);
                        }
                    }
                    // "Enter" if self.mode == Mode::Edit =>
                    // self.link.send_message(Msg::EnterCommand), "Escape" =>
                    // self.link.send_message(Msg::EscapeCommand),
                    "ArrowUp" => ctx.link().send_message(Msg::PrevCommand),
                    "ArrowDown" => ctx.link().send_message(Msg::NextCommand),
                    "ArrowLeft" if self.mode == Mode::Normal => ctx.link().send_message(Msg::Prev),
                    "ArrowRight" if self.mode == Mode::Normal => ctx.link().send_message(Msg::Next),
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
        self.update_errors(ctx);
        true
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
    let target: HtmlInputElement = event_target.dyn_into().unwrap_throw();
    // web_sys::console::log_1(&target.value().into());
    target.value()
}
