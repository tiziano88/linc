use crate::{
    node::NodeComponent,
    schema::{Field, SCHEMA},
    types::*,
};
use gloo_storage::{LocalStorage, Storage};
use serde::{Deserialize, Serialize};
use std::{
    collections::{BTreeMap, HashMap},
    rc::Rc,
};
use web_sys::{window, InputEvent, MouseEvent};
use yew::{html, prelude::*, Html, KeyboardEvent};

#[derive(PartialEq, Clone, Serialize, Deserialize)]
pub struct GlobalState {
    pub node_store: Rc<NodeStore>,
    pub mode: Mode,
    pub show_serialized: bool,
    pub rich_render: bool,
}

impl GlobalState {
    fn node_store_mut(&mut self) -> &mut NodeStore {
        Rc::make_mut(&mut self.node_store)
    }
}

#[derive(PartialEq, Clone)]
pub struct Model {
    pub global_state: Rc<GlobalState>,

    pub root: Hash,

    pub selected_path: Path,
    pub hover_path: Path,

    pub node_state: HashMap<Path, NodeState>,

    pub stack: Vec<String>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Msg {
    Select(Path),
    Hover(Path),

    Store,
    Load,
    Parse(String),

    Prev,
    Next,
    Parent,

    AddItem,
    DeleteItem,

    SetMode(Mode),

    ReplaceNode(Path, Node, bool),
    AddField(Path, usize),

    SetNodeValue(Path, String),

    CommandKey(Path, KeyboardEvent),

    ToggleSerialized,
    ToggleRenderer,

    Copy,
    Cut,
    Paste,
    /* EnterCommand,
     * EscapeCommand,
     */
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn view(&self, ctx: &Context<Self>) -> Html {
        let onmouseover = ctx.link().callback(move |e: MouseEvent| {
            e.stop_propagation();
            Msg::Hover(vec![])
        });
        let parse = ctx
            .link()
            .callback(move |e: InputEvent| Msg::Parse(get_value_from_input_event(e)));

        let serialized = if self.global_state.show_serialized {
            html! {
                <div class="column">{ self.view_node_store(&self.global_state.node_store) }</div>
            }
        } else {
            html! {}
        };

        let onkeypress = ctx.link().callback(move |e: KeyboardEvent| {
            e.stop_propagation();
            Msg::CommandKey(vec![], e)
        });
        html! {
            <div
              tabindex="0"
              onkeydown={ onkeypress }
              onmouseover={ onmouseover }
              >
                <div class="sticky top-0 bg-white">
                    <div>{ "LINC" }</div>
                    <div>{ "click on an empty node to see list of possible completions" }</div>
                    <div class="column">
                        <div>{ "Mode: " }{ format!("{:?}", self.global_state.mode) }</div>
                        <div class="h-8">{ display_cursor(&self.selected_path) }</div>
                    </div>

                    <div>{ self.view_actions(ctx) }</div>
                </div>
                <div class="flex">
                    <NodeComponent
                      global_state={ self.global_state.clone() }
                      cursor={ self.root().clone() }
                      selected_path={ self.selected_path.clone() }
                      onselect={ ctx.link().callback(Msg::Select) }
                      updatemodel={ ctx.link().callback(|m| m) }
                    />
                </div>
                <div class="h-40">
                    <div>{ format!("Ref: {:?}", self.path(&self.selected_path).map(|c| c.hash)) }</div>
                    <div>{ format!("Node: {:?}", self.path(&self.selected_path).and_then(|c| c.node(&self.global_state.node_store))) }</div>
                    <textarea type="text" class="border-solid border-black border" oninput={ parse } />
                    { serialized }
                </div>
            </div>
        }
    }

    fn create(_ctx: &Context<Self>) -> Self {
        // let key_listener = KeyboardService::register_key_down(
        //     &window().unwrap(),
        //     ctx.link().callback(move |e: KeyboardEvent| {
        //         // e.stop_propagation();
        //         // e.stop_immediate_propagation();
        //         // e.prevent_default();
        //         Msg::CommandKey(e)
        //     }),
        // );
        let (node_store, root) = super::initial::initial();
        Model {
            global_state: Rc::new(GlobalState {
                node_store: Rc::new(node_store),
                mode: Mode::Normal,
                show_serialized: false,
                rich_render: true,
            }),

            root,

            selected_path: vec![],
            hover_path: vec![],

            node_state: HashMap::new(),

            stack: vec![],
        }
    }

    fn changed(&mut self, _ctx: &Context<Self>) -> bool {
        false
    }

    fn rendered(&mut self, _ctx: &Context<Self>, _first_render: bool) {}

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        if let Msg::Hover(_) = msg {
            return false;
        }
        log::info!("update {:?}", msg);
        const GLOBAL_STATE_KEY: &str = "linc_global_state";
        const ROOT_NODE_KEY: &str = "linc_root_node";
        match msg {
            Msg::ToggleSerialized => {
                self.global_state_mut().show_serialized = !self.global_state.show_serialized;
            }
            Msg::ToggleRenderer => {
                self.global_state_mut().rich_render = !self.global_state.rich_render;
            }
            Msg::Select(path) => {
                self.selected_path = path;
            }
            Msg::Hover(path) => {
                self.hover_path = path;
            }
            // TODO: sibling vs inner
            Msg::Prev => {
                self.prev();
            }
            // Preorder tree traversal.
            Msg::Next => {
                self.next();
            }
            Msg::Parent => {
                self.parent();
            }
            Msg::Cut => {
                if let Some(cursor) = self.path(&self.selected_path) {
                    self.stack.push(cursor.hash);
                }
            }
            Msg::Copy => {
                if let Some(cursor) = self.path(&self.selected_path) {
                    self.stack.push(cursor.hash);
                }
            }
            Msg::Paste => if let Some(node_ref) = self.stack.last() {},
            Msg::Store => {
                LocalStorage::set(GLOBAL_STATE_KEY, &*self.global_state).unwrap();
                LocalStorage::set(ROOT_NODE_KEY, self.root.clone()).unwrap();
            }
            Msg::Load => {
                let res: gloo_storage::Result<GlobalState> = LocalStorage::get(GLOBAL_STATE_KEY);
                if let Ok(global_state) = res {
                    self.global_state = Rc::new(global_state);
                }
                self.root = LocalStorage::get(ROOT_NODE_KEY).unwrap();
            }
            Msg::Parse(v) => {
                /*
                log::debug!("parse {:?}", v);
                let html = html_parser::Dom::parse(&v).unwrap();
                log::debug!("parsed {:?}", html);
                fn add_string(model: &mut Model, value: &str) -> Hash {
                    model.node_store.add_node(&Node {
                        kind: "string".into(),
                        value: value.into(),
                        links: BTreeMap::new(),
                    })
                }
                fn add_node(model: &mut Model, node: &html_parser::Node) -> Hash {
                    match node {
                        html_parser::Node::Element(e) => {
                            let mut children: BTreeMap<usize, Vec<String>> = BTreeMap::new();
                            e.attributes.iter().for_each(|(k, v)| {
                                children.entry(k.clone()).or_insert_with(Vec::new).push(
                                    add_string(model, &v.as_ref().cloned().unwrap_or_default()),
                                );
                            });
                            e.children.iter().for_each(|v| {
                                children
                                    .entry("children".to_string())
                                    .or_insert_with(Vec::new)
                                    .push(add_node(model, v));
                            });
                            model.node_store.add_node(&Node {
                                kind: e.name.clone(),
                                value: "".into(),
                                links: children,
                            })
                        }
                        html_parser::Node::Text(t) => add_string(model, t),
                        html_parser::Node::Comment(c) => add_string(model, c),
                    }
                }
                fn add_dom(model: &mut Model, e: &html_parser::Dom) -> Hash {
                    let mut children: BTreeMap<String, Vec<String>> = BTreeMap::new();
                    e.children.iter().for_each(|v| {
                        children
                            .entry("children".to_string())
                            .or_insert_with(Vec::new)
                            .push(add_node(model, v));
                    });
                    model.node_store.add_node(&Node {
                        kind: "dom".into(),
                        value: "".to_string(),
                        links: children,
                    })
                }
                let h = add_dom(self, &html);
                self.node_store.root = h;
                    */
            }
            Msg::SetMode(mode) => {
                Rc::make_mut(&mut self.global_state).mode = mode;
            }
            Msg::AddField(path, field_id) => {
                let mut node = self
                    .path(&path)
                    .unwrap()
                    .node(&self.global_state.node_store)
                    .unwrap()
                    .clone();
                node.links
                    .entry(field_id)
                    .or_insert_with(Vec::new)
                    .push("".into());
                let n = node.links[&field_id].len();
                self.replace_node(&path, &node);
                self.selected_path = append(
                    &path,
                    Selector {
                        field_id,
                        index: n - 1,
                    },
                );
            }
            Msg::ReplaceNode(path, node, mv) => {
                log::info!("replace node {:?} {:?}", path, node);
                self.replace_node(&path, &node);
                if mv {
                    ctx.link().send_message(Msg::Next);
                } else {
                    ctx.link().send_message(Msg::Select(path));
                }
            }
            Msg::SetNodeValue(path, value) => {
                self.selected_path = path.clone();
                let mut node = self
                    .path(&path)
                    .unwrap()
                    .node(&self.global_state.node_store)
                    .cloned()
                    .unwrap_or_default();
                node.value = value;
                self.replace_node(&path, &node);
            }
            Msg::AddItem => {
                let selected_path = self.selected_path.clone();
                let (selector, parent_path) = selected_path.split_last().unwrap();
                let new_ref = self.global_state_mut().node_store_mut().put(&Node {
                    kind: "invalid".to_string(),
                    value: "invalid".to_string(),
                    links: BTreeMap::new(),
                });
                let mut parent = self
                    .path(parent_path)
                    .unwrap()
                    .node(&self.global_state.node_store)
                    .unwrap()
                    .clone();
                // If the field does not exist, create a default one.
                let children = parent.links.entry(selector.field_id).or_default();
                let new_index = selector.index + 1;
                children.insert(new_index, new_ref);
                self.replace_node(parent_path, &parent);
                // Select newly created element.
                self.selected_path.last_mut().unwrap().index = new_index;
                // self.next();
            }
            Msg::DeleteItem => {
                let selected_path = self.selected_path.clone();
                let (selector, parent_path) = selected_path.split_last().unwrap();
                let mut parent = self
                    .path(parent_path)
                    .unwrap()
                    .node(&self.global_state.node_store)
                    .unwrap()
                    .clone();
                // If the field does not exist, create a default one.
                let children = parent.links.entry(selector.field_id).or_default();
                children.remove(selector.index);
                self.replace_node(parent_path, &parent);
                // Select parent.
                self.selected_path = self.selected_path[..self.selected_path.len() - 1].to_vec();
            }
            Msg::CommandKey(_path, e) => {
                log::info!("key: {}", e.key());
                // self.selected_path = self.selected_path
                let node = self
                    .path(&self.selected_path)
                    .unwrap()
                    .node(&self.global_state.node_store)
                    .cloned()
                    .unwrap_or_default();

                /*
                let selection = window().unwrap().get_selection().unwrap().unwrap();
                let anchor_node = selection.anchor_node().unwrap();
                let _anchor_offset = selection.anchor_offset();
                let anchor_node_value = anchor_node.node_value().unwrap_or_default();
                log::info!(
                    "selection: {:?} {} {}",
                    selection,
                    selection.anchor_offset(),
                    anchor_node_value
                );
                */

                // See https://developer.mozilla.org/en-US/docs/Web/API/KeyboardEvent/code
                match e.key().as_ref() {
                    "Enter" => {
                        self.global_state_mut().mode = Mode::Edit;
                    }
                    "Escape" => {
                        self.global_state_mut().mode = Mode::Normal;
                        // If it is a pure value, select the parent again so another field may be
                        // added.
                        if node.kind.is_empty() {
                            self.selected_path =
                                self.selected_path[..self.selected_path.len() - 1].to_vec();
                        }
                    }
                    // "Enter" if self.mode == Mode::Edit =>
                    // self.link.send_message(Msg::EnterCommand), "Escape" =>
                    // self.link.send_message(Msg::EscapeCommand),
                    "ArrowUp" if self.global_state.mode == Mode::Normal => {
                        ctx.link().send_message(Msg::Parent)
                    }
                    "ArrowDown" => {}
                    "ArrowLeft" if self.global_state.mode == Mode::Normal => {
                        ctx.link().send_message(Msg::Prev)
                    }
                    "ArrowRight" if self.global_state.mode == Mode::Normal => {
                        ctx.link().send_message(Msg::Next)
                    }
                    /*
                    "i" if self.mode == Mode::Normal => {
                        e.prevent_default();
                        self.link.send_message(Msg::SetMode(Mode::Edit))
                    }
                    "c" if self.mode == Mode::Normal => {
                        e.prevent_default();
                        self.link.send_message(Msg::SetMode(Mode::Edit))
                    }
                    "e" if self.mode == Mode::Normal => {
                        e.prevent_default();
                        self.link.send_message(Msg::SetMode(Mode::Edit))
                    }
                    */
                    _ => {}
                }
            }
        };
        // self.focus_command_line();
        self.update_errors(ctx);
        true
    }
}

