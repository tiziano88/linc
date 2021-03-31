use yew::prelude::*;

use crate::{
    schema::{Field, Multiplicity, SCHEMA},
    types::*,
};

impl Model {
    pub fn view_actions(&self) -> Html {
        let actions = vec![
            Action {
                image: None,
                text: "store".to_string(),
                msg: Msg::Store,
            },
            Action {
                image: None,
                text: "load".to_string(),
                msg: Msg::Load,
            },
            Action {
                image: None,
                text: "Normal mode".to_string(),
                msg: Msg::SetMode(Mode::Normal),
            },
            Action {
                image: None,
                text: "Edit mode".to_string(),
                msg: Msg::SetMode(Mode::Edit),
            },
            Action {
                image: Some("gg-arrow-left".to_string()),
                text: "prev".to_string(),
                msg: Msg::Prev,
            },
            Action {
                image: Some("gg-arrow-right".to_string()),
                text: "next".to_string(),
                msg: Msg::Next,
            },
            Action {
                image: Some("gg-corner-right-up".to_string()),
                text: "parent".to_string(),
                msg: Msg::Parent,
            },
            Action {
                image: Some("gg-corner-double-up-right".to_string()),
                text: "+item".to_string(),
                msg: Msg::AddItem,
            },
            Action {
                image: Some("gg-close".to_string()),
                text: "delete".to_string(),
                msg: Msg::DeleteItem,
            },
        ];
        let actions = actions
            .iter()
            // .filter(|a| self.is_valid_action(a))
            .map(|a| self.view_action(a));

        html! {
            <div>
                { for actions }
            </div>
        }
    }

    pub fn current_field(&self) -> Option<Field> {
        self.field(&self.cursor)
    }

    pub fn field(&self, path: &Path) -> Option<Field> {
        let parent_path = parent(&self.cursor);

        match path.back() {
            Some(selector) => {
                let parent_ref = self.lookup_path(&self.file.root, &parent_path).unwrap();
                let parent = self.lookup(&parent_ref).unwrap();
                SCHEMA
                    .get_kind(&parent.kind)
                    .unwrap()
                    .get_field(&selector.field)
            }
            None => None,
        }
    }

    fn view_action(&self, action: &Action) -> Html {
        let msg = action.msg.clone();
        let callback = self.link.callback(move |_: MouseEvent| msg.clone());
        let img = match &action.image {
            Some(image) => html! {
                <div class="inline-block">
                    <i class=image></i>
                </div>
            },
            None => html! {<span></span>},
        };
        html! {
            <button
              class="action hover:bg-blue-200 hover:text-blue-800 group flex items-center rounded-md bg-blue-100 text-blue-600 text-sm font-medium px-4 py-2"
              onclick=callback>
                { img }
                { &action.text }
            </button>
        }
    }

