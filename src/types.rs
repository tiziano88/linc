use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::collections::VecDeque;
use yew::prelude::*;
use yew::services::storage::Area;
use yew::services::StorageService;

pub type Ref = String;

pub fn invalid_ref() -> Ref {
    "".to_string()
}

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
    pub text: String,
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
                        let children = &v.children[&head.field];
                        let r = children[head.index.unwrap()].clone();
                        self.lookup_path(&r, path)
                    }
                    _ => None,
                }
            }
            None => Some(reference.clone()),
        }

        /*
        let i = match head {
            Some(Selector::Index(i)) => i,
            _ => return None,
        };
        let mut reference = self.file.bindings[i].clone();
        let mut node = self.lookup(&reference)?;
        while let Some(selector) = path.pop_front() {
            node = self.lookup(&reference)?;
            match self.lookup(&reference) {
                Some(node) => match node.child(selector) {
                    Some(Child::Single(r)) => reference = r.clone(),
                    Some(Child::Multiple(rs)) => match path.pop_front() {
                        Some(Selector::Index(i)) => reference = rs[i].clone(),
                        _ => return None,
                    },
                    None => return None,
                },
                None => return None,
            }
        }
        node = self.lookup(&reference)?;
        Some(&node)
        */
    }
    // pub fn selected_node(&self) -> Option<&Node> {
    //     self.path
    //         .back()
    //         .and_then(|reference| self.lookup(reference))
    // }
    // pub fn current(&self) -> Option<Ref> {
    //     self.path.back().cloned()
    // }

    pub fn parent_ref(&self) -> Option<Ref> {
        let mut parent_cursor = self.cursor.clone();
        parent_cursor.pop_back().unwrap();
        self.lookup_path(&self.file.root, parent_cursor)
    }
}

#[derive(Clone)]
pub enum Msg {
    Select(Path),
    Rename(Ref, String),

    Store,
    Load,

    Prev,
    Next,
    Parent,

    AddArgument,
    AddItem,
    AddExpression,
    NewFn,

    SetValue(Value),

    SetText(String),
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

pub enum Child {
    Single(Ref),
    Multiple(Vec<Ref>),
}

pub enum FieldType {
    Single,
    Repeated,
}

impl Node {
    // pub fn label(&self) -> Option<&Label> {
    //     match &self.value {
    //         Value::Binding(ref v) => Some(&v.label),
    //         Value::Pattern(ref v) => Some(&v.label),
    //         Value::FunctionDefinition(ref v) => Some(&v.label),
    //         _ => None,
    //     }
    // }

    // pub fn rename(&mut self, name: String) {
    //     match &mut self.value {
    //         Value::Binding(ref mut v) => v.label.name = name,
    //         Value::Pattern(ref mut v) => v.label.name = name,
    //         Value::FunctionDefinition(ref mut v) => v.label.name = name,
    //         _ => {}
    //     }
    // }

    pub fn map_ref<F>(&mut self, mut f: F)
    where
        F: FnMut(&Ref) -> Ref,
    {
        match &mut self.value {
            /*
            Value::Block(ref mut v) => {
                v.expressions = v.expressions.iter().map(f).collect();
            }
            Value::BinaryOperator(ref mut v) => {
                v.left = f(&v.left);
                v.right = f(&v.right);
            }
            Value::FunctionCall(ref mut v) => {
                v.function = f(&v.function);
                v.arguments = v.arguments.iter().map(f).collect();
            }
            Value::FunctionDefinition(ref mut v) => {
                v.body = f(&v.body);
                v.arguments = v.arguments.iter().map(f).collect();
            }
            */
            _ => {}
        }
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("could not serialize to JSON")
    }

    pub fn child(&self, selector: Selector) -> Option<Child> {
        child(&self.value, selector)
    }

    pub fn first(&self) -> Option<Selector> {
        match &self.value {
            /*
            Value::FunctionDefinition(v) => Some(Selector::Field("args".to_string())),
            Value::FunctionCall(v) => Some(Selector::Field("args".to_string())),
            Value::BinaryOperator(v) => Some(Selector::Field("left".to_string())),
            */
            _ => None,
        }
    }

