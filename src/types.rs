use serde::{Deserialize, Serialize};
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
pub enum Selector {
    Field(String),
    Index(usize),
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

impl Model {
    pub fn lookup(&self, reference: &Ref) -> Option<&Node> {
        self.file.lookup(reference)
    }
    pub fn lookup_mut(&mut self, reference: &Ref) -> Option<&mut Node> {
        self.file.lookup_mut(reference)
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

    AddArgument,
    AddItem,
    AddExpression,
    NewFn,

    SetValue(Value),
}

#[derive(Serialize, Deserialize)]
pub struct File {
    pub bindings: Vec<Ref>,
    pub nodes: Vec<Node>,
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

#[derive(Serialize, Deserialize)]
pub struct Node {
    pub reference: Ref,
    pub value: Value,
}

fn invalid_node() -> Node {
    Node {
        reference: "".to_string(),
        value: Value::Hole,
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub enum Value {
    Hole,

    Bool(bool),
    Int(i32),
    Float(f32),
    String(String),

    Ref(Ref),
    Binding(BindingValue),
    Pattern(PatternValue),

    Block(BlockValue),
    List(ListValue),
    If(IfValue),
    FunctionDefinition(FunctionDefinitionValue),
    FunctionCall(FunctionCallValue),
    BinaryOperator(BinaryOperatorValue),
}

impl Node {
    pub fn label(&self) -> Option<&Label> {
        match &self.value {
            Value::Binding(ref v) => Some(&v.label),
            Value::Pattern(ref v) => Some(&v.label),
            Value::FunctionDefinition(ref v) => Some(&v.label),
            _ => None,
        }
    }

    pub fn rename(&mut self, name: String) {
        match &mut self.value {
            Value::Binding(ref mut v) => v.label.name = name,
            Value::Pattern(ref mut v) => v.label.name = name,
            Value::FunctionDefinition(ref mut v) => v.label.name = name,
            _ => {}
        }
    }

    pub fn map_ref<F>(&mut self, mut f: F)
    where
        F: FnMut(&Ref) -> Ref,
    {
        match &mut self.value {
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
            _ => {}
        }
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("could not serialize to JSON")
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct BindingValue {
    pub label: Label,
    pub value: Ref,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct PatternValue {
    pub label: Label,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct BlockValue {
    pub expressions: Vec<Ref>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ListValue {
    pub items: Vec<Ref>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct IfValue {
    pub conditional: Ref,
    pub true_body: Ref,
    pub false_body: Ref,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct FunctionDefinitionValue {
    pub label: Label,
    pub arguments: Vec<Ref>,
    pub return_type: Ref,
    pub outer_attributes: Vec<Ref>,
    pub inner_attributes: Vec<Ref>,
    pub body: Ref,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct FunctionCallValue {
    pub function: Ref,
    pub arguments: Vec<Ref>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct BinaryOperatorValue {
    pub operator: String,
    pub left: Ref,
    pub right: Ref,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Pattern {
    pub label: Label,
}

#[derive(Serialize, Deserialize, Clone)]
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
                    <div class="column">{ self.view_file(&self.file, self.cursor) }</div>
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
                        reference: "111".to_string(),
                        value: Value::FunctionDefinition(FunctionDefinitionValue {
                            label: Label {
                                name: "main".to_string(),
                                colour: "red".to_string(),
                            },
                            arguments: vec![],
                            outer_attributes: vec![],
                            inner_attributes: vec![],
                            return_type: invalid_ref(),
                            body: "123".to_string(),
                        }),
                    },
                    Node {
                        reference: "124".to_string(),
                        value: Value::Int(123),
                    },
                    Node {
                        reference: "12".to_string(),
                        value: Value::FunctionDefinition(FunctionDefinitionValue {
                            label: Label {
                                name: "factorial".to_string(),
                                colour: "red".to_string(),
                            },
                            arguments: vec!["222".to_string()],
                            outer_attributes: vec![],
                            inner_attributes: vec![],
                            return_type: invalid_ref(),
                            body: "228".to_string(),
                        }),
                    },
                    Node {
                        reference: "222".to_string(),
                        value: Value::Pattern(PatternValue {
                            label: Label {
                                name: "x".to_string(),
                                colour: "".to_string(),
                            },
                        }),
                    },
                    Node {
                        reference: "228".to_string(),
                        value: Value::BinaryOperator(BinaryOperatorValue {
                            operator: "*".to_string(),
                            left: "1231".to_string(),
                            right: "1232".to_string(),
                        }),
                    },
                    Node {
                        reference: "1231".to_string(),
                        value: Value::Ref("222".to_string()),
                    },
                    Node {
                        reference: "1232".to_string(),
                        value: Value::FunctionCall(FunctionCallValue {
                            function: "12".to_string(),
                            arguments: vec!["229".to_string()],
                        }),
                    },
                    Node {
                        reference: "229".to_string(),
                        value: Value::BinaryOperator(BinaryOperatorValue {
                            operator: "-".to_string(),
                            left: "230".to_string(),
                            right: "231".to_string(),
                        }),
                    },
                    Node {
                        reference: "230".to_string(),
                        value: Value::Ref("222".to_string()),
                    },
                    Node {
                        reference: "231".to_string(),
                        value: Value::Int(1),
                    },
                ],
                bindings: vec!["111".to_string(), "12".to_string()],
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
                if let Some(node) = self.lookup_mut(&reference) {
                    node.rename(name);
                }
            }

            Msg::Prev => {}
            Msg::Next => {}
            // Parent
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
                let reference =
                    self.file
                        .add_node(Value::FunctionDefinition(FunctionDefinitionValue {
                            label: Label {
                                name: "xxx".to_string(),
                                colour: "red".to_string(),
                            },
                            arguments: vec![],
                            outer_attributes: vec![],
                            inner_attributes: vec![],
                            return_type: invalid_ref(),
                            body: "11111".to_string(),
                        }));
                self.file.bindings.push(reference);
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
