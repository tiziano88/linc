#![recursion_limit = "256"]

use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use yew::services::storage::{Area, StorageService};
use yew::{html, Component, ComponentLink, Html, Renderable, ShouldRender};

type Ref = String;

fn invalid_ref() -> Ref {
    "".to_string()
}

fn new_ref() -> Ref {
    uuid::Uuid::new_v4().to_hyphenated().to_string()
}

type Path = VecDeque<Ref>;

pub struct Model {
    file: File,
    store: StorageService,
    path: Path,
}

impl Model {
    fn lookup(&self, reference: &Ref) -> Option<&Node> {
        self.file.lookup(reference)
    }
    fn lookup_mut(&mut self, reference: &Ref) -> Option<&mut Node> {
        self.file.lookup_mut(reference)
    }
    fn selected_node(&self) -> Option<&Node> {
        self.path
            .back()
            .and_then(|reference| self.lookup(reference))
    }
}

#[derive(Clone)]
pub enum Msg {
    Select(Path),
    Rename(Ref, String),

    Store,
    Load,

    AddArgument,
    AddItem,
    AddExpression,
    NewFn,

    SetValue(Value),
}

#[derive(Serialize, Deserialize)]
struct File {
    bindings: Vec<Ref>,
    nodes: Vec<Node>,
}