    pub fn next(&self, selector: Selector) -> Option<Selector> {
        match &self.value {
            /*
            Value::FunctionDefinition(v) => match &selector {
                Selector::Field(f) if f == "args" => {
                    Some(Selector::Field("return_type".to_string()))
                }
                Selector::Field(f) if f == "return_type" => {
                    Some(Selector::Field("body".to_string()))
                }
                Selector::Field(f) if f == "body" => None,
                _ => None,
            },
            Value::FunctionCall(v) => match &selector {
                Selector::Field(f) if f == "args" => None,
                _ => None,
            },
            Value::BinaryOperator(v) => match &selector {
                Selector::Field(f) if f == "left" => Some(Selector::Field("right".to_string())),
                Selector::Field(f) if f == "right" => None,
                _ => None,
            },
            */
            _ => None,
        }
    }
}

pub fn child(value: &Value, selector: Selector) -> Option<Child> {
    match &value {
        /*
        Value::FunctionDefinition(v) => match &selector {
            Selector::Field(f) if f == "args" => Some(Child::Multiple(v.arguments.clone())),
            Selector::Field(f) if f == "body" => Some(Child::Single(v.body.clone())),
            Selector::Field(f) if f == "return_type" => Some(Child::Single(v.return_type.clone())),
            _ => None,
        },
        Value::FunctionCall(v) => match &selector {
            Selector::Field(f) if f == "args" => Some(Child::Multiple(v.arguments.clone())),
            _ => None,
        },
        Value::BinaryOperator(v) => match &selector {
            Selector::Field(f) if f == "left" => Some(Child::Single(v.left.clone())),
            Selector::Field(f) if f == "right" => Some(Child::Single(v.right.clone())),
            _ => None,
        },
        */
        _ => None,
    }
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
                    <div class="column">{ self.view_file_json(&self.file) }</div>
                    <div class="column">
                        <div>{ format!("Cursor: {:?}", self.cursor) }</div>
                    </div>
               </div>
            </div>
        }
    }

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Model {
            store: StorageService::new(Area::Local).expect("could not create storage service"),
            text: "".to_string(),
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
            Msg::Rename(reference, name) => {
                // if let Some(node) = self.lookup_mut(&reference) {
                //     node.rename(name);
                // }
            }

            // TODO: sibling vs inner
            Msg::Prev => {
                let flattened_paths = self.flatten_paths(&self.file.root, Path::new());
                log::info!("paths: {:?}", flattened_paths);
                let current_path_index = flattened_paths.iter().position(|x| *x == self.cursor);
                log::info!("current: {:?}", current_path_index);
                self.cursor = flattened_paths[current_path_index.unwrap() - 1].clone();
            }
            // Preorder tree traversal.
            Msg::Next => {
                let flattened_paths = self.flatten_paths(&self.file.root, Path::new());
                log::info!("paths: {:?}", flattened_paths);
                let current_path_index = flattened_paths.iter().position(|x| *x == self.cursor);
                log::info!("current: {:?}", current_path_index);
                self.cursor = flattened_paths[current_path_index.unwrap() + 1].clone();
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

            Msg::AddArgument => {
                // let reference = self.file.add_node(Value::Pattern(PatternValue {
                //     label: Label {
                //         name: "xxx".to_string(),
                //         colour: "red".to_string(),
                //     },
                // }));
                // if let Some(node) = self
                //     .current()
                //     .and_then(|reference| self.lookup_mut(&reference))
                // {
                //     if let Value::FunctionDefinition(ref mut v) = node.value {
                //         v.arguments.push(reference);
                //     }
                // }
            }
            Msg::AddItem => {
                // let reference = self.file.add_node(Value::Hole);
                // if let Some(node) = self
                //     .current()
                //     .and_then(|reference| self.lookup_mut(&reference))
                // {
                //     if let Value::List(ref mut v) = node.value {
                //         v.items.push(reference);
                //     }
                // }
            }
            Msg::AddExpression => {
                // let reference = self.file.add_node(Value::Hole);
                // if let Some(node) = self
                //     .current()
                //     .and_then(|reference| self.lookup_mut(&reference))
                // {
                //     if let Value::Block(ref mut v) = node.value {
                //         v.expressions.push(reference);
                //     }
                // }
            }
            Msg::NewFn => {
                // let reference =
                //     self.file
                //         .add_node(Value::FunctionDefinition(FunctionDefinitionValue {
                //             label: Label {
                //                 name: "xxx".to_string(),
                //                 colour: "red".to_string(),
                //             },
                //             arguments: vec![],
                //             outer_attributes: vec![],
                //             inner_attributes: vec![],
                //             return_type: invalid_ref(),
                //             body: "11111".to_string(),
                //         }));
                // self.file.bindings.push(reference);
            }
            Msg::SetText(v) => {
                self.text = v;
            }
            Msg::SetValue(v) => {
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

                // if let Some(node) = self
                //     .parent()
                //     .and_then(|reference| self.lookup_mut(&reference))
                // {
                //     node.map_ref(|r| {
                //         if *r == current {
                //             reference.clone()
                //         } else {
                //             r.to_string()
                //         }
                //     });
                // }
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
                type_: Type::Ref,
                multiplicity: Multiplicity::Repeated,
                validator: whatever,
            }],
        },
        Kind {
            name: "paragraph",
            fields: &[Field {
                name: "text",
                type_: Type::String,
                multiplicity: Multiplicity::Single,
                validator: whatever,
            }],
        },
        Kind {
            name: "list",
            fields: &[Field {
                name: "items",
                type_: Type::Ref,
                multiplicity: Multiplicity::Repeated,
                validator: whatever,
            }],
        },
    ],
};

