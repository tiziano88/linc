use crate::{
    node::NodeComponent,
    schema::{Field, SCHEMA},
    types::*,
};
use gloo_storage::{LocalStorage, Storage};
use std::{
    collections::{BTreeMap, HashMap},
    rc::Rc,
};
use web_sys::{window, InputEvent, MouseEvent};
use yew::{html, prelude::*, Html, KeyboardEvent};

#[derive(PartialEq, Clone)]
pub struct Model {
    pub file: File,

    pub cursor: Path,
    pub hover: Path,
    pub mode: Mode,

    pub node_state: HashMap<Path, NodeState>,

    pub show_serialized: bool,
    pub rich_render: bool,
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

        let serialized = if self.show_serialized {
            html! {
                <div class="column">{ self.view_file_json(&self.file) }</div>
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
              onkeydown={ onkeypress }
              onmouseover={ onmouseover }
              >
                <div class="sticky top-0 bg-white">
                    <div>{ "LINC" }</div>
                    <div>{ "click on an empty node to see list of possible completions" }</div>
                    <div class="column">
                        <div>{ "Mode: " }{ format!("{:?}", self.mode) }</div>
                        <div>{ display_cursor(&self.cursor) }</div>
                    </div>

                    <div>{ self.view_actions(ctx) }</div>
                </div>
                <div class="flex">
                    <NodeComponent
                      model={ Rc::from(self.clone()) }
                      hash={ self.file.root.clone() }
                      onselect={ ctx.link().callback(Msg::Select) }
                      updatemodel={ ctx.link().callback(|m| m) }
                      path={ vec![] }
                    />
                </div>
                <div class="h-40">
                    <div>{ format!("Ref: {:?}", self.file.root().traverse(&self.file, &self.cursor).unwrap().hash) }</div>
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
        Model {
            file: super::initial::initial(),
            mode: Mode::Normal,
            cursor: vec![],
            hover: vec![],
            node_state: HashMap::new(),
            show_serialized: false,
            rich_render: true,
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
        const KEY: &str = "linc_file";
        match msg {
            Msg::ToggleSerialized => {
                self.show_serialized = !self.show_serialized;
            }
            Msg::ToggleRenderer => {
                self.rich_render = !self.rich_render;
            }
            Msg::Select(path) => {
                self.cursor = path;
            }
            Msg::Hover(path) => {
                self.hover = path;
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
                self.cursor = self.cursor[..self.cursor.len() - 1].to_vec();
            }
            Msg::Cut => {
                if let Some(cursor) = self.file.root().traverse(&self.file, &self.cursor) {
                    self.stack.push(cursor.hash);
                }
            }
            Msg::Copy => {
                if let Some(cursor) = self.file.root().traverse(&self.file, &self.cursor) {
                    self.stack.push(cursor.hash);
                }
            }
            Msg::Paste => if let Some(node_ref) = self.stack.last() {},
            Msg::Store => {
                LocalStorage::set(KEY, &self.file).unwrap();
            }
            Msg::Load => {
                let res: gloo_storage::Result<File> = LocalStorage::get(KEY);
                if let Ok(file) = res {
                    self.file = file;
                }
            }
            Msg::Parse(v) => {
                /*
                log::debug!("parse {:?}", v);
                let html = html_parser::Dom::parse(&v).unwrap();
                log::debug!("parsed {:?}", html);
                fn add_string(model: &mut Model, value: &str) -> Hash {
                    model.file.add_node(&Node {
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
                            model.file.add_node(&Node {
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
                    model.file.add_node(&Node {
                        kind: "dom".into(),
                        value: "".to_string(),
                        links: children,
                    })
                }
                let h = add_dom(self, &html);
                self.file.root = h;
                    */
            }
            Msg::SetMode(mode) => {
                self.mode = mode;
            }
            Msg::AddField(path, field_id) => {
                let mut node = self
                    .file
                    .root()
                    .traverse(&self.file, &path)
                    .unwrap()
                    .node(&self.file)
                    .unwrap()
                    .clone();
                node.links
                    .entry(field_id)
                    .or_insert_with(Vec::new)
                    .push("".into());
                let n = node.links[&field_id].len();
                let new_root = self.file.replace_node(&path, &node);
                if let Some(new_root) = new_root {
                    self.file.root = new_root;
                }
                self.cursor = append(
                    &path,
                    Selector {
                        field_id,
                        index: n - 1,
                    },
                );
            }
            Msg::ReplaceNode(path, node, mv) => {
                log::info!("replace node {:?} {:?}", path, node);
                let new_root = self.file.replace_node(&path, &node);
                log::info!("new root: {:?}", new_root);
                if let Some(new_root) = new_root {
                    self.file.root = new_root;
                }
                if mv {
                    ctx.link().send_message(Msg::Next);
                } else {
                    ctx.link().send_message(Msg::Select(path));
                }
            }
            Msg::SetNodeValue(path, value) => {
                self.cursor = path.clone();
                let mut node = self
                    .file
                    .root()
                    .traverse(&self.file, &path)
                    .unwrap()
                    .node(&self.file)
                    .cloned()
                    .unwrap_or_default();
                node.value = value;
                let new_root = self.file.replace_node(&path, &node);
                if let Some(new_root) = new_root {
                    self.file.root = new_root;
                }
            }
            Msg::AddItem => {
                let (selector, parent_path) = self.cursor.split_last().unwrap();
                let new_ref = self.file.add_node(&Node {
                    kind: "invalid".to_string(),
                    value: "invalid".to_string(),
                    links: BTreeMap::new(),
                });
                let mut parent = self
                    .file
                    .root()
                    .traverse(&self.file, parent_path)
                    .unwrap()
                    .node(&self.file)
                    .unwrap()
                    .clone();
                // If the field does not exist, create a default one.
                let children = parent.links.entry(selector.field_id).or_default();
                let new_index = selector.index + 1;
                children.insert(new_index, new_ref);
                self.file.replace_node(parent_path, &parent);
                // Select newly created element.
                self.cursor.last_mut().unwrap().index = new_index;
                // self.next();
            }
            Msg::DeleteItem => {
                let (selector, parent_path) = self.cursor.split_last().unwrap();
                let mut parent = self
                    .file
                    .root()
                    .traverse(&self.file, parent_path)
                    .unwrap()
                    .node(&self.file)
                    .unwrap()
                    .clone();
                // If the field does not exist, create a default one.
                let children = parent.links.entry(selector.field_id).or_default();
                children.remove(selector.index);
                if let Some(new_root) = self.file.replace_node(parent_path, &parent) {
                    self.file.root = new_root;
                }
                // Select parent.
                self.cursor = self.cursor[..self.cursor.len() - 1].to_vec();
            }
            Msg::CommandKey(path, e) => {
                log::info!("key: {}", e.key());
                self.cursor = path.clone();
                let node = self
                    .file
                    .root()
                    .traverse(&self.file, &path)
                    .unwrap()
                    .node(&self.file)
                    .cloned()
                    .unwrap_or_default();

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

                // See https://developer.mozilla.org/en-US/docs/Web/API/KeyboardEvent/code
                match e.key().as_ref() {
                    "Enter" => {}
                    "Escape" => {
                        self.mode = Mode::Normal;
                        // If it is a pure value, select the parent again so another field may be
                        // added.
                        if node.kind.is_empty() {
                            self.cursor = self.cursor[..self.cursor.len() - 1].to_vec();
                        }
                    }
                    // "Enter" if self.mode == Mode::Edit =>
                    // self.link.send_message(Msg::EnterCommand), "Escape" =>
                    // self.link.send_message(Msg::EscapeCommand),
                    "ArrowUp" => {}
                    "ArrowDown" => {}
                    "ArrowLeft" if self.mode == Mode::Normal => ctx.link().send_message(Msg::Prev),
                    "ArrowRight" if self.mode == Mode::Normal => ctx.link().send_message(Msg::Next),
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
    fn prev(&mut self) {
        if let Some(cursor) = self.file.root().traverse(&self.file, &self.cursor) {
            if let Some(prev) = cursor.prev(&self.file) {
                self.cursor = prev.path.clone();
            }
        }
        /*
        let flattened_paths = self.flatten_paths(&[]);
        log::info!("paths: {:?}", flattened_paths);
        let current_path_index = flattened_paths.iter().position(|x| *x == self.cursor);
        log::info!("current: {:?}", current_path_index);
        if let Some(current_path_index) = current_path_index {
            if current_path_index > 0 {
                if let Some(path) = flattened_paths.get(current_path_index - 1) {
                    self.cursor = path.clone();
                }
            }
        }
        */
    }

    fn next(&mut self) {
        if let Some(cursor) = self.file.root().traverse(&self.file, &self.cursor) {
            if let Some(next) = cursor.next(&self.file) {
                self.cursor = next.path.clone();
            }
            /*
            if let Some((field_id, children)) = &node.links.iter().next() {
                if !children.is_empty() {
                    self.cursor.push(Selector {
                        field_id: **field_id,
                        index: 0,
                    });
                }
            } else {
                if let Some((last, parent_path)) = self.cursor.split_last() {
                    if let Some(parent) = self.file.lookup(parent_path) {
                        let children_len =
                            parent.links.get(&last.field_id).map(Vec::len).unwrap_or(0);
                        if last.index < children_len - 1 {
                            let mut new_cursor = parent_path.to_vec();
                            new_cursor.push(Selector {
                                field_id: last.field_id,
                                index: last.index + 1,
                            });
                            self.cursor = new_cursor;
                        } else {
                            if let Some((next_field_id, next_children)) = parent
                                .links
                                .range((
                                    std::ops::Bound::Excluded(last.field_id),
                                    std::ops::Bound::Unbounded,
                                ))
                                .next()
                            {}
                        }
                    }
                }
            }
            */
        }
    }

    pub fn update_errors(&mut self, ctx: &Context<Self>) {
        self.update_errors_node(ctx, &self.cursor.clone());
    }

    pub fn update_errors_node(&mut self, _ctx: &Context<Self>, path: &[Selector]) {
        /*
        let node = match self.file.lookup(path) {
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

    pub fn view_file_json(&self, file: &File) -> Html {
        // let serialized = serde_json::to_string_pretty(file).expect("could not serialize to
        // JSON");
        let serialized = format!("root: {:?}\nfile: {:#?}", file.root, file);
        html! {
            <pre>{ serialized }</pre>
        }
    }
}
