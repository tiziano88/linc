use crate::{
    command_line::{CommandLine, Entry},
    schema::{default_renderer, FieldValidator, KindValue, ValidatorContext, SCHEMA},
    types::{append, File, Hash, Model, Msg, Node, Selector},
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
    pub model: Rc<Model>,
    #[prop_or_default]
    pub hash: Option<Hash>,
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
    pub validators: &'static [FieldValidator],
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
        log::debug!("creating node");
        Self {
            input_node_ref: NodeRef::default(),
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        let props = ctx.props();
        let path = props.path.clone();
        let cursor = props.model.cursor.clone();
        let selected = path == cursor;
        if selected {
            self.focus_input();
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        log::debug!("rendering node");
        let default_node = Node::default();
        let props = ctx.props();
        let hash = props.hash.clone().unwrap_or_default();
        let node = props.model.file.lookup_hash(&hash).unwrap_or(&default_node);
        let path = props.path.clone();
        let cursor = props.model.cursor.clone();
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
                Callback::from(move |()| {
                    onupdatemodel.emit(Msg::Parent);
                })
            };
            html! {
              <CommandLine
                input_node_ref={ self.input_node_ref.clone() }
                entries={ entries }
                value={ node.value.clone() }
                oninput={ Callback::from(move |v| {
                    onupdatemodel.emit(Msg::SetNodeValue(path.clone(), v));
                 }) }
                onselect={ ctx.props().updatemodel.clone() }
                onenter={ onenter }
                enabled={ selected }
              />
            }
        } else {
            let renderer = kind
                .map(|r| {
                    let KindValue::Struct { renderer, .. } = r.value;
                    renderer
                })
                .unwrap_or(default_renderer);
            // TODO: Disable default renderer.
            let renderer = default_renderer;
            let validator_context = ValidatorContext {
                model: props.model.clone(),
                path: path.clone(),
                node: node.clone(),
                onselect: props.onselect.clone(),
                updatemodel: props.updatemodel.clone(),
            };
            let content = renderer(&validator_context);
            let header = html! {
                <div>
                    <div class={ KIND_CLASSES.join(" ") }>
                        { node.kind.clone() }
                    </div>
                    <div class="inline-block text-xs border border-black">
                        { ctx.props().hash.clone().unwrap_or("-".to_string()) }
                    </div>
                </div>
            };
            let footer = if selected {
                let entries: Vec<Entry> = kind
                    .map(|k| {
                        let KindValue::Struct { fields, .. } = k.value;
                        fields
                            .iter()
                            .map(|f| Entry {
                                label: f.name.to_string(),
                                description: "".to_string(),
                                action: Msg::AddField(path.to_vec(), f.name.to_string()),
                                valid_classes: FIELD_CLASSES
                                    .iter()
                                    .map(|v| v.to_string())
                                    .collect(),
                            })
                            .collect()
                    })
                    .unwrap_or_default();
                // Make it look like an actual field.
                html! {
                    <div class="pl-3">
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
                { header }
                { content }
                { footer }
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

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            NodeMsg::Click => {
                self.focus_input();
                true
            }
        }
    }

    fn changed(&mut self, ctx: &Context<Self>) -> bool {
        log::debug!("changed");
        true
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
