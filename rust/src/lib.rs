use serde::{Deserialize, Serialize};
use yew::{html, Component, ComponentLink, Html, Renderable, ShouldRender};

type Ref = i32;

pub struct Model {
    file: File,
    selected: Option<Ref>,
}

pub enum Msg {
    Select(Ref),
    Rename(Ref, String),
}

#[derive(Serialize, Deserialize)]
struct File {
    bindings: Vec<Node>,
}

impl File {
    fn lookup(&self, reference: Ref) -> Option<&Node> {
        self.bindings
            .iter()
            .filter_map(|v| v.lookup(reference))
            .next()
    }
}

#[derive(Serialize, Deserialize)]
struct Node {
    reference: Ref,
    value: Value,
}

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
    fn lookup(&self, reference: Ref) -> Option<&Node> {
        if reference == self.reference {
            Some(self)
        } else {
            match self.value {
                Value::Hole => None,
                Value::Bool(_) => None,
                Value::Int(_) => None,
                Value::Float(_) => None,
                Value::String(_) => None,
                Value::Ref(_) => None,
                Value::Binding(_) => None,
                Value::Pattern(_) => None,
                Value::Block(_) => None,
                Value::List(_) => None,
                Value::If(ref v) => v
                    .conditional
                    .lookup(reference)
                    .or(v.true_body.lookup(reference))
                    .or(v.false_body.lookup(reference)),
                Value::FunctionDefinition(ref v) => v
                    .arguments
                    .iter()
                    .filter_map(|v| v.lookup(reference))
                    .next()
                    .or(v.body.lookup(reference)),
                Value::FunctionCall(ref v) => v
                    .arguments
                    .iter()
                    .filter_map(|v| v.lookup(reference))
                    .next(),
                Value::BinaryOperator(ref v) => {
                    v.left.lookup(reference).or(v.right.lookup(reference))
                }
            }
        }
    }

    fn label(&self) -> Option<&Label> {
        match &self.value {
            Value::Binding(ref v) => Some(&v.label),
            _ => None,
        }
    }
}

#[derive(Serialize, Deserialize)]
struct BindingValue {
    label: Label,
    value: Box<Node>,
}

#[derive(Serialize, Deserialize)]
struct PatternValue {
    label: Label,
}

#[derive(Serialize, Deserialize)]
struct BlockValue {
    expressions: Vec<Node>,
}

#[derive(Serialize, Deserialize)]
struct ListValue {
    items: Vec<Node>,
}

#[derive(Serialize, Deserialize)]
struct IfValue {
    conditional: Box<Node>,
    true_body: Box<Node>,
    false_body: Box<Node>,
}

#[derive(Serialize, Deserialize)]
struct FunctionDefinitionValue {
    arguments: Vec<Node>,
    body: Box<Node>,
}

#[derive(Serialize, Deserialize)]
struct FunctionCallValue {
    function: Ref,
    arguments: Vec<Node>,
}

#[derive(Serialize, Deserialize)]
struct BinaryOperatorValue {
    operator: String,
    left: Box<Node>,
    right: Box<Node>,
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
                bindings: vec![
                    Node {
                        reference: 111,
                        value: Value::Binding(BindingValue {
                            label: Label {
                                name: "main".to_string(),
                                colour: "red".to_string(),
                            },
                            value: Box::new(Node {
                                reference: 123,
                                value: Value::FunctionDefinition(FunctionDefinitionValue {
                                    arguments: vec![],
                                    body: Box::new(Node {
                                        reference: 124,
                                        value: Value::Int(123),
                                    }),
                                }),
                            }),
                        }),
                    },
                    Node {
                        reference: 12,
                        value: Value::Binding(BindingValue {
                            label: Label {
                                name: "factorial".to_string(),
                                colour: "red".to_string(),
                            },
                            value: Box::new(Node {
                                reference: 1231312,
                                value: Value::FunctionDefinition(FunctionDefinitionValue {
                                    arguments: vec![Node {
                                        reference: 222,
                                        value: Value::Pattern(PatternValue {
                                            label: Label {
                                                name: "x".to_string(),
                                                colour: "".to_string(),
                                            },
                                        }),
                                    }],
                                    body: Box::new(Node {
                                        reference: 228,
                                        value: Value::Hole,
                                    }),
                                }),
                            }),
                        }),
                    },
                ],
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
                /*
                  for b in self.file.bindings.iter() {
                  let mut n = Node::Binding(b);
                  n.map(|n| match n {
                  Node::Expression(v) => {}
                  Node::Pattern(v) => {}
                  Node::Binding(ref mut v) => {
                  if v.reference == reference {
                  v.label.name = name.clone();
                  }
                  }
                  });
                  }
                  if let Some(ref mut node) = self.file.lookup(reference) {
                  match node {
                  Node::Expression(_) => {}
                  Node::Pattern(_) => {}
                  Node::Binding(ref mut v) => {
                *v = &Binding {
                reference: 1111,
                label: Label {
                name: "test".to_string(),
                colour: "red".to_string(),
                },
                value: Expression {
                reference: 123,
                value: Value::Hole,
                },
                }
                }
                };
                }
                */
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
                <div>{ for file.bindings.iter().map(|v| self.view_binding(&v)) }</div>
                <div>{ "JSON" }</div>
                <pre>{ serialized }</pre>
            </div>
        }
    }

    fn view_label(&self, label: &Label) -> Html<Model> {
        html! {
            <span>{ &label.name }</span>
        }
    }

    fn view_binding(&self, node: &Node) -> Html<Model> {
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
            <span class=class
                  onclick=|_| Msg::Select(reference)>
                <span>{ self.view_value(&node.value) }</span>
            </span>
        }
    }

    fn view_value(&self, value: &Value) -> Html<Model> {
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
                let node = self.file.lookup(*reference);
                let text = node
                    .and_then(|n| n.label())
                    .map(|l| l.name.clone())
                    .unwrap_or("<UNKNOWN>".to_string());
                html! { <span>{ text }</span> }
            }
            Value::Binding(v) => {
                html! {
                    <span>
                        { self.view_label(&v.label) }
                        { "=" }
                        { self.view_node(&v.value) }
                    </span>
                }
            }
            Value::Pattern(_) => {
                html! { <span>{ "xxx" }</span> }
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
                html! {
                    <span>
                        { "fn" }
                        { "(" }{ for v.arguments.iter().map(|v| self.view_node(v)) }{ ")" }
                        { self.view_node(&v.body) }
                    </span>
                }
            }
            Value::FunctionCall(v) => {
                let node = self.file.lookup(v.function);
                let function_name = node
                    .and_then(|n| n.label())
                    .map(|l| l.name.clone())
                    .unwrap_or("<UNKNOWN>".to_string());
                html! {
                    <span>
                        { function_name }
                        { "(" }{ for v.arguments.iter().map(|v| self.view_node(v)) }{ ")" }
                    </span>
                }
            }
            Value::BinaryOperator(v) => {
                html! {
                    <span>
                        { self.view_node(&v.left) }
                        { &v.operator }
                        { self.view_node(&v.right) }
                    </span>
                }
            }
        }
    }

    fn node_by_reference(&self, reference: Ref) -> Option<Node> {
        None
    }
}
