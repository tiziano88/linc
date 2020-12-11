use itertools::Itertools;
use maplit::hashmap;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::collections::VecDeque;
use yew::prelude::*;
use yew::services::storage::Area;
use yew::services::StorageService;

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

    fn parse_command(&self, command: &str) -> Option<Value> {
        let mut value = match command {
            "if" => Some(Value::Inner(Inner {
                kind: "if".to_string(),
                children: HashMap::new(),
            })),
            "enum" => Some(Value::Inner(Inner {
                kind: "enum".to_string(),
                children: HashMap::new(),
            })),
            "variant" => Some(Value::Inner(Inner {
                kind: "enum_variant".to_string(),
                children: HashMap::new(),
            })),
            "struct" => Some(Value::Inner(Inner {
                kind: "struct".to_string(),
                children: HashMap::new(),
            })),
            "field" => Some(Value::Inner(Inner {
                kind: "struct_field".to_string(),
                children: HashMap::new(),
            })),
            "string" => Some(Value::Inner(Inner {
                kind: "string".to_string(),
                children: HashMap::new(),
            })),
            "fn" => Some(Value::Inner(Inner {
                kind: "function_definition".to_string(),
                children: HashMap::new(),
            })),
            "==" => Some(Value::Inner(Inner {
                kind: "binary_operator".to_string(),
                children: HashMap::new(),
            })),
            "+" => Some(Value::Inner(Inner {
                kind: "binary_operator".to_string(),
                children: HashMap::new(),
            })),
            "-" => Some(Value::Inner(Inner {
                kind: "binary_operator".to_string(),
                children: HashMap::new(),
            })),
            "let" => Some(Value::Inner(Inner {
                kind: "binding".to_string(),
                children: HashMap::new(),
            })),
            "::" => Some(Value::Inner(Inner {
                kind: "qualify".to_string(),
                children: HashMap::new(),
            })),
            "." => Some(Value::Inner(Inner {
                kind: "field_access".to_string(),
                children: HashMap::new(),
            })),
            "(" => Some(Value::Inner(Inner {
                kind: "function_call".to_string(),
                children: HashMap::new(),
            })),
            "{" => Some(Value::Inner(Inner {
                kind: "block".to_string(),
                children: HashMap::new(),
            })),
            "[" => None,
            "," => None,
            "false" => Some(Value::Bool(false)),
            "true" => Some(Value::Bool(true)),
            _ => {
                if let Some(v) = command.strip_prefix('"') {
                    Some(Value::String(v.to_string()))
                } else if let Ok(v) = command.parse::<i32>() {
                    Some(Value::Int(v))
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
                    // Cursor is pointing to a field but not a specific child, create the first child.
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

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Inner {
    pub kind: String,
    pub children: HashMap<String, Vec<Ref>>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct BindingValue {
    pub label: Label,
    pub value: Ref,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PatternValue {
    pub label: Label,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct BlockValue {
    pub expressions: Vec<Ref>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ListValue {
    pub items: Vec<Ref>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct IfValue {
    pub conditional: Ref,
    pub true_body: Ref,
    pub false_body: Ref,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct FunctionDefinitionValue {
    pub label: Label,
    pub arguments: Vec<Ref>,
    pub return_type: Ref,
    pub outer_attributes: Vec<Ref>,
    pub inner_attributes: Vec<Ref>,
    pub body: Ref,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct FunctionCallValue {
    pub function: Ref,
    pub arguments: Vec<Ref>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct BinaryOperatorValue {
    pub operator: String,
    pub left: Ref,
    pub right: Ref,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Pattern {
    pub label: Label,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Label {
    pub name: String,
    pub colour: String,
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
        html! {
            <div>
                <div>{ "LINC" }</div>
                <div>{ self.view_actions() }</div>
                <div class="wrapper">
                    <div class="column">{ self.view_file(&self.file) }</div>
                    <div class="column">
                        <div>{ display_cursor(&self.cursor) }</div>
                        <div>{ format!("Ref: {:?}", self.lookup_path(&self.file.root, self.cursor.clone())) }</div>
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
                            self.set_value(value);
                            self.parsed_command = None;
                            self.command = "".to_string();
                            self.next();
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

pub const MARKDOWN_SCHEMA: Schema = Schema {
    kinds: &[
        Kind {
            name: "document",
            fields: &[Field {
                name: "paragraphs",
                type_: Type::Any,
                multiplicity: Multiplicity::Repeated,
                validator: whatever,
            }],
            inner: Some("paragraphs"),
        },
        Kind {
            name: "paragraph",
            fields: &[Field {
                name: "text",
                type_: Type::String,
                multiplicity: Multiplicity::Single,
                validator: whatever,
            }],
            inner: Some("text"),
        },
        Kind {
            name: "list",
            fields: &[Field {
                name: "items",
                type_: Type::Any,
                multiplicity: Multiplicity::Repeated,
                validator: whatever,
            }],
            inner: Some("items"),
        },
    ],
};

// https://doc.rust-lang.org/stable/reference/expressions.html
const RUST_EXPRESSION: Type = Type::Alt(&[
    Type::Inner("if"),
    Type::Inner("string"),
    Type::Inner("field_access"),
    Type::Inner("operator"),
]);

// https://doc.rust-lang.org/stable/reference/items.html
const RUST_ITEM: Type = Type::Alt(&[
    Type::Inner("function_definition"),
    Type::Inner("struct"),
    Type::Inner("enum"),
]);

// https://doc.rust-lang.org/stable/reference/types.html#type-expressions
const RUST_TYPE: Type = Type::Alt(&[
    Type::Inner("tuple_type"),
    Type::Inner("reference_type"),
    Type::Inner("array_type"),
    Type::Inner("slice_type"),
]);

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
                    type_: Type::Any,
                    multiplicity: Multiplicity::Single,
                    validator: whatever,
                },
            ],
            inner: None,
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
        },
        Kind {
            name: "block",
            fields: &[Field {
                name: "statements",
                type_: Type::Any,
                multiplicity: Multiplicity::Repeated,
                validator: whatever,
            }],
            inner: Some("statements"),
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
        },
        Kind {
            name: "qualify",
            fields: &[
                Field {
                    name: "parent",
                    type_: Type::Any,
                    multiplicity: Multiplicity::Single,
                    validator: whatever,
                },
                Field {
                    name: "child",
                    type_: Type::String,
                    multiplicity: Multiplicity::Single,
                    validator: whatever,
                },
            ],
            inner: Some("parent"),
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
                    name: "name",
                    type_: Type::String,
                    multiplicity: Multiplicity::Single,
                    validator: identifier,
                },
                Field {
                    name: "arguments", // Pattern
                    type_: Type::Any,
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
                    type_: Type::Any,
                    multiplicity: Multiplicity::Single,
                    validator: whatever,
                },
            ],
            inner: None,
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
                    type_: Type::Any,
                    multiplicity: Multiplicity::Repeated,
                    validator: whatever,
                },
            ],
            inner: None,
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
                    type_: Type::Inner("type"),
                    multiplicity: Multiplicity::Single,
                    validator: whatever,
                },
            ],
            inner: None,
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
        },
    ],
};

type Validator = fn(&Value) -> bool;

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
}

pub struct Field {
    pub name: &'static str,
    pub type_: Type,
    pub multiplicity: Multiplicity,
    pub validator: Validator,
}

pub enum Type {
    Any,
    Bool,
    String,
    Inner(&'static str),
    Alt(&'static [Type]), // Choice between other types.
}

pub enum Multiplicity {
    // Required -- show hole if not present
    // Optional -- hide if not present
    Single,
    Repeated,
}
