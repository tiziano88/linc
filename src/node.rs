use crate::{
    command_line::{CommandLine, Entry},
    model::{GlobalState, Model, Msg},
    schema::{
        default_renderer, Field, Kind, Schema, ValidatorContext, RUST_FUNCTION_CALL, SCHEMA, *,
    },
    types::{parent, Cursor, Hash, Mode, Node, Selector},
};
use std::{collections::BTreeMap, rc::Rc};
use web_sys::HtmlInputElement;
use yew::prelude::*;

pub struct NodeComponent {
    input_node_ref: NodeRef,
    old_props: Option<NodeProperties>,
    // Memoize callbacks?
    ondelete: Callback<()>,
}

#[derive(Properties, PartialEq, Clone)]
pub struct NodeProperties {
    pub global_state: Rc<GlobalState>,
    pub cursor: Cursor,
    pub selected_path: Vec<Selector>,
    #[prop_or_default]
    pub placeholder: String,
    // When a new value is typed in the text box.
    #[prop_or_default]
    pub oninput: Callback<String>,
    // When the node is selected (e.g. clicked).
    #[prop_or_default]
    pub onselect: Callback<Vec<Selector>>,
    #[prop_or_default]
    pub updatemodel: Callback<Msg>,
    #[prop_or_default]
    pub allowed_kinds: &'static [&'static str],
}

pub enum NodeMsg {
    Click,
}

pub const FIELD_CLASSES: &[&str] = &[
    "bg-blue-300",
    "inline-block",
    "p-0.5",
    "px-1",
    "border-blue-800",
    "border",
    "rounded",
];
pub const KIND_CLASSES: &[&str] = &[
    "kind",
    "bg-yellow-400",
    "border-yellow-800",
    "border-4",
    "inline-block",
    "p-0.5",
    "px-1",
];

impl Component for NodeComponent {
    type Message = NodeMsg;
    type Properties = NodeProperties;

