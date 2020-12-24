use yew::prelude::*;

use crate::{
    schema::{Field, Multiplicity, SCHEMA},
    types::*,
};

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
            Action {
                text: "x".to_string(),
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

    pub fn current_field(&self) -> Option<&Field> {
        match self.cursor.back() {
            Some(selector) => {
                let parent = self.lookup(&self.parent_ref().unwrap()).unwrap();
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
        match SCHEMA.get_kind(kind) {
            Some(kind) => kind.fields,
            None => &[],
        }
    }

    pub fn flatten_paths(&self, reference: &Ref, base: Path) -> Vec<Path> {
        log::info!("flatten: {:?} {:?}", reference, base);
        match &self.lookup(reference) {
            Some(node) => {
                let mut paths = vec![];
                for field in Model::traverse_fields(node.kind.as_ref()) {
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
                                index: Some(n),
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
                                    index: Some(children.len()),
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

    fn view_node_list(&self, references: &[Ref], path: &Path) -> (Html, Vec<Html>) {
        let head = {
            let mut path = path.clone();
            if path.len() > 0 {
                let path_len = path.len();
                path[path_len - 1].index = Some(references.len());
            }
            let selected = path == self.cursor;
            let mut classes = vec!["node".to_string()];
            if selected {
                classes.push("selected".to_string());
            }
            let path_clone = path.clone();
            let callback = self
                .link
                .callback(move |_: MouseEvent| Msg::Select(path_clone.clone()));
            html! {
                <div onclick=callback class=classes.join(" ")>{ "▷" }</div>
            }
        };

        let nodes = references
            .iter()
            .enumerate()
            .map(|(i, n)| {
                let mut path = path.clone();
                let l = path.len();
                path[l - 1].index = Some(i);
                self.view_node(n, &path)
            })
            .collect::<Vec<_>>();
        (head, nodes)
    }

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
                Some(node) => self.view_value(&node, &path),
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

    pub fn view_child(&self, node: &Node, field_name: &str, path: &Path) -> Html {
        let path = append(
            &path,
            Selector {
                field: field_name.to_string(),
                index: Some(0),
            },
        );
        match node.children.get(field_name).and_then(|v| v.get(0)) {
            Some(n) => self.view_node(n, &path),
            // Empty list vs hole vs special value?
            // TODO: How to traverse nested lists in preorder?
            None => self.view_node(&"-".to_string(), &path),
        }
    }

    /// Returns the head and children, separately.
    pub fn view_children(&self, node: &Node, field_name: &str, path: &Path) -> (Html, Vec<Html>) {
        let path = append(&path, field(field_name));
        // let cursor = sub_cursor(&cursor, field(field_name));
        let empty = vec![];
        let children = node.children.get(field_name).unwrap_or(&empty);
        self.view_node_list(&children, &path)
    }

    fn view_value(&self, node: &Node, path: &Path) -> Html {
        match SCHEMA.get_kind(&node.kind) {
            Some(kind) => (kind.renderer)(self, node, path),
            None => html! { <span>{ "unknown kind: " }{ node.kind.clone() }</span> },
        }
    }
}
