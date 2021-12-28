use crate::{
    command_line::{CommandLine, Entry},
    schema::{FieldValidator, KindValue, SCHEMA},
    types::{append, File, Hash, Msg, Node, Selector},
};
use std::{collections::BTreeMap, rc::Rc};
use web_sys::HtmlInputElement;
use yew::prelude::*;

pub struct NodeComponent {
    input_node_ref: NodeRef,
}

#[derive(Properties, PartialEq)]
pub struct NodeProperties {
    pub path: Vec<Selector>,
    pub cursor: Vec<Selector>,
    #[prop_or_default]
    pub hash: Option<Hash>,
    #[prop_or_default]
    pub placeholder: String,
    pub file: Rc<File>,
    // When a new value is typed in the text box.
    #[prop_or_default]
    pub oninput: Callback<String>,
    // When the node is selected (e.g. clicked).
    #[prop_or_default]
    pub onselect: Callback<Vec<Selector>>,
    #[prop_or_default]
    pub updatemodel: Callback<Msg>,
    #[prop_or_default]
    pub validators: &'static [FieldValidator],
}

pub enum NodeMsg {
    Noop,
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
const KIND_CLASSES: &[&str] = &[
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
        Self {
            input_node_ref: NodeRef::default(),
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        let props = ctx.props();
        let path = props.path.clone();
        let cursor = props.cursor.clone();
        let selected = path == cursor;
        if selected {
            self.focus_input();
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let default_node = Node::default();
        let props = ctx.props();
        let hash = props.hash.clone().unwrap_or_default();
        let node = props.file.lookup_hash(&hash).unwrap_or(&default_node);
        let path = props.path.clone();
        let cursor = props.cursor.clone();
        let oninput = props.oninput.clone();
        let selected = path == cursor;
        let kind = SCHEMA.get_kind(&node.kind);
        let inner = if node.kind.is_empty() {
            let onupdatemodel = ctx.props().updatemodel.clone();
            let path = path.clone();
            let entries: Vec<Entry> = props
                .validators
                .iter()
                .flat_map(|v| match v {
                    FieldValidator::Kind(k) => Some(Entry {
                        label: k.to_string(),
                        description: "".to_string(),
                        action: Msg::ReplaceNode(
                            path.clone(),
                            Node {
                                kind: k.to_string(),
                                value: "".to_string(),
                                children: BTreeMap::new(),
                            },
                            false,
                        ),
                        valid_classes: KIND_CLASSES.iter().map(|v| v.to_string()).collect(),
                    }),
                    FieldValidator::Literal(v) => None,
                })
                .collect();
            log::info!("{:?}", entries);
            let onenter = {
                let path = path.clone();
                let onupdatemodel = onupdatemodel.clone();
                ctx.link().callback(move |()| {
                    onupdatemodel.emit(Msg::Parent);
                    NodeMsg::Noop
                })
            };
            html! {
              <CommandLine
                input_node_ref={ self.input_node_ref.clone() }
                entries={ entries }
                value={ node.value.clone() }
                oninput={ ctx.link().callback(move |v| {
                    onupdatemodel.emit(Msg::SetNodeValue(path.clone(), v));
                    NodeMsg::Noop
                 }) }
                onselect={ ctx.props().updatemodel.clone() }
                onenter={ onenter }
                enabled={ selected }
              />
            }
        } else {
            // Node.
            // https://codepen.io/xotonic/pen/JRLAOR
            let children: Vec<_> = if false {
                node.children
                    .iter()
                    .map(|(field_name, hashes)| {
                        let field_schema = kind.and_then(|k| k.get_field(field_name));
                        let validators = field_schema
                            .map(|v| v.validators.clone())
                            .unwrap_or_default();
                        let values = hashes.iter().enumerate().map(|(i, h)| {
                            let child_path = append(
                                &path,
                                Selector {
                                    field: field_name.clone(),
                                    index: i,
                                },
                            );
                            html! {
                                <div class="px-6">
                                    <NodeComponent
                                      file={ ctx.props().file.clone() }
                                      hash={ h.clone() }
                                      cursor={ ctx.props().cursor.clone() }
                                      onselect={ ctx.props().onselect.clone() }
                                      path={ child_path }
                                      updatemodel={ ctx.props().updatemodel.clone() }
                                      validators={ validators }
                                    />
                                </div>
                            }
                        });
                        html! {
                            <span class="px-1">
                                <div class="sticky bg-gray-300 mx-3" >
                                    { field_name }
                                </div>
                                <div>{ for values }</div>
                            </span>
                        }
                    })
                    .collect()
            } else {
                node.children
                    .iter()
                    .flat_map(|(field_name, hashes)| {
                        let field_schema = kind.and_then(|k| k.get_field(field_name));
                        let validators = field_schema
                            .map(|v| v.validators.clone())
                            .unwrap_or_default();
                        let path = path.clone();
                        hashes.iter().enumerate().map(move |(i, h)| {
                            let child_path = append(
                                &path,
                                Selector {
                                    field: field_name.clone(),
                                    index: i,
                                },
                            );
                            let onclick = {
                                let onselect = props.onselect.clone();
                                let child_path = child_path.clone();
                                ctx.link().callback(move |e: MouseEvent| {
                                    log::info!("click");
                                    e.stop_propagation();
                                    onselect.emit(child_path.clone());
                                    NodeMsg::Noop
                                })
                            };
                            // TODO: Sticky field headers.
                            html! {
                                <div class="px-6"
                                  onclick={ onclick }
                                >
                                    <div class={ FIELD_CLASSES.join(" ") }>
                                        { field_name }
                                    </div>
                                    <div class="inline-block">
                                        { ":" }
                                    </div>
                                    <NodeComponent
                                      file={ ctx.props().file.clone() }
                                      hash={ h.clone() }
                                      cursor={ ctx.props().cursor.clone() }
                                      onselect={ ctx.props().onselect.clone() }
                                      path={ child_path }
                                      updatemodel={ ctx.props().updatemodel.clone() }
                                      validators={ validators }
                                    />
                                </div>
                            }
                        })
                    })
                    .collect()
            };
            let entries: Vec<Entry> = kind
                .map(|k| {
                    let KindValue::Struct { fields, .. } = k.value;
                    fields
                        .iter()
                        .map(|f| Entry {
                            label: f.name.to_string(),
                            description: "".to_string(),
                            action: Msg::AddField(path.to_vec(), f.name.to_string()),
                            valid_classes: FIELD_CLASSES.iter().map(|v| v.to_string()).collect(),
                        })
                        .collect()
                })
                .unwrap_or_default();
            let edit = if selected {
                // Make it look like an actual field.
                html! {
                    <div class="px-6">
                        <CommandLine
                            input_node_ref={ self.input_node_ref.clone() }
                            entries={ entries }
                            value={ node.value.clone() }
                            onselect={ ctx.props().updatemodel.clone() }
                            enabled=true
                        />
                    </div>
                }
            } else {
                html! {}
            };
            html! {
              <>
                <div class={ KIND_CLASSES.join(" ") }>
                    { node.kind.clone() }
                </div>
                <div class="inline-block text-xs border border-black">
                    { ctx.props().hash.clone().unwrap_or("-".to_string()) }
                </div>
                <div class="divide-y divide-black border-t border-b border-black border-solid">
                    { for children }
                </div>
                { edit }
              </>
            }
        };
        let onselect = ctx.props().onselect.clone();
        let onclick = {
            let path = path.clone();
            ctx.link().callback(move |e: MouseEvent| {
                e.stop_propagation();
                onselect.emit(path.clone());
                NodeMsg::Click
            })
        };
        let mut classes = vec!["node".to_string(), "align-top".to_string()];
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

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            NodeMsg::Noop => false,
            NodeMsg::Click => {
                self.focus_input();
                true
            }
        }
    }
}

impl NodeComponent {
    fn focus_input(&self) {
        let input_node = self.input_node_ref.clone();
        input_node
            .cast::<HtmlInputElement>()
            .unwrap()
            .focus()
            .unwrap();
    }
}
