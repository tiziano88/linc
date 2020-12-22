use crate::{
    command_line::{self, CommandLine},
    schema::SCHEMA,
};
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, VecDeque};
use yew::{
    html,
    prelude::*,
    services::{storage::Area, StorageService},
};

pub type Ref = String;

pub const INVALID_REF: &str = "-";

pub fn new_ref() -> Ref {
    uuid::Uuid::new_v4().to_hyphenated().to_string()
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Selector {
    pub field: String,
    pub index: Option<usize>,
}

pub fn field(name: &str) -> Selector {
    Selector {
        field: name.to_string(),
        index: None,
    }
}

pub type Path = VecDeque<Selector>;

pub fn append(path: &Path, selector: Selector) -> Path {
    let mut new_path = path.clone();
    new_path.push_back(selector);
    new_path
}

pub struct Model {
    pub file: File,
    pub store: StorageService,
    pub cursor: Path,
    pub link: ComponentLink<Self>,
    pub command: String,
    pub parsed_command: Option<Value>,
}

impl Model {
    pub fn lookup(&self, reference: &Ref) -> Option<&Node> {
        self.file.lookup(reference)
    }
    pub fn lookup_mut(&mut self, reference: &Ref) -> Option<&mut Node> {
        self.file.lookup_mut(reference)
    }
    pub fn lookup_path(&self, reference: &Ref, relative_path: Path) -> Option<Ref> {
        let mut path = relative_path;
        match path.pop_front() {
            Some(head) => {
                let base = self.lookup(reference).unwrap();
                match &base.value {
                    Value::Inner(v) => {
                        let children = &v.children.get(&head.field).cloned().unwrap_or_default();
                        match head.index {
                            Some(index) => {
                                let r = children.get(index).cloned().unwrap_or_default();
                                self.lookup_path(&r, path)
                            }
                            None => None,
                        }
                    }
                    _ => None,
                }
            }
            None => Some(reference.clone()),
        }
    }

    pub fn parent_ref(&self) -> Option<Ref> {
        let mut parent_cursor = self.cursor.clone();
        parent_cursor.pop_back().unwrap();
        self.lookup_path(&self.file.root, parent_cursor)
    }

    pub fn current_ref(&self) -> Option<Ref> {
        self.lookup_path(&self.file.root, self.cursor.clone())
    }

    fn parse_command(&mut self, command: &str) -> Option<Value> {
        let mut value = match command {
            "false" => Some(Value::Bool(false)),
            "true" => Some(Value::Bool(true)),
            _ => {
                if let Some(v) = command.strip_prefix('"') {
                    Some(Value::String(v.to_string()))
                } else if let Ok(v) = command.parse::<i32>() {
                    Some(Value::Int(v))
                } else if let Some(_) = SCHEMA.kinds.iter().find(|k| k.name == command) {
                    Some(Value::Inner(Inner {
                        kind: command.to_string(),
                        children: HashMap::new(),
                    }))
                } else {
                    None
                }
            }
        };
        if let Some(Value::Inner(ref mut inner)) = value {
            let kind = SCHEMA.kinds.iter().find(|k| k.name == inner.kind).unwrap();
            if let Some(inner_field) = kind.inner {
                if let Some(reference) = self.current_ref() {
                    inner
                        .children
                        .entry(inner_field.to_string())
                        .or_default()
                        .push(reference);
                }
            }
        }
        if let Some(v) = &value {
            let valid = self.is_valid_value(v);
            log::info!("valid: {:?}", valid);
        }
        value
    }

    fn prev(&mut self) {
        let flattened_paths = self.flatten_paths(&self.file.root, Path::new());
        log::info!("paths: {:?}", flattened_paths);
        let current_path_index = flattened_paths.iter().position(|x| *x == self.cursor);
        log::info!("current: {:?}", current_path_index);
        if let Some(current_path_index) = current_path_index {
            if let Some(path) = flattened_paths.get(current_path_index - 1) {
                self.cursor = path.clone();
            }
        }
    }

    fn next(&mut self) {
        let flattened_paths = self.flatten_paths(&self.file.root, Path::new());
        log::info!("paths: {:?}", flattened_paths);
        let current_path_index = flattened_paths.iter().position(|x| *x == self.cursor);
        log::info!("current: {:?}", current_path_index);
        if let Some(current_path_index) = current_path_index {
            if let Some(path) = flattened_paths.get(current_path_index + 1) {
                self.cursor = path.clone();
            }
        } else {
            if let Some(path) = flattened_paths.get(0) {
                self.cursor = path.clone();
            }
        }
    }

    fn set_value(&mut self, v: Value) {
        let new_ref = self.file.add_node(v);

        let selector = self.cursor.back().unwrap().clone();
        let parent_ref = self.parent_ref().unwrap();
        log::info!("parent ref: {:?}", parent_ref);
        let parent = self.lookup_mut(&parent_ref).unwrap();
        log::info!("parent: {:?}", parent);

        match &mut parent.value {
            Value::Inner(ref mut inner) => {
                // If the field does not exist, create a default one.
                let children = inner.children.entry(selector.field).or_default();
                match selector.index {
                    Some(i) => match children.get_mut(i) {
                        Some(c) => *c = new_ref,
                        None => children.push(new_ref),
                    },
                    // Cursor is pointing to a field but not a specific child, create the first
                    // child.
                    None => children.push(new_ref),
                }
            }
            _ => {}
        }
    }
}

#[derive(Clone)]
pub enum Msg {
    Select(Path),

    Store,
    Load,

    Prev,
    Next,
    Parent,

    AddItem,
    DeleteItem,

    SetCommand(String),
    CommandKey(KeyboardEvent),
}

#[derive(Serialize, Deserialize)]
pub struct File {
    pub nodes: Vec<Node>,
    pub root: Ref,
}

impl File {
    pub fn lookup(&self, reference: &Ref) -> Option<&Node> {
        self.nodes
            .iter()
            .filter(|v| v.reference == *reference)
            .next()
    }

    fn lookup_mut(&mut self, reference: &Ref) -> Option<&mut Node> {
        self.nodes
            .iter_mut()
            .filter(|v| v.reference == *reference)
            .next()
    }

    fn add_node(&mut self, value: Value) -> Ref {
        let reference = new_ref();
        let node = Node {
            reference: reference.clone(),
            value: value,
        };
        self.nodes.push(node);
        reference
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Node {
    pub reference: Ref,
    pub value: Value,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Value {
    Hole,

    Bool(bool),
    Int(i32),
    Float(f32),
    String(String),

    Inner(Inner),
}

// TODO: Navigate to children directly, but use :var to navigate to variables, otherwise skip them
// when navigating.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Inner {
    pub kind: String,
    pub children: HashMap<String, Vec<Ref>>,
}

fn display_selector(selector: &Selector) -> Vec<Html> {
    let mut segments = Vec::new();
    segments.push(html! {
        <span>
          { selector.field.clone() }
        </span>
    });
    if let Some(index) = selector.index {
        segments.push(html! {
            <span>
              { format!("[{}]", index) }
            </span>
        });
    }
    segments
}

fn display_cursor(cursor: &Path) -> Html {
    let segments = cursor
        .iter()
        .flat_map(|s| display_selector(s))
        .intersperse(html! { <span>{ ">"}</span>});
    html! {
        <div>{ for segments }</div>
    }
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn view(&self) -> Html {
        let callback = self.link.callback(|v: String| Msg::SetCommand(v));
        let allowed_kinds = self
            .current_field()
            .map(|f| f.type_.prefixes())
            .unwrap_or_default();
        let state = if self.command.is_empty() {
            command_line::State::Empty
        } else if self.parsed_command.is_some() {
            command_line::State::Valid
        } else {
            command_line::State::Invalid
        };
        let onkeypress = self
            .link
            .callback(move |e: KeyboardEvent| Msg::CommandKey(e));
        log::info!("allowed kinds: {:?}", allowed_kinds);
        html! {
            <div onkeydown=onkeypress>
                <div>{ "LINC" }</div>
                <div>{ self.view_actions() }</div>
                <div class="grid grid-rows-2">
                    <CommandLine values=allowed_kinds on_change=callback base_value=self.command.clone() state=state />
                    <div class="wrapper h-40">
                        <div class="column">{ self.view_file(&self.file) }</div>
                        <div class="column">
                            <div>{ display_cursor(&self.cursor) }</div>
                            <div>{ format!("Ref: {:?}", self.lookup_path(&self.file.root, self.cursor.clone())) }</div>
                        </div>
                        <div class="column">{ self.view_file_json(&self.file) }</div>
                    </div>
                </div>
            </div>
        }
    }

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Model {
            store: StorageService::new(Area::Local).expect("could not create storage service"),
            command: "".to_string(),
            parsed_command: None,
            file: super::initial::initial(),
            cursor: VecDeque::new(),
            link,
        }
    }

    fn change(&mut self, _props: ()) -> ShouldRender {
        false
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        const KEY: &str = "linc_file";
        match msg {
            Msg::Select(path) => {
                self.cursor = path;
            }

            // TODO: sibling vs inner
            Msg::Prev => {
                self.prev();
            }
            // Preorder tree traversal.
            Msg::Next => {
                self.next();
            }
            Msg::Parent => {
                self.cursor.pop_back();
            }
            Msg::Store => {
                self.store.store(KEY, yew::format::Json(&self.file));
            }
            Msg::Load => {
                if let yew::format::Json(Ok(file)) = self.store.restore(KEY) {
                    self.file = file;
                }
            }
            Msg::AddItem => {
                let selector = self.cursor.back().unwrap().clone();
                let parent_ref = self.parent_ref().unwrap();
                let parent = self.lookup_mut(&parent_ref).unwrap();
                let new_ref = INVALID_REF.to_string();
                match &mut parent.value {
                    Value::Inner(ref mut inner) => {
                        log::info!("inner");
                        // If the field does not exist, create a default one.
                        let children = inner.children.entry(selector.field).or_default();
                        let new_index = selector.index.map(|i| i + 1).unwrap_or(0);
                        children.insert(new_index, new_ref);
                        // Select newly created element.
                        self.cursor.back_mut().unwrap().index = Some(new_index);
                    }
                    _ => {}
                }
            }
            Msg::DeleteItem => {
                let selector = self.cursor.back().unwrap().clone();
                let parent_ref = self.parent_ref().unwrap();
                let parent = self.lookup_mut(&parent_ref).unwrap();
                match &mut parent.value {
                    Value::Inner(ref mut inner) => {
                        log::info!("inner");
                        // If the field does not exist, create a default one.
                        let children = inner.children.entry(selector.field).or_default();
                        children.remove(selector.index.unwrap());
                    }
                    _ => {}
                }
            }
            Msg::SetCommand(v) => {
                self.parsed_command = self.parse_command(&v);
                self.command = v;
            }
            Msg::CommandKey(v) => {
                log::info!("key: {}", v.key());
                // See https://developer.mozilla.org/en-US/docs/Web/API/KeyboardEvent/code
                match v.key().as_ref() {
                    "Enter" => match self.parsed_command.clone() {
                        Some(value) => {
                            if self.is_valid_value(&value) {
                                self.set_value(value);
                                self.parsed_command = None;
                                self.command = "".to_string();
                                self.next();
                            } else {
                                log::error!("invalid value: {:?}", value);
                            }
                        }
                        None => log::info!("invalid command: {}", self.command),
                    },
                    "Escape" => {
                        self.parsed_command = None;
                        self.command = "".to_string();
                    }
                    "ArrowRight" if self.command.is_empty() => {
                        self.next();
                    }
                    "ArrowLeft" if self.command.is_empty() => {
                        self.prev();
                    }
                    _ => {}
                }
            }
        };
        true
    }
}

pub struct Action {
    pub text: String,
    pub msg: Msg,
}
