use std::collections::VecDeque;
use yew::prelude::*;

use crate::types::*;

impl Model {
    pub fn view_actions(&self) -> Html {
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
        let actions = actions.iter().map(|a| self.view_action(a));
        html! {
            <div>
            { for actions }
            </div>
        }
    }

    fn view_action(&self, action: &Action) -> Html {
        let msg = action.msg.clone();
        let callback = self.link.callback(move |_: MouseEvent| msg.clone());
        html! {
            <div class="action" onclick=callback>
            { &action.text }
            </div>
        }
    }

    pub fn view_file(&self, file: &File) -> Html {
        html! {
            <div>{ for file.bindings.iter().map(|v| self.view_binding(v)) }</div>
        }
    }

    pub fn view_file_json(&self, file: &File) -> Html {
        let serialized = serde_json::to_string_pretty(file).expect("could not serialize to JSON");
        html! {
            <pre>{ serialized }</pre>
        }
    }

    fn view_label(&self, label: &Label, path: Path) -> Html {
        let reference = path.back().unwrap_or(&invalid_ref()).clone();
        let callback = self
            .link
            .callback(move |e: InputData| Msg::Rename(reference.clone(), e.value));
        html! {
            <input oninput=callback
                type="text"
                value=label.name/>
        }
    }

    fn view_binding(&self, reference: &Ref) -> Html {
        let node = self
            .lookup(reference)
            .map(|n| self.view_node(n, VecDeque::new()))
            .unwrap_or(self.view_invalid());
        html! {
            <div>{ node }</div>
        }
    }

    fn view_invalid(&self) -> Html {
        html! {
            <span>{ "ERROR" }</span>
        }
    }

    fn view_node(&self, node: &Node, mut path: Path) -> Html {
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
        let callback = self
            .link
            .callback(move |_: MouseEvent| Msg::Select(path.clone()));
        html! {
            <div class=classes.join(" ") onclick=callback>
                <span>{ value }</span>
            </div>
        }
    }

    fn view_value(&self, value: &Value, path: Path) -> Html {
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
                let expressions = v
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
                let items = v
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
                let true_body = self
                    .lookup(&v.true_body)
                    .map(|n| self.view_node(n, path.clone()))
                    .unwrap_or(self.view_invalid());
                let false_body = self
                    .lookup(&v.false_body)
                    .map(|n| self.view_node(n, path.clone()))
                    .unwrap_or(self.view_invalid());
                html! {
                    <span>
                    { "if" }{ conditional }
                    { "{" }<div class="block">{ true_body }</div>{ "}" }
                    { "else" }
                    { "{" }<div class="block">{ false_body }</div>{ "}" }
                    </span>
                }
            }
            Value::FunctionDefinition(v) => {
                let label = self.view_label(&v.label, path.clone());
                let args = v
                    .arguments
                    .iter()
                    // TODO: We should not filter out invalid nodes.
                    .filter_map(|r| self.lookup(r))
                    .map(|n| self.view_node(n, path.clone()));
                let body = self
                    .lookup(&v.body)
                    .map(|n| self.view_node(n, path.clone()))
                    .unwrap_or(self.view_invalid());
                let return_type = self
                    .lookup(&v.return_type)
                    .map(|n| self.view_node(n, path.clone()))
                    .unwrap_or(self.view_invalid());

                let mut p = path.clone();
                p.push_back("xxx".to_string());

                let callback = self
                    .link
                    .callback(move |_: MouseEvent| Msg::Select(p.clone()));
                html! {
                    <span>
                    <div onclick=callback>{ "#" }</div>
                    { "fn" }{ label }
                    { "(" }{ for args }{ ")" }
                    { "->" }{ return_type }
                    { "{" }<div class="block">{ body }</div>{ "}" }
                    </span>
                }
            }
            Value::FunctionCall(v) => {
                let node = self.file.lookup(&v.function);
                let function_name = node
                    .and_then(|n| n.label())
                    .map(|l| l.name.clone())
                    .unwrap_or("<UNKNOWN>".to_string());
                let args = v
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
