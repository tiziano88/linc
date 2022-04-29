use crate::{
    command_line::{CommandLine, Entry},
    model::{GlobalState, Model, Msg, TreeMsg, TreeState},
    schema::{
        default_renderer, Field, Kind, Schema, ValidatorContext, RUST_FUNCTION_CALL, SCHEMA, *,
    },
    types::{parent, Cursor, Hash, Link, LinkTarget, LinkType, Mode, Node, Selector},
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
    #[prop_or_default]
    pub placeholder: String,
    // When a new value is typed in the text box.
    #[prop_or_default]
    pub oninput: Callback<String>,
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
        let updatetree = ctx.props().updatetree.clone();
        Self {
            input_node_ref: NodeRef::default(),
            old_props: None,
            ondelete: Callback::from(move |()| updatetree.emit(TreeMsg::Parent)),
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, _first_render: bool) {
        let props = ctx.props();
        let global_state = &props.global_state;
        let tree_state = &props.tree_state;
        let cursor = &props.cursor;
        let selected = tree_state.selected_path == cursor.path();
        if selected && global_state.mode == Mode::Edit {
            self.focus_input();
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let props = ctx.props();
        let global_state = &props.global_state;
        let tree_state = &props.tree_state;
        let node_store = &global_state.node_store;
        let cursor = &props.cursor;
        let link = &cursor.link;
        let hash = &link.hash;
        let node_path = cursor.path();
        let _oninput = props.oninput.clone();
        let selected_path = &tree_state.selected_path;
        let selected = selected_path == &node_path;
        log::info!("cursor: {:?}", cursor);
        log::info!("link: {:?}", cursor.link);
        log::info!("target: {:?}", cursor.link.get(&node_store));
        let inner = match cursor.link.get(&node_store) {
            None => {
                let onupdatetree = ctx.props().updatetree.clone();
                let node_path = node_path.clone();
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
                        action: TreeMsg::ReplaceNode(
                            node_path.clone(),
                            create_node(kind_id),
                            false,
                        ),
                        valid_classes: KIND_CLASSES.iter().map(|v| v.to_string()).collect(),
                    })
                    .collect();
                let onenter = {
                    let onupdatetree = onupdatetree.clone();
                    Callback::from(move |()| {
                        onupdatetree.emit(TreeMsg::Parent);
                    })
                };
                let onupdatetree0 = onupdatetree.clone();
                let placeholder = "***".to_string();
                let value = "".to_string();
                html! {
                  <CommandLine
                    input_node_ref={ self.input_node_ref.clone() }
                    entries={ entries }
                    value={ value.clone() }
                    placeholder={ placeholder }
                    oninput={ Callback::from(move |v: String| {
                        onupdatetree.emit(TreeMsg::SetNodeValue(node_path.clone(), v.as_bytes().to_vec()));
                     }) }
                    onselect={ ctx.props().updatetree.clone() }
                    ondelete={ Callback::from(move |()| {
                        onupdatetree0.emit(TreeMsg::DeleteItem);
                     }) }
                    onenter={ onenter }
                    enabled={ selected && global_state.mode == Mode::Edit }
                  />
                }
            }
            Some(LinkTarget::Raw(value)) => {
                let onupdatetree = ctx.props().updatetree.clone();
                let node_path = node_path.clone();
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
                        action: TreeMsg::ReplaceNode(
                            node_path.clone(),
                            create_node(kind_id),
                            false,
                        ),
                        valid_classes: KIND_CLASSES.iter().map(|v| v.to_string()).collect(),
                    })
                    .collect();
                let onenter = {
                    let onupdatetree = onupdatetree.clone();
                    Callback::from(move |()| {
                        onupdatetree.emit(TreeMsg::Parent);
                    })
                };
                let onupdatetree0 = onupdatetree.clone();
                let placeholder = if value.is_empty() {
                    "***".to_string()
                } else {
                    "".to_string()
                };
                let value =
                    String::from_utf8(value.to_vec()).unwrap_or("INVALID STRING".to_string());
                html! {
                  <CommandLine
                    input_node_ref={ self.input_node_ref.clone() }
                    entries={ entries }
                    value={ value.clone() }
                    placeholder={ placeholder }
                    oninput={ Callback::from(move |v: String| {
                        onupdatetree.emit(TreeMsg::SetNodeValue(node_path.clone(), v.as_bytes().to_vec()));
                     }) }
                    onselect={ ctx.props().updatetree.clone() }
                    ondelete={ Callback::from(move |()| {
                        onupdatetree0.emit(TreeMsg::DeleteItem);
                     }) }
                    onenter={ onenter }
                    enabled={ selected && global_state.mode == Mode::Edit }
                  />
                }
            }
            Some(LinkTarget::Parsed(node)) => {
                let kind = SCHEMA.get_kind(&node.kind);
                let renderer = if global_state.rich_render {
                    kind.and_then(|k| k.renderer).unwrap_or(default_renderer)
                } else {
                    default_renderer
                };
                let validator_context = ValidatorContext {
                    global_state: global_state.clone(),
                    tree_state: tree_state.clone(),
                    cursor: cursor.clone(),
                    updatemodel: props.updatemodel.clone(),
                    updatetree: props.updatetree.clone(),
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
                                    action: TreeMsg::AddField(node_path.to_vec(), **field_id),
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
                                    action: TreeMsg::DeleteItem,
                                    valid_classes: vec![],
                                },
                                // TODO: should apply to literals too.
                                Entry {
                                    label: "call".to_string(),
                                    description: "".to_string(),
                                    action: TreeMsg::ReplaceNode(
                                        node_path.clone(),
                                        Node {
                                            kind: RUST_FUNCTION_CALL.to_string(),
                                            links: maplit::btreemap! {
                                                0 => vec![Link {
                                                    type_: LinkType::Raw,
                                                    hash: hash.clone(),
                                                }],
                                            },
                                        },
                                        false,
                                    ),
                                    valid_classes: vec![],
                                },
                                Entry {
                                    label: "array".to_string(),
                                    description: "".to_string(),
                                    action: TreeMsg::ReplaceNode(
                                        node_path.clone(),
                                        Node {
                                            kind: RUST_ARRAY_TYPE.to_string(),
                                            links: maplit::btreemap! {
                                                0 => vec![Link {
                                                    type_: LinkType::Raw,
                                                    hash: hash.clone(),
                                                }],
                                            },
                                        },
                                        false,
                                    ),
                                    valid_classes: vec![],
                                },
                                Entry {
                                    label: "move up".to_string(),
                                    description: "".to_string(),
                                    action: TreeMsg::ReplaceNode(
                                        parent(&node_path).to_vec(),
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
                    // TODO: What value to use?
                    let value = "".to_string();
                    html! {
                        <div class="pl-3 absolute bg-white">
                            <CommandLine
                                input_node_ref={ self.input_node_ref.clone() }
                                entries={ entries }
                                value={ value.clone() }
                                onselect={ ctx.props().updatetree.clone() }
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
            }
        };
        let onupdatetree = ctx.props().updatetree.clone();
        let onclick = {
            let node_path = node_path;
            ctx.link().callback(move |e: MouseEvent| {
                e.stop_propagation();
                onupdatetree.emit(TreeMsg::Select(node_path.clone()));
                NodeMsg::Click
            })
        };
        let mut classes = vec![
            "align-top",
            "flex-auto",
            "border-4",
            "cursor-default",
            "inline-block",
            "font-mono",
        ];
        if selected {
            classes.push("border-blue-500")
        }
        html! {
            <div
              class={ classes.join(" ") }
            //   tabindex="0"
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
        let new_props = ctx.props();
        log::debug!("Node changed {:?}", new_props.cursor);
        let same = if let Some(old_props) = &self.old_props {
            log::debug!(
                "same global_state: {:?}",
                old_props.global_state == new_props.global_state
            );
            log::debug!(
                "same tree_state: {:?}",
                old_props.tree_state == new_props.tree_state
            );
            log::debug!("same cursor: {:?}", old_props.cursor == new_props.cursor);
            log::debug!(
                "same updatemodel: {:?}",
                old_props.updatemodel == new_props.updatemodel
            );
            log::debug!("same oninput: {:?}", old_props.oninput == new_props.oninput);
            old_props.global_state == new_props.global_state
                || old_props.cursor == new_props.cursor
                || old_props.tree_state == new_props.tree_state
        } else {
            false
        };
        self.old_props = Some(new_props.clone());
        // !same
        true
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
