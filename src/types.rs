use crate::schema::SCHEMA;
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

    pub raw_command: String,
    pub parsed_commands: Vec<Node>,
    pub selected_command_index: usize,
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
                let children = &base.children.get(&head.field).cloned().unwrap_or_default();
                match head.index {
                    Some(index) => {
                        let r = children.get(index).cloned().unwrap_or_default();
                        self.lookup_path(&r, path)
                    }
                    None => None,
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

    fn parse_commands(&mut self) -> Vec<Node> {
        self.current_field()
            .map(|field| {
                field
                    .kind
                    .iter()
                    .filter_map(|kind| SCHEMA.get_kind(kind))
                    .filter_map(|kind| {
                        (kind.parser)(&self.raw_command).map(|value| Node {
                            kind: kind.name.to_string(),
                            value,
                            children: HashMap::new(),
                        })
                    })
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default()
        // let mut value = match command {
        //     "false" => Some(Value::Bool(false)),
        //     "true" => Some(Value::Bool(true)),
        //     _ => {
        //         if let Some(v) = command.strip_prefix('"') {
        //             Some(Value::String(v.to_string()))
        //         } else if let Ok(v) = command.parse::<i32>() {
        //             Some(Value::Int(v))
        //         } else if let Some(_) = SCHEMA.kinds.iter().find(|k| k.name == command) {
        //             Some(Value::Inner(Inner {
        //                 kind: command.to_string(),
        //                 children: HashMap::new(),
        //             }))
        //         } else {
        //             None
        //         }
        //     }
        // };
        // if let Some(Value::Inner(ref mut inner)) = value {
        //     let kind = SCHEMA.kinds.iter().find(|k| k.name == inner.kind).unwrap();
        //     if let Some(inner_field) = kind.inner {
        //         if let Some(reference) = self.current_ref() {
        //             inner
        //                 .children
        //                 .entry(inner_field.to_string())
        //                 .or_default()
        //                 .push(reference);
        //         }
        //     }
        // }
        // if let Some(v) = &value {
        //     let valid = self.is_valid_value(v);
        //     log::info!("valid: {:?}", valid);
        // }
        // value
    }

    fn prev(&mut self) {
        let flattened_paths = self.flatten_paths(&self.file.root, Path::new());
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

    fn set_value(&mut self, node: Node) {
        let new_ref = self.file.add_node(node);

        let selector = self.cursor.back().unwrap().clone();
        let parent_ref = self.parent_ref().unwrap();
        log::info!("parent ref: {:?}", parent_ref);
        let parent = self.lookup_mut(&parent_ref).unwrap();
        log::info!("parent: {:?}", parent);

        // If the field does not exist, create a default one.
        let children = parent.children.entry(selector.field).or_default();
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
    ReplaceCurrentNode(Node),
    CommandKey(KeyboardEvent),
}

#[derive(Serialize, Deserialize)]
pub struct File {
    pub nodes: HashMap<Ref, Node>,
    pub root: Ref,
}

impl File {
    pub fn lookup(&self, reference: &Ref) -> Option<&Node> {
        self.nodes.get(reference)
    }

    fn lookup_mut(&mut self, reference: &Ref) -> Option<&mut Node> {
        self.nodes.get_mut(reference)
    }

    fn add_node(&mut self, node: Node) -> Ref {
        let reference = new_ref();
        self.nodes.insert(reference.clone(), node);
        reference
    }
}

// TODO: Navigate to children directly, but use :var to navigate to variables, otherwise skip them
// when navigating.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Node {
    pub kind: String,
    pub value: String,
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
        .intersperse(html! { <span>{ ">" }</span>});
    html! {
        <div>{ for segments }</div>
    }
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn view(&self) -> Html {
        let callback = self.link.callback(|v: String| Msg::SetCommand(v));
        let onkeypress = self
            .link
            .callback(move |e: KeyboardEvent| Msg::CommandKey(e));
        let oninput = self
            .link
            .callback(move |e: InputData| Msg::SetCommand(e.value));
        let values = self.parsed_commands.iter().enumerate().map(|(i, v)| {
            let s = v.clone();
            // let callback = self.link.callback(move |_| Msg::ReplaceCurrentNode(v));
            let mut classes = vec!["border", "border-solid", "border-blue-500"];
            if self.selected_command_index == i {
                classes.push("bg-yellow-500");
            }
            html! {
                <div
                //   onclick=callback
                // XXX
                  class=classes.join(" ")>{ v.kind.clone() }
                </div>
            }
        });
        html! {
            <div onkeydown=onkeypress>
                <div>{ "LINC" }</div>
                <div>{ self.view_actions() }</div>
                <div class="grid grid-rows-2">
                    // <CommandLine values=allowed_kinds on_change=callback base_value=self.command.clone() state=state />
                    <div class="h-40">
                        <input oninput=oninput value=self.raw_command />
                        { for values }
                    </div>
                    <div class="h-40">
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
            raw_command: "".to_string(),
            parsed_commands: vec![],
            selected_command_index: 0,
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
                self.parsed_commands = self.parse_commands();
            }
            // TODO: sibling vs inner
            Msg::Prev => {
                self.prev();
                self.parsed_commands = self.parse_commands();
            }
            // Preorder tree traversal.
            Msg::Next => {
                self.next();
                self.parsed_commands = self.parse_commands();
            }
            Msg::Parent => {
                self.cursor.pop_back();
                self.parsed_commands = self.parse_commands();
            }
            Msg::Store => {
                self.store.store(KEY, yew::format::Json(&self.file));
            }
            Msg::Load => {
                if let yew::format::Json(Ok(file)) = self.store.restore(KEY) {
                    self.file = file;
                }
            }
            Msg::ReplaceCurrentNode(n) => {
                self.file.nodes.insert(self.current_ref().unwrap(), n);
                self.parsed_commands = self.parse_commands();
                self.selected_command_index = 0;
            }
            Msg::AddItem => {
                let selector = self.cursor.back().unwrap().clone();
                let parent_ref = self.parent_ref().unwrap();
                let new_ref = self.file.add_node(Node {
                    kind: "invalid".to_string(),
                    value: "invalid".to_string(),
                    children: HashMap::new(),
                });
                let parent = self.lookup_mut(&parent_ref).unwrap();
                log::info!("inner");
                // If the field does not exist, create a default one.
                let children = parent.children.entry(selector.field).or_default();
                let new_index = selector.index.map(|i| i + 1).unwrap_or(0);
                children.insert(new_index, new_ref);
                // Select newly created element.
                self.cursor.back_mut().unwrap().index = Some(new_index);
            }
            Msg::DeleteItem => {
                let selector = self.cursor.back().unwrap().clone();
                let parent_ref = self.parent_ref().unwrap();
                let parent = self.lookup_mut(&parent_ref).unwrap();
                // If the field does not exist, create a default one.
                let children = parent.children.entry(selector.field).or_default();
                children.remove(selector.index.unwrap());
            }
            Msg::SetCommand(v) => {
                self.raw_command = v;
                self.parsed_commands = self.parse_commands();
                self.selected_command_index = 0;
            }
            Msg::CommandKey(v) => {
                log::info!("key: {}", v.key());
                // See https://developer.mozilla.org/en-US/docs/Web/API/KeyboardEvent/code
                match v.key().as_ref() {
                    "Enter" => match self
                        .parsed_commands
                        .get(self.selected_command_index)
                        .clone()
                    {
                        Some(node) => {
                            // Replace current node.
                            // self.file
                            //     .nodes
                            //     .insert(self.current_ref().unwrap(), node.clone());
                            self.set_value(node.clone());
                            self.next();
                            self.raw_command = "".to_string();
                            self.parsed_commands = self.parse_commands();
                            self.selected_command_index = 0;
                        }
                        None => log::info!("invalid command"),
                    },
                    "Escape" => {
                        self.raw_command = "".to_string();
                        self.parsed_commands = self.parse_commands();
                        self.selected_command_index = 0;
                    }
                    "ArrowUp" => {
                        if self.selected_command_index > 0 {
                            self.selected_command_index -= 1;
                        }
                    }
                    "ArrowDown" => {
                        if self.selected_command_index < (self.parsed_commands.len() - 1) {
                            self.selected_command_index += 1;
                        }
                    }
                    "ArrowRight" if self.raw_command.is_empty() => {
                        self.next();
                        self.parsed_commands = self.parse_commands();
                        self.selected_command_index = 0;
                    }
                    "ArrowLeft" if self.raw_command.is_empty() => {
                        self.prev();
                        self.parsed_commands = self.parse_commands();
                        self.selected_command_index = 0;
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
