use serde::{Deserialize, Serialize};
use yew::{html, Component, ComponentLink, Html, Renderable, ShouldRender};

type Ref = i32;

pub struct Model {
    file: File,
    selected: Option<Ref>,
}

impl Model {
    fn lookup(&self, reference: Ref) -> Option<&Node> {
        self.file.lookup(reference)
    }
    fn lookup_mut(&mut self, reference: Ref) -> Option<&mut Node> {
        self.file.lookup_mut(reference)
    }
}

pub enum Msg {
    Select(Ref),
    Rename(Ref, String),
}

#[derive(Serialize, Deserialize)]
struct File {
    bindings: Vec<Ref>,
    nodes: Vec<Node>,
}

impl File {
    fn lookup(&self, reference: Ref) -> Option<&Node> {
        self.nodes
            .iter()
            .filter(|v| v.reference == reference)
            .next()
    }

    fn lookup_mut(&mut self, reference: Ref) -> Option<&mut Node> {
        self.nodes
            .iter_mut()
            .filter(|v| v.reference == reference)
            .next()
    }
}

#[derive(Serialize, Deserialize)]
struct Node {
    reference: Ref,
    value: Value,
}

const ERROR_NODE: Node = Node {
    reference: 1111111111,
    value: Value::Hole,
};

#[derive(Serialize, Deserialize)]
enum Value {
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
            _ => None,
        }
    }
    fn rename(&mut self, name: String) {
        match &mut self.value {
            Value::Binding(ref mut v) => v.label.name = name,
            Value::Pattern(ref mut v) => v.label.name = name,
            _ => {}
        }
    }
}

#[derive(Serialize, Deserialize)]
struct BindingValue {
    label: Label,
    value: Ref,
}

#[derive(Serialize, Deserialize)]
struct PatternValue {
    label: Label,
}

#[derive(Serialize, Deserialize)]
struct BlockValue {
    expressions: Vec<Ref>,
}

#[derive(Serialize, Deserialize)]
struct ListValue {
    items: Vec<Ref>,
}

#[derive(Serialize, Deserialize)]
struct IfValue {
    conditional: Ref,
    true_body: Ref,
    false_body: Ref,
}

#[derive(Serialize, Deserialize)]
struct FunctionDefinitionValue {
    arguments: Vec<Ref>,
    body: Ref,
}

#[derive(Serialize, Deserialize)]
struct FunctionCallValue {
    function: Ref,
    arguments: Vec<Ref>,
}

#[derive(Serialize, Deserialize)]
struct BinaryOperatorValue {
    operator: String,
    left: Ref,
    right: Ref,
}

#[derive(Serialize, Deserialize)]
struct Pattern {
    label: Label,
}

#[derive(Serialize, Deserialize)]
struct Label {
    name: String,
    colour: String,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Model {
            file: File {
                nodes: vec![
                    Node {
                        reference: 111,
                        value: Value::Binding(BindingValue {
                            label: Label {
                                name: "main".to_string(),
                                colour: "red".to_string(),
                            },
                            value: 123,
                        }),
                    },
                    Node {
                        reference: 123,
                        value: Value::FunctionDefinition(FunctionDefinitionValue {
                            arguments: vec![],
                            body: 124,
                        }),
                    },
                    Node {
                        reference: 124,
                        value: Value::Int(123),
                    },
                    Node {
                        reference: 12,
                        value: Value::Binding(BindingValue {
                            label: Label {
                                name: "factorial".to_string(),
                                colour: "red".to_string(),
                            },
                            value: 1231312,
                        }),
                    },
                    Node {
                        reference: 1231312,
                        value: Value::FunctionDefinition(FunctionDefinitionValue {
                            arguments: vec![222],
                            body: 228,
                        }),
                    },
                    Node {
                        reference: 222,
                        value: Value::Pattern(PatternValue {
                            label: Label {
                                name: "x".to_string(),
                                colour: "".to_string(),
                            },
                        }),
                    },
                    Node {
                        reference: 228,
                        value: Value::BinaryOperator(BinaryOperatorValue {
                            operator: "*".to_string(),
                            left: 1231,
                            right: 1232,
                        }),
                    },
                    Node {
                        reference: 1231,
                        value: Value::Ref(222),
                    },
                    Node {
                        reference: 1232,
                        value: Value::FunctionCall(FunctionCallValue {
                            function: 12,
                            arguments: vec![229],
                        }),
                    },
                    Node {
                        reference: 229,
                        value: Value::BinaryOperator(BinaryOperatorValue {
                            operator: "-".to_string(),
                            left: 230,
                            right: 231,
                        }),
                    },
                    Node {
                        reference: 230,
                        value: Value::Ref(222),
                    },
                    Node {
                        reference: 231,
                        value: Value::Int(1),
                    },
                ],
                bindings: vec![111, 12],
            },
            selected: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Select(reference) => {
                self.selected = Some(reference);
            }
            Msg::Rename(reference, name) => {
                if let Some(node) = self.lookup_mut(reference) {
                    node.rename(name);
                }
            }
        };
        true
    }
}

