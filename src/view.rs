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
                text: "parent".to_string(),
                msg: Msg::Parent,
            },
            Action {
                text: "+item".to_string(),
                msg: Msg::AddItem,
            },
        ];
        let actions = actions
            .iter()
            // .filter(|a| self.is_valid_action(a))
            .map(|a| self.view_action(a));

        let oninput = self
            .link
            .callback(move |e: InputData| Msg::SetCommand(e.value));
        let onkeypress = self
            .link
            .callback(move |e: KeyboardEvent| Msg::CommandKey(e));

        let mut command_class = vec![
            "focus:border-blue-500",
            "focus:ring-1",
            "focus:ring-blue-500",
            "focus:outline-none",
            "text-sm",
            "text-black",
            "placeholder-gray-500",
            "border",
            "border-gray-200",
            "rounded-md",
            "py-2",
            "pl-10",
        ];

        if self.parsed_command.is_some() {
            command_class.push("bg-green-500")
        } else {
            command_class.push("bg-red-500")
        }

        html! {
            <div>
            <input
              class=command_class
              oninput=oninput
              onkeydown=onkeypress
              value=self.command
            ></input>
            { for actions }
            </div>
        }
    }

    fn is_valid_value(&self, new_value: &Value) -> bool {
        match self.cursor.back() {
            Some(selector) => {
                let parent = self.lookup(&self.parent_ref().unwrap()).unwrap();
                match &parent.value {
                    Value::Inner(v) => {
                        let field = &RUST_SCHEMA
                            .kinds
                            .iter()
                            .find(|k| k.name == v.kind)
                            .unwrap()
                            .fields
                            .iter()
                            .find(|f| f.name == selector.field)
                            .unwrap();
                        match field.type_ {
                            Type::String => {
                                if let Value::String(_) = new_value {
                                    (field.validator)(new_value)
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
                            Type::Ref => true,
                        }
                    }
                    _ => true,
                }
            }
            None => true,
        }
    }

    fn view_action(&self, action: &Action) -> Html {
        let msg = action.msg.clone();
        let callback = self.link.callback(move |_: MouseEvent| msg.clone());
        html! {
            <button
              class="action hover:bg-blue-200 hover:text-blue-800 group flex items-center rounded-md bg-blue-100 text-blue-600 text-sm font-medium px-4 py-2"
              onclick=callback>
                { &action.text }
            </button>
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

    pub fn traverse_fields(kind: &str) -> &[Field] {
        match RUST_SCHEMA.kinds.iter().find(|k| k.name == kind) {
            Some(kind) => kind.fields,
            None => &[],
        }
    }

    pub fn flatten_paths(&self, reference: &Ref, base: Path) -> Vec<Path> {
        log::info!("flatten: {:?} {:?}", reference, base);
        match &self.lookup(reference) {
            Some(node) => match &node.value {
                Value::Inner(v) => {
                    let mut paths = vec![];
                    for field in Model::traverse_fields(v.kind.as_ref()) {
                        match field.multiplicity {
                            // If repeated field, stay on the parent.
                            Multiplicity::Repeated => {
                                let new_base = append(
                                    &base,
                                    Selector {
                                        field: field.name.to_string(),
                                        index: None,
                                    },
                                );
                                paths.push(new_base.clone());
                            }
                            // If single field, skip directly to the first (and only) child.
                            Multiplicity::Single => {}
                        }
                        let mut children = v.children.get(field.name).cloned().unwrap_or_default();
                        match field.multiplicity {
                            Multiplicity::Single => {
                                if children.is_empty() {
                                    children.push("dummy".to_string());
                                }
                            }
                            Multiplicity::Repeated => {}
                        };
                        for (n, child) in children.iter().enumerate() {
                            let new_base = append(
                                &base,
                                Selector {
                                    field: field.name.to_string(),
                                    index: Some(n),
                                },
                            );
                            paths.push(new_base.clone());
                            log::info!("child: {:?}[{:?}]->{:?}", reference, field.name, child);
                            paths.extend(self.flatten_paths(child, new_base));
                        }
                    }
                    paths
                }
                _ => vec![],
            },
            None => vec![],
        }
    }

    fn view_node_list(&self, references: &[Ref], path: &Path) -> Vec<Html> {
        let selected = path == &self.cursor;
        let mut classes = vec!["node".to_string()];
        if selected {
            classes.push("selected".to_string());
        }
        let mut nodes = references
            .iter()
            .enumerate()
            .map(|(i, n)| {
                let mut path = path.clone();
                let l = path.len();
                path[l - 1].index = Some(i);
                self.view_node(n, &path)
            })
            .collect::<Vec<_>>();
        let path = path.clone();
        let callback = self
            .link
            .callback(move |_: MouseEvent| Msg::Select(path.clone()));
        let head = html! {
            <div onclick=callback class=classes.join(" ")>{ "▷" }</div>
        };
        nodes.insert(0, head);
        nodes
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
        let value = if reference == INVALID_REF {
            html! {
                <span>{ "◆" }</span>
            }
        } else {
            match self.lookup(reference) {
                Some(node) => self.view_value(&node.reference, &node.value, &path),
                None => {
                    html! {
                        <span>{ format!("invalid: {}", reference) }</span>
                    }
                }
            }
        };
        html! {
            <div class=classes.join(" ") onclick=callback>
                { value }
            </div>
        }
    }

    fn view_child(&self, value: &Inner, field_name: &str, path: &Path) -> Html {
        let path = append(
            &path,
            Selector {
                field: field_name.to_string(),
                index: Some(0),
            },
        );
        match value.children.get(field_name).and_then(|v| v.get(0)) {
            Some(n) => self.view_node(n, &path),
            // Empty list vs hole vs special value?
            // TODO: How to traverse nested lists in preorder?
            None => self.view_node(&"-".to_string(), &path),
            // None => self.view_node_list(&[], &path),
        }
    }

    fn view_children(&self, value: &Inner, field_name: &str, path: &Path) -> Vec<Html> {
        let path = append(&path, field(field_name));
        // let cursor = sub_cursor(&cursor, field(field_name));
        let empty = vec![];
        let children = value.children.get(field_name).unwrap_or(&empty);
        self.view_node_list(&children, &path)
    }

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
                    let bindings = self
                        .view_children(&v, "bindings", &path)
                        .into_iter()
                        .map(|b| {
                            html! {
                                <div>{ b }</div>
                            }
                        });
                    html! {
                        <div>
                        { for bindings }
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
                    let operator = self.view_child(&v, "operator", &path);
                    let left = self.view_child(&v, "left", &path);
                    let right = self.view_child(&v, "right", &path);
                    html! {
                        <span>
                        { left }
                        { operator }
                        { right }
                        </span>
                    }
                }
                "accessor" => {
                    let object = self.view_child(&v, "object", &path);
                    let field = self.view_child(&v, "field", &path);
                    html! {
                        <span>
                        { object }
                        { "." }
                        { field }
                        </span>
                    }
                }
                "binding" => {
                    let name = self.view_child(&v, "name", &path);
                    let value = self.view_child(&v, "value", &path);
                    html! {
                        <span>{ "let" }{ name }{ "=" }{ value }</span>
                    }
                }
                "qualify" => {
                    let parent = self.view_child(&v, "parent", &path);
                    let child = self.view_child(&v, "child", &path);
                    html! {
                        <span>{ parent }{ "::" }{ child }</span>
                    }
                }
                "if" => {
                    let condition = self.view_child(&v, "condition", &path);
                    let true_body = self.view_child(&v, "true_body", &path);
                    let false_body = self.view_child(&v, "false_body", &path);
                    html! {
                        <span>
                            <div>
                                { "if" }{ condition }{ "{" }
                            </div>
                            <div class="indent">
                                { true_body }
                            </div>
                            <div>
                                { "}" }{ "else" }{ "{" }
                            </div>
                            <div class="indent">
                                { false_body }
                            </div>
                            <div>
                                { "}" }
                            </div>
                        </span>
                    }
                }
                "function_definition" => {
                    let label = self.view_child(&v, "name", &path);
                    let args = self.view_children(&v, "arguments", &path);
                    let body = self.view_child(&v, "body", &path);
                    let return_type = self.view_child(&v, "return_type", &path);
                    // let async_ = self.view_child(&v, "async", &path);
                    // let pub_ = self.view_child(&v, "pub", &path);

                    html! {
                        <span>
                            <div>{ "#" }</div>
                            // { pub_ }
                            <div>{ "fn" }{ label }{ "(" }{ for args }{ ")" }{ "->" }{ return_type }{ "{" }</div>
                            <div class="indent">{ body }</div>{ "}" }
                        </span>
                    }
                }
                "struct" => {
                    let label = self.view_child(&v, "name", &path);
                    let fields = self
                        .view_children(&v, "fields", &path)
                        .into_iter()
                        .map(|v| {
                            html! {
                                <div class="indent">{ v }{ "," }</div>
                            }
                        });

                    html! {
                        <span>
                        { "struct" }{ label }
                        { "{" }{ for fields }{ "}" }
                        </span>
                    }
                }
                "string" => {
                    let value = self.view_child(&v, "value", &path);

                    html! {
                        <span>
                        { "\"" }{ value }{ "\"" }
                        </span>
                    }
                }
                "enum" => {
                    let label = self.view_child(&v, "name", &path);
                    let variants = self
                        .view_children(&v, "variants", &path)
                        .into_iter()
                        .map(|v| {
                            html! {
                                <div class="indent">{ v }{ "," }</div>
                            }
                        });

                    html! {
                        <span>
                        { "enum" }{ label }
                        { "{" }{ for variants }{ "}" }
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
                    let function = self.view_child(&v, "function", path);
                    // let function_name = self.view_children(&v, "function");
                    // let function_name = "xxx";
                    // .and_then(|n| n.label())
                    // .map(|l| l.name.clone())
                    // .unwrap_or("<UNKNOWN>".to_string());
                    let args = self.view_children(&v, "arguments", path);
                    html! {
                        <span>
                        { function }
                        { "(" }{ for args }{ ")" }
                        </span>
                    }
                }
                kind => {
                    html! { <span>{ kind }</span> }
                }
            },
        }
    }
}