pub const RUST_SCHEMA: Schema = Schema {
    kinds: &[
        Kind {
            name: "document",
            fields: &[Field {
                name: "bindings",
                type_: Type::Ref,
                multiplicity: Multiplicity::Repeated,
                validator: whatever,
            }],
        },
        Kind {
            name: "block",
            fields: &[Field {
                name: "statements",
                type_: Type::Ref,
                multiplicity: Multiplicity::Repeated,
                validator: whatever,
            }],
        },
        Kind {
            name: "if",
            fields: &[
                Field {
                    name: "condition", // Expression
                    type_: Type::Ref,
                    multiplicity: Multiplicity::Single,
                    validator: whatever,
                },
                Field {
                    name: "true_body", // Expression
                    type_: Type::Ref,
                    multiplicity: Multiplicity::Single,
                    validator: whatever,
                },
                Field {
                    name: "false_body", // Expression
                    type_: Type::Ref,
                    multiplicity: Multiplicity::Single,
                    validator: whatever,
                },
            ],
        },
        Kind {
            name: "binary_operator",
            fields: &[
                Field {
                    name: "left",
                    type_: Type::Ref,
                    multiplicity: Multiplicity::Single,
                    validator: whatever,
                },
                Field {
                    name: "right",
                    type_: Type::Ref,
                    multiplicity: Multiplicity::Single,
                    validator: whatever,
                },
            ],
        },
        Kind {
            name: "function_definition",
            fields: &[
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
                Field {
                    name: "name",
                    type_: Type::String,
                    multiplicity: Multiplicity::Single,
                    validator: whatever,
                },
                Field {
                    name: "arguments", // Pattern
                    type_: Type::Ref,
                    multiplicity: Multiplicity::Repeated,
                    validator: whatever,
                },
                Field {
                    name: "return_type", // Type
                    type_: Type::Ref,
                    multiplicity: Multiplicity::Single,
                    validator: whatever,
                },
                Field {
                    name: "body", // Expression
                    type_: Type::Ref,
                    multiplicity: Multiplicity::Single,
                    validator: whatever,
                },
            ],
        },
        Kind {
            name: "pattern",
            fields: &[
                Field {
                    name: "name",
                    type_: Type::String,
                    multiplicity: Multiplicity::Single,
                    validator: whatever,
                },
                Field {
                    name: "type", // Type
                    type_: Type::Ref,
                    multiplicity: Multiplicity::Single,
                    validator: whatever,
                },
            ],
        },
        Kind {
            name: "let",
            fields: &[
                Field {
                    name: "name",
                    type_: Type::String,
                    multiplicity: Multiplicity::Single,
                    validator: whatever,
                },
                Field {
                    name: "type", // Type
                    type_: Type::Ref,
                    multiplicity: Multiplicity::Single,
                    validator: whatever,
                },
                Field {
                    name: "value", // Expression
                    type_: Type::Ref,
                    multiplicity: Multiplicity::Single,
                    validator: whatever,
                },
            ],
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
                    type_: Type::Ref,
                    multiplicity: Multiplicity::Repeated,
                    validator: whatever,
                },
            ],
        },
        Kind {
            name: "function_call",
            fields: &[Field {
                name: "arguments", // Expression
                type_: Type::Ref,
                multiplicity: Multiplicity::Repeated,
                validator: whatever,
            }],
        },
        Kind {
            name: "struct",
            fields: &[
                Field {
                    name: "name",
                    type_: Type::String,
                    multiplicity: Multiplicity::Single,
                    validator: whatever,
                },
                Field {
                    name: "fields", // Pattern
                    type_: Type::Ref,
                    multiplicity: Multiplicity::Repeated,
                    validator: whatever,
                },
            ],
        },
        Kind {
            name: "enum",
            fields: &[
                Field {
                    name: "name",
                    type_: Type::String,
                    multiplicity: Multiplicity::Single,
                    validator: whatever,
                },
                Field {
                    name: "variants",
                    type_: Type::Ref,
                    multiplicity: Multiplicity::Repeated,
                    validator: whatever,
                },
            ],
        },
    ],
};

type Validator = fn(Value) -> bool;

fn whatever(_: Value) -> bool {
    true
}

pub struct Schema {
    pub kinds: &'static [Kind],
}

pub struct Kind {
    pub name: &'static str,
    pub fields: &'static [Field],
}

pub struct Field {
    pub name: &'static str,
    pub type_: Type,
    pub multiplicity: Multiplicity,
    pub validator: Validator,
}

pub enum Type {
    Bool,
    String,
    Ref,
}

pub enum Multiplicity {
    Single,
    Repeated,
}
