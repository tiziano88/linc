use yew::prelude::*;

use crate::types::*;
use std::collections::HashMap;

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
                text: "parent".to_string(),
                msg: Msg::Parent,
            },
            Action {
                text: "+arg".to_string(),
                msg: Msg::AddArgument,
            },
            Action {
                text: "+fn".to_string(),
                msg: Msg::NewFn,
            },
            // Action {
            //     text: "{}".to_string(),
            //     msg: Msg::SetValue(Value::Block(BlockValue {
            //         expressions: vec![],
            //     })),
            // },
            // Action {
            //     text: "{☆}".to_string(),
            //     msg: Msg::SetValue(Value::Block(BlockValue {
            //         expressions: vec![],
            //     })),
            // },
            Action {
                text: "+expr".to_string(),
                msg: Msg::AddExpression,
            },
            // Action {
            //     text: "[]".to_string(),
            //     msg: Msg::SetValue(Value::List(ListValue { items: vec![] })),
            // },
            // Action {
            //     text: "[☆]".to_string(),
            //     msg: Msg::SetValue(Value::List(ListValue { items: vec![] })),
            // },
            Action {
                text: "+item".to_string(),
                msg: Msg::AddItem,
            },
            Action {
                text: "If (◆) then ◆".to_string(),
                msg: Msg::SetValue(Value::Inner(Inner {
                    kind: "if".to_string(),
                    children: HashMap::new(),
                })),
            },
            Action {
                text: "***".to_string(),
                msg: Msg::SetValue(Value::Inner(Inner {
                    kind: "binary_operator".to_string(),
                    children: HashMap::new(),
                })),
            },
            Action {
                text: "struct".to_string(),
                msg: Msg::SetValue(Value::Inner(Inner {
                    kind: "struct".to_string(),
                    children: HashMap::new(),
                })),
            },
            Action {
                text: "enum".to_string(),
                msg: Msg::SetValue(Value::Inner(Inner {
                    kind: "enum".to_string(),
                    children: HashMap::new(),
                })),
            },
            Action {
                text: "false".to_string(),
                msg: Msg::SetValue(Value::Bool(false)),
            },
            Action {
                text: "true".to_string(),
                msg: Msg::SetValue(Value::Bool(true)),
            },
            Action {
                text: "0".to_string(),
                msg: Msg::SetValue(Value::Int(0)),
            },
            /*
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
                msg: Msg::SetValue(Value::Inner(Inner {
                    kind: "if".to_string(),
                    children: HashMap::new(),
                })),
            },
            */
            Action {
                text: "Int".to_string(),
                msg: Msg::SetValue(Value::Int(0)),
            },
        ];
        let actions = actions
            .iter()
            .filter(|a| self.is_valid_action(a))
            .map(|a| self.view_action(a));

        let oninput = self
            .link
            .callback(move |e: InputData| Msg::SetText(e.value));
        let text = self.text.clone();
        let onclick = self
            .link
            .callback(move |_: MouseEvent| Msg::SetValue(Value::String(text.clone())));

        html! {
            <div>
            <input oninput=oninput></input>
            <div class="action" onclick=onclick>{ "Set Value" }</div>
            { for actions }
            </div>
        }
    }

    fn is_valid_action(&self, action: &Action) -> bool {
        match &action.msg {
            Msg::SetValue(new_value) => match self.cursor.back() {
                Some(selector) => {
                    let parent = self.lookup(&self.parent_ref().unwrap()).unwrap();
                    match &parent.value {
                        Value::Inner(v) => {
                            match &RUST_SCHEMA
                                .kinds
                                .iter()
                                .find(|k| k.name == v.kind)
                                .unwrap()
                                .fields
                                .iter()
                                .find(|f| f.name == selector.field)
                                .unwrap()
                                .type_
                            {
                                Type::String => {
                                    if let Value::String(_) = new_value {
                                        true
                                    } else {
                                        false
                                    }
                                }
                                Type::Bool => {
                                    if let Value::Bool(_) = new_value {
                                        true
                                    } else {
                                        false
                                    }
                                }
                                Type::Ref => {
                                    if let Value::Inner(_) = new_value {
                                        true
                                    } else {
                                        false
                                    }
                                }
                            }
                        }
                        _ => true,
                    }
                }
                None => true,
            },
            _ => true,
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
        let node = self.view_node(&file.root, &Path::new());
        html! {
            <pre>{ node }</pre>
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

    pub fn traverse_fields(kind: &str) -> Vec<&str> {
        match RUST_SCHEMA.kinds.iter().find(|k| k.name == kind) {
            Some(kind) => kind.fields.iter().map(|f| f.name).collect(),
            None => Vec::new(),
        }
        /*
        match kind {
            "document" => &["bindings"],
            "ref" => &["target"],
            "if" => &["condition", "true_body", "false_body"],
            "binary_operator" => &["left", "right"],
            "function_definition" => &["name", "arguments", "return_type", "body"],
            "struct" => &["name", "fields"],
            "enum" => &["name", "variants"],
            // "pattern" => &["name"],
            "function_call" => &["arguments"],
            _ => &[],
        }
        */
    }

    pub fn flatten_paths(&self, reference: &Ref, base: Path) -> Vec<Path> {
        log::info!("flatten: {:?} {:?}", reference, base);
        match &self.lookup(reference) {
            Some(node) => match &node.value {
                Value::Inner(v) => {
                    let mut paths = vec![];
                    for field in Model::traverse_fields(v.kind.as_ref()) {
                        match v.children.get(field) {
                            Some(children) => {
                                for (n, child) in children.iter().enumerate() {
                                    let new_base = append(
                                        &base,
                                        Selector {
                                            field: field.to_string(),
                                            index: Some(n),
                                        },
                                    );
                                    paths.push(new_base.clone());
                                    log::info!("child: {:?}[{:?}]->{:?}", reference, field, child);
                                    paths.extend(self.flatten_paths(child, new_base));
                                }
                            }
                            None => {
                                let new_base = append(
                                    &base,
                                    Selector {
                                        field: field.to_string(),
                                        index: None,
                                    },
                                );
                                paths.push(new_base.clone());
                            }
                        }
                        // paths.push(append(&base, crate::types::field(field)));
                        /*
                        for (n, child) in v
                            .children
                            .get(*field)
                            .cloned()
                            .unwrap_or_default()
                            .iter()
                            .enumerate()
                        {
                            let new_base = append(
                                &base,
                                Selector {
                                    field: field.to_string(),
                                    index: Some(n),
                                },
                            );
                            paths.push(new_base.clone());
                            log::info!("child: {:?}[{:?}]->{:?}", reference, field, child);
                            paths.extend(self.flatten_paths(child, new_base));
                        }
                        */
                    }
                    paths
                    // fields
                    //     .iter()
                    //     .map(|f| expand_field(v, f))
                    //     .flatten()
                    //     .map(|s| append(&base, s))
                    //     .collect()
                }
                _ => vec![],
            },
            None => vec![],
        }
    }

    fn view_node_list(&self, references: &[Ref], path: &Path) -> Html {
        // let sp = format!("{:?}", path);
        // let selected = match &cursor {
        //     Some(cursor) => cursor.is_empty(),
        //     None => false,
        // };
        let mut classes = vec!["node".to_string()];
        // if selected {
        //     classes.push("selected".to_string());
        // }
        let nodes = references.iter().enumerate().map(|(i, n)| {
            let mut path = path.clone();
            let l = path.len();
            path[l - 1].index = Some(i);
            self.view_node(n, &path)
        });
        let path = path.clone();
        let callback = self
            .link
            .callback(move |_: MouseEvent| Msg::Select(path.clone()));
        html! {
            <span onclick=callback>
                <span>{ "[" }</span>
                { for nodes }
                <span>{ "]" }</span>
            </span>
        }
    }

    // fn view_node(&self, reference: &Ref, path: Path, cursor: Option<Path>) -> Html {
    fn view_node(&self, reference: &Ref, path: &Path) -> Html {
        let selected = path == &self.cursor;
        let mut classes = vec!["node".to_string()];
        if selected {
            classes.push("selected".to_string());
        }
        let path_clone = path.clone();
        let callback = self
            .link
            .callback(move |_: MouseEvent| Msg::Select(path_clone.clone()));
        let value = match self.lookup(reference) {
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
                // if target {
                //     classes.push("target".to_string());
                // }
                self.view_value(&node.reference, &node.value, &path)
            }
            // <div class=classes.join(" ") onclick=callback path={sp}>
            None => {
                html! {
                        <span>{ "error" }</span>
                }
            }
        };
        html! {
            <div class=classes.join(" ") onclick=callback>
                { value }
            </div>
        }
    }

    fn callback_child<IN>(&self, path: &Path, child: Selector) -> Callback<IN> {
        let mut path = path.clone();
        path.push_back(child);
        self.link.callback(move |_: IN| Msg::Select(path.clone()))
    }

    // fn view_child(
    //     &self,
    //     value: &Value,
    //     path: &Path,
    //     cursor: &Option<Path>,
    //     selector: Selector,
    // ) -> Html {
    //     let path = append(&path, selector.clone());
    //     let cursor = sub_cursor(&cursor, selector.clone());
    //     match child(value, selector) {
    //         Some(Child::Single(reference)) => self.view_node(&reference, path, cursor),
    //         Some(Child::Multiple(references)) => self.view_node_list(&references, path, cursor),
    //         None => html! { <span>{ "???" }</span> },
    //     }
    // }

    fn view_child(&self, value: &Inner, field_name: &str, path: &Path) -> Html {
        let path = append(
            &path,
            Selector {
                field: field_name.to_string(),
                index: Some(0),
            },
        );
        // let cursor = sub_cursor(&cursor, field(field_name));
        match value.children.get(field_name).and_then(|v| v.get(0)) {
            Some(n) => self.view_node(n, &path),
            // Empty list vs hole vs special value?
            // TODO: How to traverse nested lists in preorder?
            None => self.view_node(&"-".to_string(), &path),
            // None => self.view_node_list(&[], &path),
        }
    }

    fn view_children(&self, value: &Inner, field_name: &str, path: &Path) -> Html {
        let path = append(&path, field(field_name));
        // let cursor = sub_cursor(&cursor, field(field_name));
        let empty = vec![];
        let children = value.children.get(field_name).unwrap_or(&empty);
        self.view_node_list(&children, &path)
    }

    fn view_field(&self, value: &Inner, field_name: &str, field_type: FieldType) {}

    fn validate(&self, value: &Value, field_name: &str) {}

    fn view_value(&self, reference: &Ref, value: &Value, path: &Path) -> Html {
        match value {
            Value::Hole => {
                html! { <span>{ "◆" }</span> }
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
            Value::Inner(v) => match v.kind.as_ref() {
                "document" => {
                    let bindings = self.view_children(&v, "bindings", &path);
                    html! {
                        <div>
                        { bindings }
                        </div>
                    }
                }
                "ref" => {
                    let target = self.view_child(&v, "target", &path);
                    html! {
                        <span>
                        { target }
                        </span>
                    }
                }
                "binary_operator" => {
                    let left = self.view_child(&v, "left", &path);
                    let right = self.view_child(&v, "right", &path);
                    html! {
                        <span>
                        { left }
                        { "***" }
                        { right }
                        </span>
                    }
                }
                "if" => {
                    let condition = self.view_child(&v, "condition", &path);
                    let true_body = self.view_child(&v, "true_body", &path);
                    let false_body = self.view_child(&v, "false_body", &path);
                    html! {
                        <span>
                        { "if" }{ condition }{ "{" }
                        { true_body }
                        { "}" }{ "else" }{ "{" }
                        { false_body }
                        { "}" }
                        </span>
                    }
                }
                "function_definition" => {
                    let label = self.view_child(&v, "name", &path);
                    let args = self.view_children(&v, "arguments", &path);
                    let body = self.view_child(&v, "body", &path);
                    let return_type = self.view_child(&v, "return_type", &path);
                    let async_ = self.view_child(&v, "async", &path);
                    let pub_ = self.view_child(&v, "pub", &path);

                    html! {
                        <span>
                        <div>{ "#" }</div>
                        { pub_ }
                        { "fn" }{ label }
                        { "(" }{ args }{ ")" }
                        { "->" }{ return_type }
                        { "{" }<div class="block">{ body }</div>{ "}" }
                        </span>
                    }
                }
                "struct" => {
                    let label = self.view_child(&v, "name", &path);
                    let fields = self.view_children(&v, "fields", &path);

                    html! {
                        <span>
                        { "struct" }{ label }
                        { "{" }{ fields }{ "}" }
                        </span>
                    }
                }
                "enum" => {
                    let label = self.view_child(&v, "name", &path);
                    let variants = self.view_children(&v, "variants", &path);

                    html! {
                        <span>
                        { "enum" }{ label }
                        { "{" }{ variants }{ "}" }
                        </span>
                    }
                }
                "pattern" => {
                    let name = self.lookup(&v.children["name"][0]).unwrap();
                    let name = match &name.value {
                        Value::String(v) => v.clone(),
                        _ => "error".to_string(),
                    };
                    html! {
                        <span>
                        { name }
                        </span>
                    }
                }
                "function_call" => {
                    let function = self.lookup(&v.children["function"][0]).unwrap();
                    let function_name = if let Value::Inner(v) = &function.value {
                        let name = self.lookup(&v.children["name"][0]).unwrap();
                        if let Value::String(v) = &name.value {
                            v.clone()
                        } else {
                            "error".to_string()
                        }
                    } else {
                        "error".to_string()
                    };
                    // let function_name = self.view_children(&v, "function");
                    // let function_name = "xxx";
                    // .and_then(|n| n.label())
                    // .map(|l| l.name.clone())
                    // .unwrap_or("<UNKNOWN>".to_string());
                    let args = self.view_children(&v, "arguments", path);
                    html! {
                        <span>
                        { function_name }
                        { "(" }{ args }{ ")" }
                        </span>
                    }
                }
                kind => {
                    html! { <span>{ kind }</span> }
                }
            },
            // Value::Ref(reference) => {
            //     let node = self.lookup(reference);
            //     let text = node
            //         .and_then(|n| n.label())
            //         .map(|l| l.name.clone())
            //         .unwrap_or("<UNKNOWN>".to_string());
            //     html! { <span>{ text }</span> }
            // }
            // Value::Binding(v) => {
            //     let label = self.view_label(reference, &v.label);
            //     let value = self.view_node(&v.value, path.clone(), cursor);
            //     let label_callback = self.callback_child(&path, field("label"));
            //     let value_callback = self.callback_child(&path, field("value"));
            //     html! {
            //         <span>
            //             <span onclick=label_callback>{ label }</span>
            //             { "=" }
            //             <span onclick=value_callback>{ value }</span>
            //         </span>
            //     }
            // }
            // Value::Pattern(v) => {
            //     html! {
            //         <span>
            //         { self.view_label(reference, &v.label) }
            //         </span>
            //     }
            // }
            // Value::Block(v) => {
            //     let expressions = v
            //         .expressions
            //         .iter()
            //         .map(|n| self.view_node(n, path.clone(), cursor.clone()));
            //     html! {
            //         <span>
            //         { "{" }
            //         { for expressions }
            //         { "}" }
            //         </span>
            //     }
            // }
            // Value::List(v) => {
            //     let items = v
            //         .items
            //         .iter()
            //         .map(|n| self.view_node(n, path.clone(), cursor.clone()));
            //     html! {
            //         <span>
            //         { "[" }{ for items }{ "]" }
            //         </span>
            //     }
            // }
            // Value::If(v) => {
            //     let conditional = self.view_child(value, &path, &cursor, field("condition"));
            //     let true_body = self.view_child(value, &path, &cursor, field("true_body"));
            //     let false_body = self.view_child(value, &path, &cursor, field("false_body"));
            //     html! {
            //         <span>
            //         { "if" }{ conditional }
            //         { "{" }<div class="block">{ true_body }</div>{ "}" }
            //         { "else" }
            //         { "{" }<div class="block">{ false_body }</div>{ "}" }
            //         </span>
            //     }
            // }
            // Value::FunctionDefinition(v) => {
            //     let label = self.view_label(reference, &v.label);

            //     let args = self.view_child(value, &path, &cursor, field("args"));
            //     let body = self.view_child(value, &path, &cursor, field("body"));
            //     let return_type = self.view_child(value, &path, &cursor, field("return_type"));

            //     html! {
            //         <span>
            //         <div>{ "#" }</div>
            //         { "fn" }{ label }
            //         { "(" }{ args }{ ")" }
            //         { "->" }{ return_type }
            //         { "{" }<div class="block">{ body }</div>{ "}" }
            //         </span>
            //     }
            // }
            // Value::FunctionCall(v) => {
            //     let node = self.file.lookup(&v.function);
            //     let function_name = node
            //         .and_then(|n| n.label())
            //         .map(|l| l.name.clone())
            //         .unwrap_or("<UNKNOWN>".to_string());
            //     let args = self.view_child(value, &path, &cursor, field("args"));
            //     html! {
            //         <span>
            //         { function_name }
            //         { "(" }{ args }{ ")" }
            //         </span>
            //     }
            // }
            // Value::BinaryOperator(v) => {
            //     let left = self.view_child(value, &path, &cursor, field("left"));
            //     let right = self.view_child(value, &path, &cursor, field("right"));
            //     html! {
            //         <span>
            //         { left }
            //         { &v.operator }
            //         { right }
            //         </span>
            //     }
            // }
        }
    }
}
