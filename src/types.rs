use crate::command_line::CommandLine;
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
                } else if let Some(_) = RUST_SCHEMA.kinds.iter().find(|k| k.name == command) {
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
            let kind = RUST_SCHEMA
                .kinds
                .iter()
                .find(|k| k.name == inner.kind)
                .unwrap();
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
        // let selected_node_json = self
        //     .path
        //     .back()
        //     .and_then(|reference| self.lookup(reference))
        //     .map(|n| n.to_json())
        //     .unwrap_or("JSON ERROR".to_string());
        let callback = self.link.callback(|_: String| Msg::Next);
        html! {
            <div>
                <div>{ "LINC" }</div>
                <CommandLine options=vec!["first".to_string(), "second".to_string()] on_change=callback />
                <div>{ self.view_actions() }</div>
                <div class="wrapper">
                    <div class="column">{ self.view_file(&self.file) }</div>
                    <div class="column">
                        <div>{ display_cursor(&self.cursor) }</div>
                        <div>{ format!("Ref: {:?}", self.lookup_path(&self.file.root, self.cursor.clone())) }</div>
                        <div>{ format!("Current allowed kinds: {:?}", self.current_field().map(|field| &field.type_)) }</div>
                    </div>
                    <div class="column">{ self.view_file_json(&self.file) }</div>
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

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
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
                log::info!("key: {}", v.code());
                // See https://developer.mozilla.org/en-US/docs/Web/API/KeyboardEvent/code
                match v.code().as_ref() {
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

// https://doc.rust-lang.org/stable/reference/expressions.html
const RUST_EXPRESSION: Type = Type::Any(&[
    Type::Inner("if"),
    Type::Inner("string"),
    Type::Inner("field_access"),
    Type::Inner("operator"),
]);

// https://doc.rust-lang.org/stable/reference/items.html
const RUST_ITEM: Type = Type::Any(&[
    Type::Inner("function_definition"),
    Type::Inner("struct"),
    Type::Inner("enum"),
]);

// https://doc.rust-lang.org/stable/reference/types.html#type-expressions
const RUST_TYPE: Type = Type::Any(&[
    Type::String,
    Type::Inner("tuple_type"),
    Type::Inner("reference_type"),
    Type::Inner("array_type"),
    Type::Inner("slice_type"),
    Type::Inner("simple_path"),
]);

// Alternative implementation: distinct structs implementing a parse_from method that only looks at
//the kind field of Inner, and we then try to parse each element with all of them until one
// matches.

pub const RUST_SCHEMA: Schema = Schema {
    kinds: &[
        Kind {
            name: "document",
            fields: &[Field {
                name: "bindings",
                type_: RUST_ITEM,
                multiplicity: Multiplicity::Repeated,
                validator: whatever,
            }],
            inner: None,
            renderer: |model: &Model, value: &Inner, path: &Path| {
                let bindings = model
                    .view_children(&value, "bindings", &path)
                    .into_iter()
                    .map(|b| {
                        html! {
                            <div>{ b }</div>
                        }
                    });
                html! {
                    <div>
                    { for bindings }
                    </div>
                }
            },
        },
        Kind {
            name: "tuple_type",
            fields: &[Field {
                name: "components",
                type_: RUST_TYPE,
                multiplicity: Multiplicity::Repeated,
                validator: whatever,
            }],
            inner: Some("components"),
            renderer: |model: &Model, value: &Inner, path: &Path| todo!(),
        },
        Kind {
            name: "reference_type",
            fields: &[
                Field {
                    name: "type",
                    type_: RUST_TYPE,
                    multiplicity: Multiplicity::Single,
                    validator: whatever,
                },
                Field {
                    name: "mutable",
                    type_: Type::Bool,
                    multiplicity: Multiplicity::Single,
                    validator: whatever,
                },
                Field {
                    name: "lifetime",
                    type_: Type::Star,
                    multiplicity: Multiplicity::Single,
                    validator: whatever,
                },
            ],
            inner: None,
            renderer: |model: &Model, value: &Inner, path: &Path| todo!(),
        },
        Kind {
            name: "constant",
            fields: &[
                Field {
                    name: "name",
                    type_: Type::String,
                    multiplicity: Multiplicity::Single,
                    validator: whatever,
                },
                Field {
                    name: "type",
                    type_: RUST_TYPE,
                    multiplicity: Multiplicity::Single,
                    validator: whatever,
                },
                Field {
                    name: "value",
                    type_: RUST_EXPRESSION,
                    multiplicity: Multiplicity::Single,
                    validator: whatever,
                },
            ],
            inner: Some("statements"),
            renderer: |model: &Model, value: &Inner, path: &Path| todo!(),
        },
        Kind {
            name: "block",
            fields: &[Field {
                name: "statements",
                type_: Type::Star,
                multiplicity: Multiplicity::Repeated,
                validator: whatever,
            }],
            inner: Some("statements"),
            renderer: |model: &Model, value: &Inner, path: &Path| {
                let statements = model
                    .view_children(value, "statements", &path)
                    .into_iter()
                    .enumerate()
                    .map(|(i, v)| {
                        if i == 0 {
                            v
                        } else {
                            html! {
                                <div class="indent">{ v }{ ";" }</div>
                            }
                        }
                    });

                html! {
                    <span>
                    { "{" }{ for statements }{ "}" }
                    </span>
                }
            },
        },
        Kind {
            name: "if",
            fields: &[
                Field {
                    name: "condition", // Expression
                    type_: RUST_EXPRESSION,
                    multiplicity: Multiplicity::Single,
                    validator: whatever,
                },
                Field {
                    name: "true_body", // Expression
                    type_: RUST_EXPRESSION,
                    multiplicity: Multiplicity::Single,
                    validator: whatever,
                },
                Field {
                    name: "false_body", // Expression
                    type_: RUST_EXPRESSION,
                    multiplicity: Multiplicity::Single,
                    validator: whatever,
                },
            ],
            inner: Some("true_body"),
            renderer: |model: &Model, value: &Inner, path: &Path| {
                let condition = model.view_child(value, "condition", &path);
                let true_body = model.view_child(value, "true_body", &path);
                let false_body = model.view_child(value, "false_body", &path);
                html! {
                    <span>
                        <div>
                            <span class="keyword">{ "if" }</span>{ condition }{ "{" }
                        </div>
                        <div class="indent">
                            { true_body }
                        </div>
                        <div>
                            { "}" }<span class="keyword">{ "else" }</span>{ "{" }
                        </div>
                        <div class="indent">
                            { false_body }
                        </div>
                        <div>
                            { "}" }
                        </div>
                    </span>
                }
            },
        },
        Kind {
            name: "string",
            fields: &[Field {
                name: "value",
                type_: Type::String,
                multiplicity: Multiplicity::Single,
                validator: whatever,
            }],
            inner: Some("value"),
            renderer: |model: &Model, value: &Inner, path: &Path| {
                let value = model.view_child(value, "value", &path);
                html! {
                    <span>
                    { "\"" }{ value }{ "\"" }
                    </span>
                }
            },
        },
        Kind {
            name: "field_access",
            fields: &[
                Field {
                    name: "object",
                    type_: RUST_EXPRESSION,
                    multiplicity: Multiplicity::Single,
                    validator: whatever,
                },
                Field {
                    name: "field",
                    type_: Type::String,
                    multiplicity: Multiplicity::Single,
                    validator: whatever,
                },
            ],
            inner: Some("object"),
            renderer: |model: &Model, value: &Inner, path: &Path| {
                let object = model.view_child(value, "object", &path);
                let field = model.view_child(value, "field", &path);
                html! {
                    <span>
                    { object }
                    { "." }
                    { field }
                    </span>
                }
            },
        },
        Kind {
            name: "simple_path",
            fields: &[Field {
                name: "segments",
                type_: Type::Star,
                multiplicity: Multiplicity::Repeated,
                validator: whatever,
            }],
            inner: Some("segments"),
            renderer: |model: &Model, value: &Inner, path: &Path| {
                let segments = model
                    .view_children(value, "segments", &path)
                    .into_iter()
                    .enumerate()
                    .map(|(i, v)| {
                        if i == 0 || i == 1 {
                            v
                        } else {
                            html! {
                                <span>{ "::" }{ v }</span>
                            }
                        }
                    });
                html! {
                    <span>{ for segments }</span>
                }
            },
        },
        Kind {
            name: "crate",
            fields: &[],
            inner: None,
            renderer: |model: &Model, value: &Inner, path: &Path| {
                html! {
                    <span class="keyword">{ "crate" }</span>
                }
            },
        },
        Kind {
            name: "binary_operator",
            fields: &[
                Field {
                    name: "operator",
                    type_: Type::String,
                    multiplicity: Multiplicity::Single,
                    validator: operator,
                },
                Field {
                    name: "left",
                    type_: RUST_EXPRESSION,
                    multiplicity: Multiplicity::Single,
                    validator: whatever,
                },
                Field {
                    name: "right",
                    type_: RUST_EXPRESSION,
                    multiplicity: Multiplicity::Single,
                    validator: whatever,
                },
            ],
            inner: Some("left"),
            renderer: |model: &Model, value: &Inner, path: &Path| {
                let operator = model.view_child(value, "operator", &path);
                let left = model.view_child(value, "left", &path);
                let right = model.view_child(value, "right", &path);
                html! {
                    <span>
                    { left }
                    { operator }
                    { right }
                    </span>
                }
            },
        },
        // https://doc.rust-lang.org/nightly/reference/items/functions.html
        Kind {
            name: "function_definition",
            fields: &[
                /*
                Field {
                    name: "pub",
                    type_: Type::Bool,
                    multiplicity: Multiplicity::Single,
                    validator: whatever,
                },
                Field {
                    name: "async",
                    type_: Type::Bool,
                    multiplicity: Multiplicity::Single,
                    validator: whatever,
                },
                */
                Field {
                    name: "comment",
                    type_: Type::Inner("markdown_document"),
                    multiplicity: Multiplicity::Single,
                    validator: whatever,
                },
                Field {
                    name: "name",
                    type_: Type::String,
                    multiplicity: Multiplicity::Single,
                    validator: identifier,
                },
                Field {
                    name: "arguments", // Pattern
                    type_: Type::Star,
                    multiplicity: Multiplicity::Repeated,
                    validator: whatever,
                },
                Field {
                    name: "return_type", // Type
                    type_: RUST_TYPE,
                    multiplicity: Multiplicity::Single,
                    validator: whatever,
                },
                Field {
                    name: "body", // Expression
                    type_: RUST_EXPRESSION,
                    multiplicity: Multiplicity::Single,
                    validator: whatever,
                },
            ],
            inner: None,
            renderer: |model: &Model, value: &Inner, path: &Path| {
                let comment = model.view_child(&value, "comment", &path);
                let label = model.view_child(&value, "name", &path);
                let args = model
                    .view_children(&value, "arguments", &path)
                    .into_iter()
                    .enumerate()
                    .map(|(i, v)| {
                        if i == 0 || i == 1 {
                            v
                        } else {
                            html! {
                                <span>{ "," }{ v }</span>
                            }
                        }
                    });
                let body = model.view_child(&value, "body", &path);
                let return_type = model.view_child(&value, "return_type", &path);
                // let async_ = self.view_child(&v, "async", &path);
                // let pub_ = self.view_child(&v, "pub", &path);

                html! {
                    <span>
                        <div>{ "//" }{ comment }</div>
                        // { pub_ }
                        <div><span class="keyword">{ "fn" }</span>{ label }{ "(" }{ for args }{ ")" }{ "->" }{ return_type }{ "{" }</div>
                        <div class="indent">{ body }</div>{ "}" }
                    </span>
                }
            },
        },
        Kind {
            name: "pattern",
            fields: &[
                Field {
                    name: "name",
                    type_: Type::String,
                    multiplicity: Multiplicity::Single,
                    validator: identifier,
                },
                Field {
                    name: "type", // Type
                    type_: RUST_TYPE,
                    multiplicity: Multiplicity::Single,
                    validator: whatever,
                },
            ],
            inner: None,
            renderer: |model: &Model, value: &Inner, path: &Path| {
                let name = model.lookup(&value.children["name"][0]).unwrap();
                let name = match &name.value {
                    Value::String(v) => v.clone(),
                    _ => "error".to_string(),
                };
                html! {
                    <span>
                    { name }
                    </span>
                }
            },
        },
        Kind {
            name: "binding",
            fields: &[
                Field {
                    name: "name",
                    type_: Type::String,
                    multiplicity: Multiplicity::Single,
                    validator: identifier,
                },
                Field {
                    name: "type", // Type
                    type_: RUST_TYPE,
                    multiplicity: Multiplicity::Single,
                    validator: whatever,
                },
                Field {
                    name: "value", // Expression
                    type_: RUST_EXPRESSION,
                    multiplicity: Multiplicity::Single,
                    validator: whatever,
                },
            ],
            inner: Some("value"),
            renderer: |model: &Model, value: &Inner, path: &Path| {
                let name = model.view_child(value, "name", &path);
                let value = model.view_child(value, "value", &path);
                html! {
                    <span>{ "let" }{ name }{ "=" }{ value }</span>
                }
            },
        },
        Kind {
            name: "type",
            fields: &[
                Field {
                    name: "name",
                    type_: Type::String,
                    multiplicity: Multiplicity::Single,
                    validator: whatever,
                },
                // Generic type parameters.
                Field {
                    name: "arguments",
                    type_: Type::Star,
                    multiplicity: Multiplicity::Repeated,
                    validator: whatever,
                },
            ],
            inner: None,
            renderer: |model: &Model, value: &Inner, path: &Path| todo!(),
        },
        Kind {
            name: "function_call",
            fields: &[
                Field {
                    name: "function",
                    type_: Type::String,
                    multiplicity: Multiplicity::Single,
                    validator: whatever,
                },
                Field {
                    name: "arguments", // Expression
                    type_: RUST_EXPRESSION,
                    multiplicity: Multiplicity::Repeated,
                    validator: whatever,
                },
            ],
            inner: None,
            renderer: |model: &Model, value: &Inner, path: &Path| {
                let function = model.view_child(value, "function", path);
                // let function_name = self.view_children(&v, "function");
                // let function_name = "xxx";
                // .and_then(|n| n.label())
                // .map(|l| l.name.clone())
                // .unwrap_or("<UNKNOWN>".to_string());
                let args = model
                    .view_children(value, "arguments", path)
                    .into_iter()
                    .enumerate()
                    .map(|(i, v)| {
                        if i == 0 || i == 1 {
                            v
                        } else {
                            html! {
                                <span>{ "," }{ v }</span>
                            }
                        }
                    });
                html! {
                    <span>
                    { function }
                    { "(" }{ for args }{ ")" }
                    </span>
                }
            },
        },
        Kind {
            name: "struct",
            fields: &[
                Field {
                    name: "name",
                    type_: Type::String,
                    multiplicity: Multiplicity::Single,
                    validator: identifier,
                },
                Field {
                    name: "fields",
                    type_: Type::Inner("struct_field"),
                    multiplicity: Multiplicity::Repeated,
                    validator: whatever,
                },
            ],
            inner: None,
            renderer: |model: &Model, value: &Inner, path: &Path| {
                let label = model.view_child(value, "name", &path);
                let fields = model
                    .view_children(value, "fields", &path)
                    .into_iter()
                    .map(|v| {
                        html! {
                            <div class="indent">{ v }{ "," }</div>
                        }
                    });

                html! {
                    <span>
                    <span class="keyword">{ "struct" }</span>{ label }
                    { "{" }{ for fields }{ "}" }
                    </span>
                }
            },
        },
        Kind {
            name: "struct_field",
            fields: &[
                Field {
                    name: "name",
                    type_: Type::String,
                    multiplicity: Multiplicity::Single,
                    validator: identifier,
                },
                Field {
                    name: "type", // Type
                    type_: RUST_TYPE,
                    multiplicity: Multiplicity::Single,
                    validator: whatever,
                },
            ],
            inner: None,
            renderer: |model: &Model, value: &Inner, path: &Path| {
                let name = model.view_child(value, "name", &path);
                let type_ = model.view_child(value, "type", &path);
                html! {
                    <span>
                    { name }{ ":" }{ type_ }
                    </span>
                }
            },
        },
        Kind {
            name: "enum",
            fields: &[
                Field {
                    name: "name",
                    type_: Type::String,
                    multiplicity: Multiplicity::Single,
                    validator: identifier,
                },
                Field {
                    name: "variants",
                    type_: Type::Inner("enum_variant"),
                    multiplicity: Multiplicity::Repeated,
                    validator: whatever,
                },
            ],
            inner: None,
            renderer: |model: &Model, value: &Inner, path: &Path| {
                let label = model.view_child(value, "name", &path);
                let variants = model
                    .view_children(value, "variants", &path)
                    .into_iter()
                    .map(|v| {
                        html! {
                            <div class="indent">{ v }{ "," }</div>
                        }
                    });

                html! {
                    <span>
                    <span class="keyword">{ "enum" }</span>{ label }
                    { "{" }{ for variants }{ "}" }
                    </span>
                }
            },
        },
        Kind {
            name: "markdown_document",
            fields: &[Field {
                name: "items",
                type_: Type::Any(&[
                    Type::String,
                    Type::Inner("markdown_heading"),
                    Type::Inner("markdown_code"),
                    Type::Inner("markdown_quote"),
                    Type::Inner("markdown_list"),
                ]),
                multiplicity: Multiplicity::Repeated,
                validator: whatever,
            }],
            inner: Some("paragraphs"),
            renderer: |model: &Model, value: &Inner, path: &Path| {
                let items = model
                    .view_children(value, "items", &path)
                    .into_iter()
                    .map(|v| {
                        html! {
                            <div>{ v }</div>
                        }
                    });
                html! {
                    <span>
                    { for items }
                    </span>
                }
            },
        },
        Kind {
            name: "markdown_heading",
            fields: &[
                Field {
                    name: "level",
                    type_: Type::Int,
                    multiplicity: Multiplicity::Single,
                    validator: whatever,
                },
                Field {
                    name: "text",
                    type_: Type::String,
                    multiplicity: Multiplicity::Single,
                    validator: whatever,
                },
            ],
            inner: Some("text"),
            renderer: |model: &Model, value: &Inner, path: &Path| {
                let level = model.view_child(value, "level", &path);
                let text = model.view_child(value, "text", &path);
                html! {
                    <span>
                    { "#" }{ level}{ text }
                    </span>
                }
            },
        },
        Kind {
            name: "markdown_list",
            fields: &[Field {
                name: "items",
                type_: Type::Inner("markdown_paragraph"),
                multiplicity: Multiplicity::Repeated,
                validator: whatever,
            }],
            inner: Some("items"),
            renderer: |model: &Model, value: &Inner, path: &Path| {
                let items = model
                    .view_children(value, "items", &path)
                    .into_iter()
                    .map(|v| {
                        html! {
                            <li>{ v }</li>
                        }
                    });
                html! {
                    <span>
                        <ul>
                            { for items }
                        </ul>
                    </span>
                }
            },
        },
    ],
};

type Validator = fn(&Value) -> bool;
type Renderer = fn(&Model, &Inner, &Path) -> Html;

fn whatever(_: &Value) -> bool {
    true
}

fn identifier(v: &Value) -> bool {
    match v {
        Value::String(v) => !v.contains(' '),
        _ => false,
    }
}

fn operator(v: &Value) -> bool {
    match v {
        Value::String(v) => v == "==",
        _ => false,
    }
}

pub struct Schema {
    pub kinds: &'static [Kind],
}

pub struct Kind {
    pub name: &'static str,
    pub fields: &'static [Field],
    pub inner: Option<&'static str>,
    pub renderer: Renderer,
    // pub aliases: &'static [&'static str],
}

pub struct Field {
    pub name: &'static str,
    pub type_: Type,
    pub multiplicity: Multiplicity,
    pub validator: Validator,
}

#[derive(Debug)]
pub enum Type {
    Star,
    Bool,
    String,
    Int,
    Inner(&'static str),
    Any(&'static [Type]), // Choice between other types.
}

impl Type {
    pub fn valid(&self, value: &Value) -> bool {
        match (self, value) {
            (Type::Star, _) => true,
            (Type::Bool, Value::Bool(_)) => true,
            (Type::Int, Value::Int(_)) => true,
            (Type::String, Value::String(_)) => true,
            (Type::Inner(k), Value::Inner(v)) => k == &v.kind,
            (Type::Any(k), _) => k.iter().any(|t| t.valid(value)),
            _ => false,
        }
    }
}

pub enum Multiplicity {
    // Required -- show hole if not present
    // Optional -- hide if not present
    Single,
    Repeated,
}