impl Renderable<Model> for Model {
    fn view(&self) -> Html<Self> {
        html! {
            <div>
                <div>{ "LINC" }</div>
                <div>{ format!("Selected: {:?}", self.selected) }</div>
                <div>{ self.view_file(&self.file) }</div>
                </div>
        }
    }
}

impl Model {
    fn view_file(&self, file: &File) -> Html<Model> {
        let serialized = serde_json::to_string_pretty(file).expect("could not serialize to JSON");
        html! {
            <div>
                <div>{ "file" }</div>
                <div>{ for file.bindings.iter().map(|v| self.view_binding(*v)) }</div>
                <div>{ "JSON" }</div>
                <pre>{ serialized }</pre>
                </div>
        }
    }

    fn view_label(&self, label: &Label, reference: Ref) -> Html<Model> {
        html! {
            <input oninput=|e| Msg::Rename(reference, e.value)
                type="text"
                value=label.name/>
        }
    }

    fn view_binding(&self, reference: Ref) -> Html<Model> {
        let node = self.lookup(reference).unwrap_or(&ERROR_NODE);
        html! {
            <div>{ self.view_node(node) }</div>
        }
    }

    fn view_node(&self, node: &Node) -> Html<Model> {
        let reference = node.reference;
        let selected = match self.selected {
            None => false,
            Some(selected_reference) => selected_reference == reference,
        };
        // TODO: Use Vec.
        let mut class = "node".to_string();
        if selected {
            class.push_str(" selected");
        }
        html! {
            <div class=class onclick=|_| Msg::Select(reference)>
                <span>{ self.view_value(&node.value, reference) }</span>
                </div>
        }
    }

    fn view_value(&self, value: &Value, reference: Ref) -> Html<Model> {
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
                let node = self.lookup(*reference);
                let text = node
                    .and_then(|n| n.label())
                    .map(|l| l.name.clone())
                    .unwrap_or("<UNKNOWN>".to_string());
                html! { <span>{ text }</span> }
            }
            Value::Binding(v) => {
                let label = self.view_label(&v.label, reference);
                let value = self.lookup(v.value).unwrap_or(&ERROR_NODE);
                if let Value::FunctionDefinition(ref fd) = value.value {
                    let mut args = fd
                        .arguments
                        .iter()
                        // TODO: We should not filter out invalid nodes.
                        .filter_map(|r| self.lookup(*r))
                        .map(|n| self.view_node(n));
                    let body = self.lookup(fd.body).unwrap_or(&ERROR_NODE);
                    html! {
                        <span>
                        { "fn" }{ label }
                        { "(" }{ for args }{ ")" }
                        { self.view_node(body) }
                        </span>
                    }
                } else {
                    html! {
                        <span>
                        { label }{ "=" }{ self.view_node(value) }
                        </span>
                    }
                }
            }
            Value::Pattern(v) => {
                html! {
                    <span>
                    { self.view_label(&v.label, reference) }
                    </span>
                }
            }
            Value::Block(_) => {
                html! { <span>{ "xxx" }</span> }
            }
            Value::List(_) => {
                html! { <span>{ "xxx" }</span> }
            }
            Value::If(_) => {
                html! { <span>{ "xxx" }</span> }
            }
            Value::FunctionDefinition(v) => {
                let mut args = v
                    .arguments
                    .iter()
                    // TODO: We should not filter out invalid nodes.
                    .filter_map(|r| self.lookup(*r))
                    .map(|n| self.view_node(n));
                let body = self.lookup(v.body).unwrap_or(&ERROR_NODE);
                html! {
                    <span>
                    { "fn" }
                    { "(" }{ for args }{ ")" }
                    { self.view_node(body) }
                    </span>
                }
            }
            Value::FunctionCall(v) => {
                let node = self.file.lookup(v.function);
                let function_name = node
                    .and_then(|n| n.label())
                    .map(|l| l.name.clone())
                    .unwrap_or("<UNKNOWN>".to_string());
                let mut args = v
                    .arguments
                    .iter()
                    // TODO: We should not filter out invalid nodes.
                    .filter_map(|r| self.lookup(*r))
                    .map(|n| self.view_node(n));
                html! {
                    <span>
                    { function_name }
                    { "(" }{ for args }{ ")" }
                    </span>
                }
            }
            Value::BinaryOperator(v) => {
                let left = self.lookup(v.left).unwrap_or(&ERROR_NODE);
                let right = self.lookup(v.right).unwrap_or(&ERROR_NODE);
                html! {
                    <span>
                    { self.view_node(left) }
                    { &v.operator }
                    { self.view_node(right) }
                    </span>
                }
            }
        }
    }

    fn node_by_reference(&self, reference: Ref) -> Option<Node> {
        None
    }
}