    fn create(ctx: &Context<Self>) -> Self {
        log::debug!("creating node");
        let updatemodel = ctx.props().updatemodel.clone();
        Self {
            input_node_ref: NodeRef::default(),
            old_props: None,
            ondelete: Callback::from(move |()| updatemodel.emit(Msg::Parent)),
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, _first_render: bool) {
        let props = ctx.props();
        let global_state = &props.global_state;
        let cursor = &props.cursor;
        let selected = props.selected_path == cursor.path();
        if selected && global_state.mode == Mode::Edit {
            self.focus_input();
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let default_node = Node::default();
        let props = ctx.props();
        let global_state = &props.global_state;
        let node_store = &global_state.node_store;
        let cursor = &props.cursor;
        let hash = &cursor.hash;
        let node = cursor.node(&node_store).unwrap_or(&default_node);
        let path = cursor.path();
        let _oninput = props.oninput.clone();
        let selected_path = &props.selected_path;
        let selected = selected_path == &cursor.path();
        let kind = SCHEMA.get_kind(&node.kind);
        let inner = if hash.is_empty() || node.kind.is_empty() {
            let onupdatemodel = ctx.props().updatemodel.clone();
            let path = path.clone();
            let entries: Vec<Entry> = props
                .allowed_kinds
                .iter()
                .map(|kind_id| Entry {
                    label: SCHEMA
                        .get_kind(kind_id)
                        .map(|k| &(*k.name))
                        .unwrap_or("INVALID")
                        .to_string(),
                    description: "".to_string(),
                    action: Msg::ReplaceNode(
                        path.clone(),
                        Node {
                            kind: kind_id.to_string(),
                            value: "".to_string(),
                            links: BTreeMap::new(),
                        },
                        false,
                    ),
                    valid_classes: KIND_CLASSES.iter().map(|v| v.to_string()).collect(),
                })
                .collect();
            let onenter = {
                let _path = path.clone();
                let onupdatemodel = onupdatemodel.clone();
                Callback::from(move |()| {
                    onupdatemodel.emit(Msg::Parent);
                })
            };
            let onupdatemodel0 = onupdatemodel.clone();
            let placeholder = if hash.is_empty() {
                "***".to_string()
            } else {
                "".to_string()
            };
            html! {
              <CommandLine
                input_node_ref={ self.input_node_ref.clone() }
                entries={ entries }
                value={ node.value.clone() }
                placeholder={ placeholder }
                oninput={ Callback::from(move |v| {
                    onupdatemodel.emit(Msg::SetNodeValue(path.clone(), v));
                 }) }
                onselect={ ctx.props().updatemodel.clone() }
                ondelete={ Callback::from(move |()| {
                    onupdatemodel0.emit(Msg::DeleteItem);
                 }) }
                onenter={ onenter }
                enabled={ selected && global_state.mode == Mode::Edit }
              />
            }
        } else {
            let renderer = if global_state.rich_render {
                kind.and_then(|k| k.renderer).unwrap_or(default_renderer)
            } else {
                default_renderer
            };
            let validator_context = ValidatorContext {
                global_state: global_state.clone(),
                selected_path: selected_path.clone(),
                cursor: cursor.clone(),
                onselect: props.onselect.clone(),
                updatemodel: props.updatemodel.clone(),
            };
            let content = renderer(&validator_context);
            let footer = if global_state.mode == Mode::Edit && selected {
                let entries: Vec<Entry> = kind
                    .map(|k| {
                        let mut all_entries = vec![];
                        let mut field_entries = k
                            .get_fields()
                            .iter()
                            .map(|(field_id, field)| Entry {
                                label: field.name.to_string(),
                                description: "".to_string(),
                                action: Msg::AddField(path.to_vec(), **field_id),
                                valid_classes: FIELD_CLASSES
                                    .iter()
                                    .map(|v| v.to_string())
                                    .collect(),
                            })
                            .collect();
                        // TODO: macros
                        let mut macro_entries = vec![
                            Entry {
                                label: "delete".to_string(),
                                description: "".to_string(),
                                action: Msg::DeleteItem,
                                valid_classes: vec![],
                            },
                            // TODO: should apply to literals too.
                            Entry {
                                label: "call".to_string(),
                                description: "".to_string(),
                                action: Msg::ReplaceNode(
                                    path.clone(),
                                    Node {
                                        kind: RUST_FUNCTION_CALL.to_string(),
                                        value: "".to_string(),
                                        links: maplit::btreemap! {
                                            0 => vec![hash.clone()],
                                        },
                                    },
                                    false,
                                ),
                                valid_classes: vec![],
                            },
                            Entry {
                                label: "array".to_string(),
                                description: "".to_string(),
                                action: Msg::ReplaceNode(
                                    path.clone(),
                                    Node {
                                        kind: RUST_ARRAY_TYPE.to_string(),
                                        value: "".to_string(),
                                        links: maplit::btreemap! {
                                            0 => vec![hash.clone()],
                                        },
                                    },
                                    false,
                                ),
                                valid_classes: vec![],
                            },
                            Entry {
                                label: "move up".to_string(),
                                description: "".to_string(),
                                action: Msg::ReplaceNode(
                                    parent(&path).to_vec(),
                                    node.clone(),
                                    false,
                                ),
                                valid_classes: vec![],
                            },
                        ];
                        all_entries.append(&mut field_entries);
                        all_entries.append(&mut macro_entries);
                        all_entries
                    })
                    .unwrap_or_default();
                // Make it look like an actual field.
                let _onupdatemodel0 = ctx.props().updatemodel.clone();
                html! {
                    <div class="pl-3 absolute bg-white">
                        <CommandLine
                            input_node_ref={ self.input_node_ref.clone() }
                            entries={ entries }
                            value={ node.value.clone() }
                            onselect={ ctx.props().updatemodel.clone() }
                            ondelete={ self.ondelete.clone() }
                            enabled=true
                        />
                    </div>
                }
            } else {
                html! {}
            };
            html! {
              <>
                { content }
                { footer }
              </>
            }
        };
        let onselect = ctx.props().onselect.clone();
        let onclick = {
            let path = path;
            ctx.link().callback(move |e: MouseEvent| {
                e.stop_propagation();
                onselect.emit(path.clone());
                NodeMsg::Click
            })
        };
        let mut classes = vec![
            "node".to_string(),
            "align-top".to_string(),
            "flex-auto".to_string(),
        ];
        if selected {
            classes.push("selected".to_string());
        }
        html! {
            <div
              class={ classes.join(" ") }
              tabindex="0"
              onclick={ onclick }
            >
              { inner }
            </div>
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            NodeMsg::Click => {
                self.focus_input();
                true
            }
        }
    }

    fn changed(&mut self, ctx: &Context<Self>) -> bool {
        let same = false;
        log::debug!("Node changed");
        log::debug!("same props: {:?}", same);
        if let Some(old_props) = &self.old_props {
            log::debug!(
                "same global_state: {:?}",
                old_props.global_state == ctx.props().global_state
            );
            log::debug!("same cursor: {:?}", old_props.cursor == ctx.props().cursor);
            log::debug!(
                "same selected_path: {:?}",
                old_props.selected_path == ctx.props().selected_path
            );
            log::debug!(
                "same updatemodel: {:?}",
                old_props.updatemodel == ctx.props().updatemodel
            );
            return old_props.global_state != ctx.props().global_state
                || old_props.cursor != ctx.props().cursor
                || old_props.selected_path != ctx.props().selected_path;
        }
        self.old_props = Some(ctx.props().clone());
        !same
    }
}

impl NodeComponent {
    fn focus_input(&self) {
        log::info!("focusing input");
        let input_node = self.input_node_ref.clone();
        if let Some(i) = input_node.cast::<HtmlInputElement>() {
            i.focus().unwrap();
        }
    }
}