impl Model {
    fn root(&self) -> Cursor {
        Cursor {
            parent: None,
            hash: self.root.clone(),
        }
    }

    pub fn path(&self, path: &[Selector]) -> Option<Cursor> {
        self.root().traverse(&self.global_state.node_store, path)
    }

    pub fn replace_node(&mut self, path: &[Selector], node: &Node) {
        if let Some(root) = self.replace_node_from(&self.root.clone(), path, node) {
            self.root = root;
        }
    }

    #[must_use]
    fn replace_node_from(&mut self, base: &Hash, path: &[Selector], node: &Node) -> Option<Hash> {
        if path.is_empty() {
            Some(self.global_state_mut().node_store_mut().put(node))
        } else {
            let mut base = self.global_state.node_store.get(base)?.clone();
            let selector = path[0].clone();
            match base
                .links
                .get(&selector.field_id)
                .and_then(|v| v.get(selector.index))
            {
                Some(old_child_hash) => {
                    let new_child_hash =
                        self.replace_node_from(old_child_hash, &path[1..], node)?;
                    base.links.get_mut(&selector.field_id)?[selector.index] = new_child_hash;
                }
                None => {
                    // WARN: Only works for one level of children.
                    let new_child_hash = self.global_state_mut().node_store_mut().put(node);
                    base.links
                        .entry(selector.field_id)
                        .or_default()
                        .push(new_child_hash);
                }
            };
            Some(self.global_state_mut().node_store_mut().put(&base))
        }
    }

