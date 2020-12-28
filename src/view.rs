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

    pub fn traverse_fields(node: &Node) -> &[Field] {
        let kind = &node.kind;
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
                index: 0,
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
                self.view_node(n, &path)
            })
            .collect::<Vec<_>>();
        (head, nodes)
    }

    fn view_value(&self, node: &Node, path: &Path) -> Html {
        match SCHEMA.get_kind(&node.kind) {
            Some(kind) => (kind.renderer)(self, node, path),
            None => html! { <span>{ "unknown kind: " }{ node.kind.clone() }</span> },
        }
    }
}
