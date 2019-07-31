use yew::{html, Component, ComponentLink, Html, Renderable, ShouldRender};

type Ref = i32;

pub struct Model {
    file: File,
    selected: Option<Ref>,
}

pub enum Msg {
    Select(Ref),
}

struct File {
    bindings: Vec<Binding>,
}

impl Lookup for File {
    fn lookup(&self, reference: Ref) -> Option<Node> {
        self.bindings
            .iter()
            .filter_map(|v| v.lookup(reference))
            .next()
    }
}

struct Binding {
    reference: Ref,
    label: Label,
    value: Expression,
}

impl Lookup for Binding {
    fn lookup(&self, reference: Ref) -> Option<Node> {
        if reference == self.reference {
            Some(Node::Binding(self))
        } else {
            self.value.lookup(reference)
        }
    }
}

enum Node<'a> {
    Expression(&'a Expression),
    Pattern(&'a Pattern),
    Binding(&'a Binding),
}

impl<'a> Node<'a> {
    fn label(&self) -> Option<&'a Label> {
        match self {
            Node::Expression(_) => None,
            Node::Pattern(v) => Some(&v.label),
            Node::Binding(v) => Some(&v.label),
        }
    }
}

struct Expression {
    reference: Ref,
    value: Value,
}

enum Value {
    Hole,

    Bool(bool),
    Int(i32),
    Float(f32),
    String(String),

    Ref(Ref),

    Block(BlockValue),
    List(ListValue),
    If(IfValue),
    FunctionDefinition(FunctionDefinitionValue),
    FunctionCall(FunctionCallValue),
    BinaryOperator(BinaryOperatorValue),
}

trait Lookup {
    fn lookup(&self, reference: Ref) -> Option<Node>;
}

impl Lookup for Expression {
    fn lookup(&self, reference: Ref) -> Option<Node> {
        if reference == self.reference {
            Some(Node::Expression(self))
        } else {
            match self.value {
                Value::Hole => None,
                Value::Bool(_) => None,
                Value::Int(_) => None,
                Value::Float(_) => None,
                Value::String(_) => None,
                Value::Ref(_) => None,
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
}

struct BlockValue {
    expressions: Vec<Expression>,
}

struct ListValue {
    items: Vec<Expression>,
}

struct IfValue {
    conditional: Box<Expression>,
    true_body: Box<Expression>,
    false_body: Box<Expression>,
}

struct FunctionDefinitionValue {
    arguments: Vec<Pattern>,
    body: Box<Expression>,
}

struct FunctionCallValue {
    function: Ref,
    arguments: Vec<Expression>,
}

struct BinaryOperatorValue {
    operator: String,
    left: Box<Expression>,
    right: Box<Expression>,
}

struct Pattern {
    reference: Ref,
    label: Label,
}

impl Lookup for Pattern {
    fn lookup(&self, reference: Ref) -> Option<Node> {
        if reference == self.reference {
            Some(Node::Pattern(self))
        } else {
            None
        }
    }
}

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
                    Binding {
                        reference: 111,
                        label: Label {
                            name: "main".to_string(),
                            colour: "red".to_string(),
                        },
                        value: Expression {
                            reference: 123,
                            value: Value::FunctionDefinition(FunctionDefinitionValue {
                                arguments: vec![],
                                body: Box::new(Expression {
                                    reference: 124,
                                    value: Value::Int(123),
                                }),
                            }),
                        },
                    },
                    Binding {
                        reference: 12,
                        label: Label {
                            name: "factorial".to_string(),
                            colour: "red".to_string(),
                        },
                        value: Expression {
                            reference: 224,
                            value: Value::FunctionDefinition(FunctionDefinitionValue {
                                arguments: vec![Pattern {
                                    reference: 222,
                                    label: Label {
                                        name: "x".to_string(),
                                        colour: "".to_string(),
                                    },
                                }],
                                body: Box::new(Expression {
                                    reference: 228,
                                    value: Value::BinaryOperator(BinaryOperatorValue {
                                        operator: "*".to_string(),
                                        left: Box::new(Expression {
                                            reference: 230,
                                            value: Value::Ref(222),
                                        }),
                                        right: Box::new(Expression {
                                            reference: 231,
                                            value: Value::FunctionCall(FunctionCallValue {
                                                function: 12,
                                                arguments: vec![Expression {
                                                    reference: 232,
                                                    value: Value::BinaryOperator(
                                                        BinaryOperatorValue {
                                                            operator: "-".to_string(),
                                                            left: Box::new(Expression {
                                                                reference: 233,
                                                                value: Value::Ref(222),
                                                            }),
                                                            right: Box::new(Expression {
                                                                reference: 234,
                                                                value: Value::Int(1),
                                                            }),
                                                        },
                                                    ),
                                                }],
                                            }),
                                        }),
                                    }),
                                }),
                            }),
                        },
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
        html! {
            <div>
                <div>{ "file" }</div>
                <div>{ for file.bindings.iter().map(|v| self.view_binding(&v)) }</div>
                </div>
        }
    }

    fn view_label(&self, label: &Label) -> Html<Model> {
        html! {
            <span>{ &label.name }</span>
        }
    }

    fn view_binding(&self, binding: &Binding) -> Html<Model> {
        let reference = binding.reference;
        let selected = match self.selected {
            None => false,
            Some(selected_reference) => selected_reference == reference,
        };
        let mut class = "node".to_string();
        if selected {
            class.push_str(" selected");
        }
        html! {
            <div class=class
                onclick=|_| Msg::Select(reference) >
                <span>{ self.view_label(&binding.label) }</span>
                <span>{ "=" }</span>
                <span>{ self.view_expression(&binding.value) }</span>
                </div>
        }
    }

    fn view_expression(&self, expression: &Expression) -> Html<Model> {
        let reference = expression.reference;
        let selected = match self.selected {
            None => false,
            Some(selected_reference) => selected_reference == reference,
        };
        let mut class = "node".to_string();
        if selected {
            class.push_str(" selected");
        }
        let value = self.view_value(&expression.value);
        html! {
            <span class=class
                onclick=|_| Msg::Select(reference) >{ value }
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
                      { "(" }{ for v.arguments.iter().map(|v| self.view_pattern(v)) }{ ")" }
                      { self.view_expression(&v.body) }
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
                      { "(" }{ for v.arguments.iter().map(|v| self.view_expression(v)) }{ ")" }
                    </span>
                }
            }
            Value::BinaryOperator(v) => {
                html! {
                    <span>
                      { self.view_expression(&v.left) }
                      { &v.operator }
                      { self.view_expression(&v.right) }
                    </span>
                }
            }
        }
    }

    fn view_pattern(&self, pattern: &Pattern) -> Html<Model> {
        let reference = pattern.reference;
        let selected = match self.selected {
            None => false,
            Some(selected_reference) => selected_reference == reference,
        };
        let mut class = "node".to_string();
        if selected {
            class.push_str(" selected");
        }
        html! {
            <span class=class
                  onclick=|_| Msg::Select(reference)>
              { &pattern.label.name }
            </span>
        }
    }

    fn node_by_reference(&self, reference: Ref) -> Option<Node> {
        None
    }
}
