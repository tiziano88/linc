use crate::{
    schema::{
        default_renderer, Entry, Field, KindValue, Multiplicity, ValidationError, ValidatorContext,
        SCHEMA,
    },
    types::*,
};
use web_sys::MouseEvent;
use yew::prelude::*;

impl Model {
    pub fn view_actions(&self, ctx: &Context<Self>) -> Html {
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
            Action {
                image: None,
                text: "serialized".to_string(),
                msg: Msg::ToggleSerialized,
            },
        ];
        let actions = actions
            .iter()
            // .filter(|a| self.is_valid_action(a))
            .map(|a| self.view_action(ctx, a));

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
        path.split_last().and_then(|(selector, parent_path)| {
            let parent = self.file.lookup(&parent_path).unwrap();
            SCHEMA
                .get_kind(&parent.kind)
                .and_then(|f| f.get_field(&selector.field))
        })
    }

    fn view_action(&self, ctx: &Context<Self>, action: &Action) -> Html {
        let msg = action.msg.clone();
        let callback = ctx.link().callback(move |_: MouseEvent| msg.clone());
        let img = match &action.image {
            Some(image) => html! {
                <div class="inline-block">
                    <i class={ image }></i>
                </div>
            },
            None => html! {<span></span>},
        };
        html! {
            <button
              class="action hover:bg-blue-200 hover:text-blue-800 group flex items-center rounded-md bg-blue-100 text-blue-600 text-sm font-medium px-4 py-2"
              onclick={ callback }>
                { img }
                { &action.text }
            </button>
        }
    }

    pub fn view_file(&self, ctx: &Context<Self>, file: &File) -> Html {
        let node = self.view_node(ctx, Some(file.root.clone()), &Path::new());
        html! {
            <pre>{ node }</pre>
        }
    }

    pub fn view_file_json(&self, file: &File) -> Html {
        // let serialized = serde_json::to_string_pretty(file).expect("could not serialize to
        // JSON");
        let serialized = format!("root: {:?}\nfile: {:#?}", file.root, file);
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

    pub fn flatten_paths(&self, path: &[Selector]) -> Vec<Path> {
        // log::info!("flatten: {:?}", path);
        match &self.file.lookup(path) {
            Some(node) => {
                let mut paths = vec![];
                for field in Model::traverse_fields(&node) {
                    let mut children = node.children.get(field.name).cloned().unwrap_or_default();
                    match field.multiplicity {
                        Multiplicity::Single => {
                            if children.is_empty() {
                                children.push("".to_string());
                            }
                        }
                        Multiplicity::Repeated => {}
                    };
                    for (n, _child) in children.iter().enumerate() {
                        let new_path = append(
                            path,
                            Selector {
                                field: field.name.to_string(),
                                index: n,
                            },
                        );
                        paths.push(new_path.clone());
                        // log::info!("child: {:?}[{:?}]->{:?}", path, field.name, child);
                        paths.extend(self.flatten_paths(&new_path));
                    }
                    match field.multiplicity {
                        // If repeated field, stay on the parent.
                        Multiplicity::Repeated => {
                            let new_base = append(
                                path,
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

    pub fn view_node(&self, ctx: &Context<Self>, hash: Option<Hash>, path: &[Selector]) -> Html {
        self.view_node_with_placeholder(ctx, hash, path, "")
    }

    fn view_node_with_placeholder(
        &self,
        ctx: &Context<Self>,
        _hash: Option<Hash>,
        path: &[Selector],
        placeholder: &str,
    ) -> Html {
        let selected = path == &self.cursor;
        let mut classes = vec!["node".to_string(), "align-top".to_string()];
        if selected {
            classes.push("selected".to_string());
        }
        if path == &self.hover {
            // classes.push("hover".to_string());
        }

        let path_clone = path.to_vec();
        let onclick = ctx.link().callback(move |e: MouseEvent| {
            e.stop_propagation();
            Msg::Select(path_clone.clone())
        });

        let path_clone = path.to_vec();
        let onmouseover = ctx.link().callback(move |e: MouseEvent| {
            e.stop_propagation();
            Msg::Hover(path_clone.clone())
        });

        let path_clone = path.to_vec();
        let entries: Vec<Entry> = self
            .parsed_commands
            .iter()
            .map(|v| Entry {
                label: v.label.to_string(),
                description: "".to_string(),
                action: Msg::ReplaceNode(path_clone.clone(), v.to_node(), true),
            })
            .collect();

        let default_node = Node {
            ..Default::default()
        };
        let node = self.file.lookup(path).unwrap_or(&default_node);

        let context = ValidatorContext {
            model: self,
            ctx,
            path,
            node,
            entries: &entries,
            placeholder,
        };

        let (value, mut fields, errors) = match SCHEMA.get_kind(&node.kind) {
            Some(kind) => {
                let KindValue::Struct { fields, .. } = kind.value;
                (
                    kind.render(&context),
                    fields.to_vec(),
                    kind.validator(&context),
                )
            }
            None => (default_renderer(&context), vec![], vec![]),
        };

        if !selected {
            fields = vec![];
        }
        // Use onmousedown to avoid re-selecting the node?
        html! {
            <div class={ classes.join(" ") } onclick={ onclick } onmouseover={ onmouseover }>
                { value }
                { for fields.iter().map(|f| self.view_field(ctx, path, f)) }
                { for errors.iter().map(|e| self.view_error(e)) }
            </div>
        }
    }

    pub fn view_field(&self, ctx: &Context<Self>, path: &[Selector], field: &Field) -> Html {
        let path = path.to_vec();
        let field_name = field.name.to_string();
        let onclick = ctx.link().callback(move |e: MouseEvent| {
            e.stop_propagation();
            Msg::AddField(path.clone(), field_name.clone())
        });
        html! {
            <div class="field" onclick={ onclick }>{ format!("+ {}", field.name) }</div>
        }
    }

    pub fn view_error(&self, error: &ValidationError) -> Html {
        html! {
            <div class="error">{ format!("error: {:?}", error.message) }</div>
        }
    }

    pub fn view_child(
        &self,
        ctx: &Context<Self>,
        node: &Node,
        field_name: &str,
        path: &[Selector],
    ) -> Html {
        self.view_child_with_placeholder(ctx, node, field_name, path, field_name)
    }

    pub fn view_child_with_placeholder(
        &self,
        ctx: &Context<Self>,
        parent: &Node,
        field_name: &str,
        path: &[Selector],
        placeholder: &str,
    ) -> Html {
        let path = append(
            &path,
            Selector {
                field: field_name.to_string(),
                index: 0,
            },
        );
        let hash = parent
            .children
            .get(field_name)
            .and_then(|v| v.get(0))
            .cloned();
        // Empty list vs hole vs special value?
        // TODO: How to traverse nested lists in preorder?
        self.view_node_with_placeholder(ctx, hash, &path, placeholder)
    }

    /// Returns the head and children, separately.
    pub fn view_children(
        &self,
        ctx: &Context<Self>,
        parent: &Node,
        field_name: &str,
        path: &[Selector],
    ) -> (Html, Vec<Html>) {
        // let path = append(&path, field(field_name));
        // let cursor = sub_cursor(&cursor, field(field_name));
        let empty = vec![];
        let children = parent.children.get(field_name).unwrap_or(&empty);
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
            // let _onclick = self.link.callback(move |e: MouseEvent| {
            //     e.stop_propagation();
            //     Msg::Select(path_clone.clone())
            // });

            let path_clone = path.clone();
            // let _onmouseover = self.link.callback(move |e: MouseEvent| {
            //     e.stop_propagation();
            //     Msg::Hover(path_clone.clone())
            // });

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
            .map(|(i, h)| {
                let path = append(
                    &path,
                    Selector {
                        field: field_name.to_string(),
                        index: i,
                    },
                );
                self.view_node(ctx, Some(h.clone()), &path)
            })
            .chain(std::iter::once({
                let path = append(
                    &path,
                    Selector {
                        field: field_name.to_string(),
                        index: children.len(),
                    },
                );
                self.view_node_with_placeholder(ctx, None, &path, &format!("{}▷", field_name))
            }))
            .collect::<Vec<_>>();
        (head, nodes)
    }
}

pub fn command_input_id(path: &[Selector]) -> String {
    let mut id = String::new();
    for selector in path {
        id.push_str(&selector.field);
        id.push_str(&selector.index.to_string());
    }
    id
}