impl File {
    fn lookup(&self, reference: &Ref) -> Option<&Node> {
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
struct Node {
    reference: Ref,
    value: Value,
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
    fn label(&self) -> Option<&Label> {
        match &self.value {
            Value::Binding(ref v) => Some(&v.label),
            Value::Pattern(ref v) => Some(&v.label),
            Value::FunctionDefinition(ref v) => Some(&v.label),
            _ => None,
        }
    }

    fn rename(&mut self, name: String) {
        match &mut self.value {
            Value::Binding(ref mut v) => v.label.name = name,
            Value::Pattern(ref mut v) => v.label.name = name,
            Value::FunctionDefinition(ref mut v) => v.label.name = name,
            _ => {}
        }
    }

    fn map_ref<F>(&mut self, mut f: F)
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

    fn to_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("could not serialize to JSON")
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct BindingValue {
    label: Label,
    value: Ref,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct PatternValue {
    label: Label,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct BlockValue {
    expressions: Vec<Ref>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ListValue {
    items: Vec<Ref>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct IfValue {
    conditional: Ref,
    true_body: Ref,
    false_body: Ref,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct FunctionDefinitionValue {
    label: Label,
    arguments: Vec<Ref>,
    body: Ref,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct FunctionCallValue {
    function: Ref,
    arguments: Vec<Ref>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct BinaryOperatorValue {
    operator: String,
    left: Ref,
    right: Ref,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Pattern {
    label: Label,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Label {
    name: String,
    colour: String,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Model {
            store: StorageService::new(Area::Local),
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
            path: VecDeque::new(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        const KEY: &str = "linc_file";
        match msg {
            Msg::Select(path) => {
                self.path = path;
            }
            Msg::Rename(reference, name) => {
                if let Some(node) = self.lookup_mut(&reference) {
                    node.rename(name);
                }
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
                let reference = self.file.add_node(Value::Pattern(PatternValue {
                    label: Label {
                        name: "xxx".to_string(),
                        colour: "red".to_string(),
                    },
                }));
                if let Some(node) = self
                    .current()
                    .and_then(|reference| self.lookup_mut(&reference))
                {
                    if let Value::FunctionDefinition(ref mut v) = node.value {
                        v.arguments.push(reference);
                    }
                }
            }
            Msg::AddItem => {
                let reference = self.file.add_node(Value::Hole);
                if let Some(node) = self
                    .current()
                    .and_then(|reference| self.lookup_mut(&reference))
                {
                    if let Value::List(ref mut v) = node.value {
                        v.items.push(reference);
                    }
                }
            }
            Msg::AddExpression => {
                let reference = self.file.add_node(Value::Hole);
                if let Some(node) = self
                    .current()
                    .and_then(|reference| self.lookup_mut(&reference))
                {
                    if let Value::Block(ref mut v) = node.value {
                        v.expressions.push(reference);
                    }
                }
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
                            body: "11111".to_string(),
                        }));
                self.file.bindings.push(reference);
            }
            Msg::SetValue(v) => {
                let reference = self.file.add_node(v);
                let current = self.current().unwrap_or(invalid_ref());
                if let Some(node) = self
                    .parent()
                    .and_then(|reference| self.lookup_mut(&reference))
                {
                    node.map_ref(|r| {
                        if *r == current {
                            reference.clone()
                        } else {
                            r.to_string()
                        }
                    });
                }
            }
        };
        true
    }
}

impl Renderable<Model> for Model {
    fn view(&self) -> Html<Self> {
        let selected_node_json = self
            .path
            .back()
            .and_then(|reference| self.lookup(reference))
            .map(|n| n.to_json())
            .unwrap_or("JSON ERROR".to_string());
        html! {
            <div>
                <div>{ "LINC" }</div>
                <div>{ self.view_actions() }</div>
                <div class="wrapper">
                    <div class="column">{ self.view_file(&self.file) }</div>
                    <div class="column">{ self.view_file_json(&self.file) }</div>
                    <div class="column">
                        <div>{ format!("Path: {:?}", self.path) }</div>
                        <pre class="column">{ selected_node_json }</pre>
                    </div>
                </div>
            </div>
        }
    }
}

struct Action {
    text: String,
    msg: Msg,
}

impl Model {
    fn current(&self) -> Option<Ref> {
        self.path.back().cloned()
    }

    fn parent(&self) -> Option<Ref> {
        let i = self.path.len() - 2;
        self.path.get(i).cloned()
    }

    fn view_actions(&self) -> Html<Model> {
        let actions = vec![
            Action {
                text: "store".to_string(),
                msg: Msg::Store,
            },
            Action {
                text: "load".to_string(),
                msg: Msg::Load,
            },
            Action {
                text: "+arg".to_string(),
                msg: Msg::AddArgument,
            },
            Action {
                text: "+fn".to_string(),
                msg: Msg::NewFn,
            },
            Action {
                text: "{}".to_string(),
                msg: Msg::SetValue(Value::Block(BlockValue {
                    expressions: vec![],
                })),
            },
            Action {
                text: "{☆}".to_string(),
                msg: Msg::SetValue(Value::Block(BlockValue {
                    expressions: vec![],
                })),
            },
            Action {
                text: "+expr".to_string(),
                msg: Msg::AddExpression,
            },
            Action {
                text: "[]".to_string(),
                msg: Msg::SetValue(Value::List(ListValue { items: vec![] })),
            },
            Action {
                text: "[☆]".to_string(),
                msg: Msg::SetValue(Value::List(ListValue { items: vec![] })),
            },
            Action {
                text: "+item".to_string(),
                msg: Msg::AddItem,
            },
            Action {
                text: "If (◆) then ◆".to_string(),
                msg: Msg::SetValue(Value::If(IfValue {
                    conditional: invalid_ref(),
                    true_body: invalid_ref(),
                    false_body: invalid_ref(),
                })),
            },
            Action {
                text: "If (☆) then ◆".to_string(),
                msg: Msg::SetValue(Value::If(IfValue {
                    conditional: invalid_ref(),
                    true_body: invalid_ref(),
                    false_body: invalid_ref(),
                })),
            },
            Action {
                text: "If (◆) then ☆".to_string(),
                msg: Msg::SetValue(Value::If(IfValue {
                    conditional: invalid_ref(),
                    true_body: invalid_ref(),
                    false_body: invalid_ref(),
                })),
            },
            Action {
                text: "Int".to_string(),
                msg: Msg::SetValue(Value::Int(0)),
            },
        ];
        let mut actions = actions.iter().map(|a| self.view_action(a));
        html! {
            <div>
            { for actions }
            </div>
        }
    }

    fn view_action(&self, action: &Action) -> Html<Model> {
        let m = action.msg.clone();
        html! {
            <div class="action" onclick=|_| m.clone()>
            { &action.text }
            </div>
        }
    }

    fn view_file(&self, file: &File) -> Html<Model> {
        html! {
            <div>{ for file.bindings.iter().map(|v| self.view_binding(v)) }</div>
        }
    }

    fn view_file_json(&self, file: &File) -> Html<Model> {
        let serialized = serde_json::to_string_pretty(file).expect("could not serialize to JSON");
        html! {
            <pre>{ serialized }</pre>
        }
    }

    fn view_label(&self, label: &Label, path: Path) -> Html<Model> {
        let reference = path.back().unwrap_or(&invalid_ref()).clone();
        html! {
            <input oninput=|e| Msg::Rename(reference.clone(), e.value)
                type="text"
                value=label.name/>
        }
    }

    fn view_binding(&self, reference: &Ref) -> Html<Model> {
        let node = self
            .lookup(reference)
            .map(|n| self.view_node(n, VecDeque::new()))
            .unwrap_or(self.view_invalid());
        html! {
            <div>{ node }</div>
        }
    }

    fn view_invalid(&self) -> Html<Model> {
        html! {
            <div>{ "ERROR" }</div>
        }
    }

    fn view_node(&self, node: &Node, mut path: Path) -> Html<Model> {
        let reference = node.reference.clone();
        path.push_back(reference.clone());
        let selected = match self.current() {
            None => false,
            Some(selected_reference) => selected_reference == reference,
        };
        let target = match self.selected_node() {
            None => false,
            Some(n) => {
                if let Value::Ref(ref target_reference) = n.value {
                    *target_reference == reference
                } else {
                    false
                }
            }
        };
        let mut classes = vec!["node".to_string()];
        if selected {
            classes.push("selected".to_string());
        }
        if target {
            classes.push("target".to_string());
        }
        let value = self.view_value(&node.value, path.clone());
        html! {
            <div class=classes.join(" ") onclick=|_| Msg::Select(path.clone())>
                <span>{ value }</span>
            </div>
        }
    }

    fn view_value(&self, value: &Value, path: Path) -> Html<Model> {
        match value {
            Value::Hole => {
                html! { <span>{ "@" }</span> }
            }
            Value::Bool(v) => {
                let v = if *v { "true" } else { "false" };
                html! { <span>{ v }</span> }
            }
            Value::Int(v) => {
                let v = format!("{}", v);
                html! { <span>{ v }</span> }
            }
            Value::Float(v) => {
                let v = format!("{}", v);
                html! { <span>{ v }</span> }
            }
            Value::String(v) => {
                html! { <span>{ v }</span> }
            }
            Value::Ref(reference) => {
                let node = self.lookup(reference);
                let text = node
                    .and_then(|n| n.label())
                    .map(|l| l.name.clone())
                    .unwrap_or("<UNKNOWN>".to_string());
                html! { <span>{ text }</span> }
            }
            Value::Binding(v) => {
                let label = self.view_label(&v.label, path.clone());
                let value = self
                    .lookup(&v.value)
                    .map(|n| self.view_node(n, path.clone()))
                    .unwrap_or(self.view_invalid());
                html! {
                    <span>
                    { label }{ "=" }{ value }
                    </span>
                }
            }
            Value::Pattern(v) => {
                html! {
                    <span>
                    { self.view_label(&v.label, path.clone()) }
                    </span>
                }
            }
            Value::Block(v) => {
                let mut expressions = v
                    .expressions
                    .iter()
                    .filter_map(|r| self.lookup(r))
                    .map(|n| self.view_node(n, path.clone()));
                html! {
                    <span>
                    { "{" }
                    { for expressions }
                    { "}" }
                    </span>
                }
            }
            Value::List(v) => {
                let mut items = v
                    .items
                    .iter()
                    .filter_map(|r| self.lookup(r))
                    .map(|n| self.view_node(n, path.clone()));
                html! {
                    <span>
                    { "[" }{ for items }{ "]" }
                    </span>
                }
            }
            Value::If(v) => {
                let conditional = self
                    .lookup(&v.conditional)
                    .map(|n| self.view_node(n, path.clone()))
                    .unwrap_or(self.view_invalid());
                html! {
                    <span>
                    { "if" }{ conditional }
                    </span>
                }
            }
            Value::FunctionDefinition(v) => {
                let label = self.view_label(&v.label, path.clone());
                let mut args = v
                    .arguments
                    .iter()
                    // TODO: We should not filter out invalid nodes.
                    .filter_map(|r| self.lookup(r))
                    .map(|n| self.view_node(n, path.clone()));
                let body = self
                    .lookup(&v.body)
                    .map(|n| self.view_node(n, path.clone()))
                    .unwrap_or(self.view_invalid());
                html! {
                    <span>
                    { "fn" }{ label }
                    { "(" }{ for args }{ ")" }
                    { body }
                    </span>
                }
            }
            Value::FunctionCall(v) => {
                let node = self.file.lookup(&v.function);
                let function_name = node
                    .and_then(|n| n.label())
                    .map(|l| l.name.clone())
                    .unwrap_or("<UNKNOWN>".to_string());
                let mut args = v
                    .arguments
                    .iter()
                    // TODO: We should not filter out invalid nodes.
                    .filter_map(|r| self.lookup(r))
                    .map(|n| self.view_node(n, path.clone()));
                html! {
                    <span>
                    { function_name }
                    { "(" }{ for args }{ ")" }
                    </span>
                }
            }
            Value::BinaryOperator(v) => {
                let left = self
                    .lookup(&v.left)
                    .map(|n| self.view_node(n, path.clone()))
                    .unwrap_or(self.view_invalid());
                let right = self
                    .lookup(&v.right)
                    .map(|n| self.view_node(n, path.clone()))
                    .unwrap_or(self.view_invalid());
                html! {
                    <span>
                    { left }
                    { &v.operator }
                    { right }
                    </span>
                }
            }
        }
    }
}