    pub fn view_file(&self, file: &File) -> Html {
        let node = self.view_node(Some(file.root.clone()), &Path::new());
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

    pub fn traverse_fields(node: &Node) -> Vec<Field> {
        let kind = &node.kind;
        match SCHEMA.get_kind(kind) {
            Some(kind) => kind.get_fields(),
            None => vec![],
        }
    }

    pub fn flatten_paths(&self, reference: &Ref, base: Path) -> Vec<Path> {
        log::info!("flatten: {:?} {:?}", reference, base);
        match &self.lookup(reference) {
            Some(node) => {
                let mut paths = vec![];
                for field in Model::traverse_fields(&node) {
                    let mut children = node.children.get(field.name).cloned().unwrap_or_default();
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
                                index: n,
                            },
                        );
                        paths.push(new_base.clone());
                        log::info!("child: {:?}[{:?}]->{:?}", reference, field.name, child);
                        paths.extend(self.flatten_paths(child, new_base));
                    }
                    match field.multiplicity {
                        // If repeated field, stay on the parent.
                        Multiplicity::Repeated => {
                            let new_base = append(
                                &base,
                                Selector {
                                    field: field.name.to_string(),
                                    index: children.len(),
                                },
                            );
                            paths.push(new_base.clone());
                        }
                        // If single field, skip directly to the first (and only) child.
                        Multiplicity::Single => {}
                    }
                }
                paths
            }
            None => vec![],
        }
    }

    fn view_node(&self, reference: Option<Ref>, path: &Path) -> Html {
        self.view_node_with_placeholder(reference, path, "◆")
    }

    fn view_node_with_placeholder(
        &self,
        reference: Option<Ref>,
        path: &Path,
        placeholder: &str,
    ) -> Html {
        let mut classes = vec!["node".to_string()];
        if path == &self.cursor {
            classes.push("selected".to_string());
        }
        if path == &self.hover {
            classes.push("hover".to_string());
        }

        let path_clone = path.clone();
        let onclick = self.link.callback(move |e: MouseEvent| {
            e.stop_propagation();
            Msg::Select(path_clone.clone())
        });

        let path_clone = path.clone();
        let onmouseover = self.link.callback(move |e: MouseEvent| {
            e.stop_propagation();
            Msg::Hover(path_clone.clone())
        });

        let path_clone = path.clone();
        let oninput = self.link.callback(move |e: InputData| {
            crate::types::Msg::SetNodeCommand(path_clone.clone(), e.value.clone())
        });

        let node_state = self.node_state.get(&path);

        let value = match reference {
            Some(reference) => match self.lookup(&reference) {
                Some(node) => self.view_value(&node, &path),
                None => {
                    html! {
                        <span>{ format!("invalid: {}", reference) }</span>
                    }
                }
            },
            None => {
                let suggestions: Vec<_> = node_state
                    .map(|v| v.parsed_commands.clone())
                    .unwrap_or_default()
                    .iter()
                    .map(|v| {
                        let path_clone = path.clone();
                        let value_string = v.value.clone().unwrap_or_default();
                        let node = v.to_node();
                        let onclick = self.link.callback(move |e: MouseEvent| match node.clone() {
                            Some(node) => Msg::ReplaceNode(path_clone.clone(), node.clone()),
                            None => Msg::Noop,
                        });
                        let classes_item = vec!["block", "border"];
                        html! {
                            <span class=classes_item.join(" ") onclick=onclick>{value_string}</span>
                        }
                    })
                    .collect();
                let classes_dropdown = vec!["absolute", "z-10", "bg-white"];
                html! {
                    <span>
                        <div class="placeholder">{ placeholder }</div>
                        <span>
                            <span class="inline-block w-full" contenteditable=true oninput=oninput>{""}</span>
                            <div class=classes_dropdown.join(" ")>
                                { for suggestions }
                            </div>
                        </span>
                    </span>
                }
            }
        };
        html! {
            <div class=classes.join(" ") onclick=onclick onmouseover=onmouseover>
                { value }
            </div>
        }
    }

    pub fn view_child(&self, node: &Node, field_name: &str, path: &Path) -> Html {
        self.view_child_with_placeholder(node, field_name, path, field_name)
    }

    pub fn view_child_with_placeholder(
        &self,
        node: &Node,
        field_name: &str,
        path: &Path,
        placeholder: &str,
    ) -> Html {
        let path = append(
            &path,
            Selector {
                field: field_name.to_string(),
                index: 0,
            },
        );
        let reference = node
            .children
            .get(field_name)
            .and_then(|v| v.get(0))
            .cloned();
        // Empty list vs hole vs special value?
        // TODO: How to traverse nested lists in preorder?
        self.view_node_with_placeholder(reference, &path, placeholder)
    }

    /// Returns the head and children, separately.
    pub fn view_children(&self, node: &Node, field_name: &str, path: &Path) -> (Html, Vec<Html>) {
        // let path = append(&path, field(field_name));
        // let cursor = sub_cursor(&cursor, field(field_name));
        let empty = vec![];
        let children = node.children.get(field_name).unwrap_or(&empty);
        let head = {
            let path = append(
                &path,
                Selector {
                    field: field_name.to_string(),
                    index: children.len(),
                },
            );
            let mut classes = vec!["node".to_string(), "placeholder".to_string()];
            if path == self.cursor {
                classes.push("selected".to_string());
            }
            if path == self.hover {
                classes.push("hover".to_string());
            }

            let path_clone = path.clone();
            let onclick = self.link.callback(move |e: MouseEvent| {
                e.stop_propagation();
                Msg::Select(path_clone.clone())
            });

            let path_clone = path.clone();
            let onmouseover = self.link.callback(move |e: MouseEvent| {
                e.stop_propagation();
                Msg::Hover(path_clone.clone())
            });

            // html! {
            //     <div onclick=onclick onmouseover=onmouseover class=classes.join(" ")>{ field_name
            // }{ "▷" }</div> }
            html! {
                <div></div>
            }
        };

        let nodes = children
            .iter()
            .enumerate()
            .map(|(i, n)| {
                let path = append(
                    &path,
                    Selector {
                        field: field_name.to_string(),
                        index: i,
                    },
                );
                self.view_node(Some(n.clone()), &path)
            })
            .chain(std::iter::once({
                let path = append(
                    &path,
                    Selector {
                        field: field_name.to_string(),
                        index: children.len(),
                    },
                );
                self.view_node_with_placeholder(None, &path, &format!("{}▷", field_name))
            }))
            .collect::<Vec<_>>();
        (head, nodes)
    }

    fn view_value(&self, node: &Node, path: &Path) -> Html {
        match SCHEMA.get_kind(&node.kind) {
            Some(kind) => kind.render(self, node, path),
            None => html! { <span>{ "unknown kind: " }{ node.kind.clone() }</span> },
        }
    }
}
