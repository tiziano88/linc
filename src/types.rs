use maplit::hashmap;
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
}

pub fn sub_cursor(cursor: &Option<Path>, selector: Selector) -> Option<Path> {
    let cursor = cursor.clone();
    if Some(selector) == cursor.clone().and_then(|v| v.clone().front().cloned()) {
        cursor.map(|mut v| v.split_off(1))
    } else {
        None
    }
}

impl Model {
    pub fn lookup(&self, reference: &Ref) -> Option<&Node> {
        self.file.lookup(reference)
    }
    pub fn lookup_mut(&mut self, reference: &Ref) -> Option<&mut Node> {
        self.file.lookup_mut(reference)
    }
    pub fn lookup_path(&self, path: Path) -> Option<&Node> {
        /*
        let mut path = path;
        let head = path.pop_front();
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
        None
    }
    // pub fn selected_node(&self) -> Option<&Node> {
    //     self.path
    //         .back()
    //         .and_then(|reference| self.lookup(reference))
    // }
    // pub fn current(&self) -> Option<Ref> {
    //     self.path.back().cloned()
    // }

    // pub fn parent(&self) -> Option<Ref> {
    //     let i = self.path.len() - 2;
    //     self.path.get(i).cloned()
    // }
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
    // Ref(Ref),
    // Binding(BindingValue),
    // Pattern(PatternValue),

    // Block(BlockValue),
    // List(ListValue),
    // If(IfValue),
    // FunctionDefinition(FunctionDefinitionValue),
    // FunctionCall(FunctionCallValue),
    // BinaryOperator(BinaryOperatorValue),
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
            file: File {
                nodes: vec![
                    Node {
                        reference: "101010".to_string(),
                        value: Value::Inner(Inner {
                            kind: "document".to_string(),
                            children: hashmap! {
                                "bindings".to_string() => vec!["111".to_string(), "12".to_string()],
                            },
                        }),
                    },
                    Node {
                        reference: "111".to_string(),
                        value: Value::Inner(Inner {
                            kind: "function_definition".to_string(),
                            children: hashmap! {
                                "name".to_string() => vec!["125".to_string()],
                                "arguments".to_string() => vec![],
                                "outer_attributes".to_string() => vec![],
                                "inner_attributes".to_string() => vec![],
                                "return_type".to_string() => vec![],
                                "body".to_string() => vec!["123".to_string()],
                            },
                        }),
                        // value: Value::FunctionDefinition(FunctionDefinitionValue {
                        //     label: Label {
                        //         name: "main".to_string(),
                        //         colour: "red".to_string(),
                        //     },
                        //     arguments: vec![],
                        //     outer_attributes: vec![],
                        //     inner_attributes: vec![],
                        //     return_type: invalid_ref(),
                        //     body: "123".to_string(),
                        // }),
                    },
                    Node {
                        reference: "124".to_string(),
                        value: Value::Int(123),
                    },
                    Node {
                        reference: "125".to_string(),
                        value: Value::String("main".to_string()),
                    },
                    Node {
                        reference: "12".to_string(),
                        value: Value::Inner(Inner {
                            kind: "function_definition".to_string(),
                            children: hashmap! {
                                "name".to_string() => vec!["126".to_string()],
                                "arguments".to_string() => vec!["222".to_string()],
                                "outer_attributes".to_string() => vec![],
                                "inner_attributes".to_string() => vec![],
                                "return_type".to_string() => vec![],
                                "body".to_string() => vec!["228".to_string()],
                            },
                        }),
                        // value: Value::FunctionDefinition(FunctionDefinitionValue {
                        //     label: Label {
                        //         name: "factorial".to_string(),
                        //         colour: "red".to_string(),
                        //     },
                        //     arguments: vec!["222".to_string()],
                        //     outer_attributes: vec![],
                        //     inner_attributes: vec![],
                        //     return_type: invalid_ref(),
                        //     body: "228".to_string(),
                        // }),
                    },
                    Node {
                        reference: "126".to_string(),
                        value: Value::String("factorial".to_string()),
                    },
                    Node {
                        reference: "222".to_string(),
                        value: Value::Inner(Inner {
                            kind: "pattern".to_string(),
                            children: hashmap! {
                                "name".to_string() => vec!["2223".to_string()],
                            },
                        }),
                    },
                    Node {
                        reference: "2223".to_string(),
                        value: Value::String("x".to_string()),
                    },
                    Node {
                        reference: "228".to_string(),
                        value: Value::Inner(Inner {
                            kind: "binary_operator".to_string(),
                            children: hashmap! {
                                "operator".to_string() => vec![],
                                "left".to_string() => vec!["1231".to_string()],
                                "right".to_string() => vec!["1232".to_string()]
                            },
                        }),
                    },
                    Node {
                        reference: "1231".to_string(),
                        value: Value::Inner(Inner {
                            kind: "ref".to_string(),
                            children: hashmap! {
                                "target".to_string() => vec!["222".to_string()],
                            },
                        }),
                    },
                    Node {
                        reference: "1232".to_string(),
                        value: Value::Inner(Inner {
                            kind: "function_call".to_string(),
                            children: hashmap! {
                                "function".to_string() => vec!["12".to_string()],
                                "arguments".to_string() => vec!["229".to_string()]
                            },
                        }),
                    },
                    Node {
                        reference: "229".to_string(),
                        value: Value::Inner(Inner {
                            kind: "binary_operator".to_string(),
                            // TODO: -
                            children: hashmap! {
                                "operator".to_string() => vec![],
                                "left".to_string() => vec!["230".to_string()],
                                "right".to_string() => vec!["231".to_string()]
                            },
                        }),
                    },
                    Node {
                        reference: "230".to_string(),
                        value: Value::Inner(Inner {
                            kind: "ref".to_string(),
                            children: hashmap! {
                                "target".to_string() => vec!["222".to_string()],
                            },
                        }),
                    },
                    Node {
                        reference: "231".to_string(),
                        value: Value::Int(1),
                    },
                ],
                root: "101010".to_string(),
            },
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

            Msg::Prev => {}
            Msg::Next => {
                // let current = self.lookup_path(self.cursor.clone()).expect("no current");
                // match current.first() {
                //     Some(next) => {
                //         let mut new = self.cursor.clone();
                //         new.push_back(next);
                //         self.cursor = new;
                //     }
                //     None => {
                //         let mut parent = self.cursor.clone();
                //         let last = parent.pop_back().expect("no last");
                //         log::info!("last: {:?}", last);
                //         log::info!("parent: {:?}", parent);
                //         let parent_node = self.lookup_path(parent.clone()).expect("no parent");
                //         log::info!("parent: {:?}", parent_node);
                //         match parent_node.next(last) {
                //             Some(next) => {
                //                 log::info!("next: {:?}", next);
                //                 let mut new = parent.clone();
                //                 new.push_back(next);
                //                 self.cursor = new;
                //             }
                //             None => {}
                //         }
                //     }
                // }
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
            Msg::SetValue(v) => {
                // let reference = self.file.add_node(v);
                // let current = self.current().unwrap_or(invalid_ref());
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