    fn parent(&mut self) {
        if let Some(current) = self.path(&self.selected_path) {
            log::debug!("current: {:?}", current);
            if let Some(parent) = current.parent(&self.global_state.node_store) {
                self.selected_path = parent.path();
            }
        }
    }

    fn prev(&mut self) {
        if let Some(cursor) = self.path(&self.selected_path) {
            if let Some(prev) = cursor.prev(&self.global_state.node_store) {
                self.selected_path = prev.path();
            }
        }
    }

    fn next(&mut self) {
        log::warn!("old selected_path: {:?}", self.selected_path);
        if let Some(cursor) = self.path(&self.selected_path) {
            if let Some(next) = cursor.next(&self.global_state.node_store) {
                log::warn!("new selected_path: {:?}", next.path());
                self.selected_path = next.path();
            }
        }
    }

    fn global_state_mut(&mut self) -> &mut GlobalState {
        Rc::make_mut(&mut self.global_state)
    }

    pub fn update_errors(&mut self, ctx: &Context<Self>) {
        self.update_errors_node(ctx, &self.selected_path.clone());
    }

    pub fn update_errors_node(&mut self, _ctx: &Context<Self>, path: &[Selector]) {
        /*
        let node = match self.node_store.lookup(path) {
            Some(node) => node.clone(),
            None => return,
        };
        let kind = &node.kind;

        if let Some(kind) = SCHEMA.get_kind(kind) {
            let crate::schema::KindValue::Struct { validator: _, .. } = kind.value;
            // let errors = validator(&ValidatorContext {
            //     model: self,
            //     ctx,
            //     node: &node,
            //     path,
            //     placeholder: "",
            // });
            // log::info!("errors: {:?} {:?}", path, errors);
        }
        for (_, children) in node.children.iter() {
            for child in children {
                // TODO
                // self.update_errors_node(child);
            }
        }
        */
    }
}

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
            Action {
                image: None,
                text: "renderer".to_string(),
                msg: Msg::ToggleRenderer,
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
              class="action hover:bg-green-200 hover:text-green-800 group flex items-center bg-green-100 text-green-600 text-sm font-medium px-4 py-2"
              onclick={ callback }>
                { img }
                { &action.text }
            </button>
        }
    }

    pub fn view_node_store(&self, node_store: &NodeStore) -> Html {
        // let serialized = serde_json::to_string_pretty(node_store).expect("could not serialize to
        // JSON");
        let serialized = format!("root: {:?}\nnode_store: {:#?}", self.root, node_store);
        html! {
            <pre>{ serialized }</pre>
        }
    }
}
