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
                text: "prev".to_string(),
                msg: Msg::Prev,
            },
            Action {
                text: "next".to_string(),
                msg: Msg::Next,
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

    pub fn view_file(&self, file: &File, cursor: Path) -> Html {
        html! {
            <div>{ for file.bindings.iter().enumerate().map(|(i, v)| self.view_binding(v, vec![Selector::Index(i)].into(), cursor.clone())) }</div>
        }
    }

    pub fn view_file_json(&self, file: &File) -> Html {
        let serialized = serde_json::to_string_pretty(file).expect("could not serialize to JSON");
        html! {
            <pre>{ serialized }</pre>
        }
    }

    fn view_label(&self, reference: &Ref, label: &Label) -> Html {
        let reference = reference.clone();
        let callback = self
            .link
            .callback(move |e: InputData| Msg::Rename(reference.clone(), e.value));
        html! {
            <input oninput=callback
                type="text"
                value=label.name/>
        }
    }

    fn view_binding(&self, reference: &Ref, path: Path, cursor: Path) -> Html {
        let node = self.view_node(reference, path, cursor);
        html! {
            <div>{ node }</div>
        }
    }

    fn view_node_list(&self, references: &[Ref], path: Path, cursor: Path) -> Html {
        let sp = format!("{:?}", path);
        let nodes = references
            .iter()
            .enumerate()
            .map(|(i, n)| self.view_node(n, append(&path, Selector::Index(i)), cursor.clone()));
        let path = path.clone();
        let callback = self
            .link
            .callback(move |_: MouseEvent| Msg::Select(path.clone()));
        html! {
            <div class="node" onclick=callback path={sp}>{ for nodes }</div>
        }
    }

    fn view_node(&self, reference: &Ref, path: Path, cursor: Path) -> Html {
        match self.lookup(reference) {
            Some(node) => {
                // let selected = remaining_path.empty();
                // let target = match self.selected_node() {
                //     None => false,
                //     Some(n) => {
                //         if let Value::Ref(ref target_reference) = n.value {
                //             *target_reference == reference
                //         } else {
                //             false
                //         }
                //     }
                // };
                let mut classes = vec!["node".to_string()];
                // if selected {
                //     classes.push("selected".to_string());
                // }
                // if target {
                //     classes.push("target".to_string());
                // }
                let value = self.view_value(&node.reference, &node.value, path.clone(), cursor);
                let sp = format!("{:?}", path);
                let callback = self
                    .link
                    .callback(move |_: MouseEvent| Msg::Select(path.clone()));
                html! {
                    <div class=classes.join(" ") onclick=callback path={sp}>
                        <span>{ value }</span>
                    </div>
                }
            }
            None => {
                html! {
                    <div>
                        <span>{ "error" }</span>
                    </div>
                }
            }
        }
    }

    fn callback_child<IN>(&self, path: &Path, child: Selector) -> Callback<IN> {
        let mut path = path.clone();
        path.push_back(child);
        self.link.callback(move |_: IN| Msg::Select(path.clone()))
    }

    fn view_value(&self, reference: &Ref, value: &Value, path: Path, cursor: Path) -> Html {
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
                let label = self.view_label(reference, &v.label);
                let value = self.view_node(&v.value, path.clone(), cursor);
                let label_callback =
                    self.callback_child(&path, Selector::Field("label".to_string()));
                let value_callback =
                    self.callback_child(&path, Selector::Field("value".to_string()));
                html! {
                    <span>
                        <span onclick=label_callback>{ label }</span>
                        { "=" }
                        <span onclick=value_callback>{ value }</span>
                    </span>
                }
            }
            Value::Pattern(v) => {
                html! {
                    <span>
                    { self.view_label(reference, &v.label) }
                    </span>
                }
            }
            Value::Block(v) => {
                let expressions = v
                    .expressions
                    .iter()
                    .map(|n| self.view_node(n, path.clone(), cursor.clone()));
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
                    .map(|n| self.view_node(n, path.clone(), cursor.clone()));
                html! {
                    <span>
                    { "[" }{ for items }{ "]" }
                    </span>
                }
            }
            Value::If(v) => {
                let conditional = self.view_node(
                    &v.conditional,
                    append(&path, Selector::Field("conditional".to_string())),
                    cursor.clone(),
                );

                let true_body = self.view_node(
                    &v.true_body,
                    append(&path, Selector::Field("true_body".to_string())),
                    cursor.clone(),
                );

                let false_body = self.view_node(
                    &v.false_body,
                    append(&path, Selector::Field("true_body".to_string())),
                    cursor.clone(),
                );

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
                let label = self.view_label(reference, &v.label);

                let args = self.view_node_list(
                    v.arguments.as_ref(),
                    append(&path, Selector::Field("args".to_string())),
                    cursor.clone(),
                );
                let body = self.view_node(
                    &v.body,
                    append(&path, Selector::Field("body".to_string())),
                    cursor.clone(),
                );
                let return_type = self.view_node(
                    &v.return_type,
                    append(&path, Selector::Field("return_type".to_string())),
                    cursor.clone(),
                );

                let mut p = path.clone();
                // p.push_back("xxx".to_string());

                let callback = self
                    .link
                    .callback(move |_: MouseEvent| Msg::Select(p.clone()));
                html! {
                    <span>
                    <div onclick=callback>{ "#" }</div>
                    { "fn" }{ label }
                    { "(" }{ args }{ ")" }
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
                let args = self.view_node_list(
                    v.arguments.as_ref(),
                    append(&path, Selector::Field("arguments".to_string())),
                    cursor,
                );
                html! {
                    <span>
                    { function_name }
                    { "(" }{ args }{ ")" }
                    </span>
                }
            }
            Value::BinaryOperator(v) => {
                let left = self.view_node(
                    &v.left,
                    append(&path, Selector::Field("left".to_string())),
                    cursor.clone(),
                );
                let right = self.view_node(
                    &v.right,
                    append(&path, Selector::Field("right".to_string())),
                    cursor.clone(),
                );
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
