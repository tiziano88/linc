use crate::{
    schema::{
        default_renderer, Field, KindValue, Multiplicity, ValidationError, ValidatorContext, SCHEMA,
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

    pub fn view_error(&self, error: &ValidationError) -> Html {
        html! {
            <div class="error">{ format!("error: {:?}", error.message) }</div>
        }
    }
}
