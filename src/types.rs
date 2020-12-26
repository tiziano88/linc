use crate::schema::SCHEMA;
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, VecDeque};
use wasm_bindgen::JsCast;
use yew::{
    html,
    services::{storage::Area, StorageService},
    web_sys::HtmlElement,
    Component, ComponentLink, FocusEvent, Html, InputData, KeyboardEvent, ShouldRender,
};

pub type Ref = String;

pub const INVALID_REF: &str = "-";

pub fn new_ref() -> Ref {
    uuid::Uuid::new_v4().to_hyphenated().to_string()
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Selector {
    pub field: String,
    pub index: usize,
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
                let r = children.get(head.index).cloned().unwrap_or_default();
                self.lookup_path(&r, path)
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
                    .flat_map(|kind| {
                        (kind.parser)(&self.raw_command)
                            .into_iter()
                            // TODO: Different matching logic (e.g. fuzzy).
                            .filter(|v| v.starts_with(&self.raw_command))
                            .map(move |value| Node {
                                kind: kind.name.to_string(),
                                value,
                                children: HashMap::new(),
                            })
                    })
                    .collect::<Vec<_>>()
                // TODO: Ranking.
            })
            .unwrap_or_default()
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
        let mut node = node;

        let current_ref = self.current_ref();
        let node_kind = SCHEMA.get_kind(&node.kind);

        if let (Some(current_ref), Some(inner_field)) =
            (current_ref, node_kind.and_then(|k| k.inner))
        {
            if current_ref != INVALID_REF {
                node.children
                    .insert(inner_field.to_string(), vec![current_ref.clone()]);
            }
        };

        let new_ref = self.file.add_node(node);

        let selector = self.cursor.back().unwrap().clone();
        let parent_ref = self.parent_ref().unwrap();
        log::info!("parent ref: {:?}", parent_ref);
        let parent = self.lookup_mut(&parent_ref).unwrap();
        log::info!("parent: {:?}", parent);

        // If the field does not exist, create a default one.
        let children = parent.children.entry(selector.field).or_default();
        match children.get_mut(selector.index) {
            Some(c) => *c = new_ref,
            None => children.push(new_ref),
        };
    }

    pub fn focus_command_line(&self) {
        yew::utils::document()
            .query_selector("#command-line")
            .unwrap()
            .unwrap()
            .dyn_into::<HtmlElement>()
            .unwrap()
            .focus()
            .unwrap();
    }
}

#[derive(Clone)]
pub enum Msg {
    Noop,

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
        let onkeypress = self
            .link
            .callback(move |e: KeyboardEvent| Msg::CommandKey(e));
        let oninput = self
            .link
            .callback(move |e: InputData| Msg::SetCommand(e.value));
        let onblur = self.link.callback(move |e: FocusEvent| Msg::Noop);
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
                  class=classes.join(" ")>
                  <span class="font-mono">
                    { v.kind.clone() }
                  </span>
                  <span>{ "::" }</span>
                  <span class="font-mono">
                    { v.value.clone() }
                  </span>
                </div>
            }
        });
        html! {
            <div onkeydown=onkeypress>
                <div>{ "LINC" }</div>
                <div>{ "left / right arrow keys (when command line is empty): move between existing nodes" }</div>
                <div>{ "up / down arrow keys: select alternative completion result" }</div>
                <div>{ "start typing in command line to filter available completion results" }</div>
                <div>{ self.view_actions() }</div>
                <div class="grid grid-rows-2">
                    // <CommandLine values=allowed_kinds on_change=callback base_value=self.command.clone() state=state />
                    <div class="h-40">
                        <input
                          id="command-line"
                          class="w-full border border-solid border-blue-500 bg-blue-100 font-mono"
                          oninput=oninput
                          onblur=onblur
                          value=self.raw_command
                        />
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
            cursor: vec![Selector {
                field: "items".to_string(),
                index: 0,
            }]
            .into(),
            link,
        }
    }

    fn change(&mut self, _props: ()) -> ShouldRender {
        self.focus_command_line();
        false
    }

    fn rendered(&mut self, _first_render: bool) {
        self.focus_command_line();
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        const KEY: &str = "linc_file";
        match msg {
            Msg::Noop => {}
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
                // If the field does not exist, create a default one.
                let children = parent.children.entry(selector.field).or_default();
                let new_index = selector.index + 1;
                children.insert(new_index, new_ref);
                // Select newly created element.
                self.cursor.back_mut().unwrap().index = new_index;
            }
            Msg::DeleteItem => {
                let selector = self.cursor.back().unwrap().clone();
                let parent_ref = self.parent_ref().unwrap();
                let parent = self.lookup_mut(&parent_ref).unwrap();
                // If the field does not exist, create a default one.
                let children = parent.children.entry(selector.field).or_default();
                children.remove(selector.index);
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
        self.focus_command_line();
        true
    }
}

pub struct Action {
    pub text: String,
    pub msg: Msg,
}
