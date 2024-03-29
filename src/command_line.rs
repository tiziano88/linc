use crate::{model::Msg, types::get_value_from_input_event};
use web_sys::HtmlInputElement;
use yew::prelude::*;

pub struct CommandLine {
    all_entries: Vec<Entry>,
    valid_entries: Vec<Entry>,
    // Among valid (filtered) entries.
    selected_command_index: usize,
    value: String,
}

#[derive(PartialEq, Clone, Properties, Debug)]
pub struct CommandLineProperties {
    pub value: String,
    pub entries: Vec<Entry>,
    pub enabled: bool,
    pub input_node_ref: NodeRef,
    #[prop_or_default]
    pub placeholder: String,
    #[prop_or_default]
    pub oninput: Callback<String>,
    #[prop_or_default]
    pub onselect: Callback<Msg>,
    #[prop_or_default]
    pub onenter: Callback<()>,
    #[prop_or_default]
    pub ondelete: Callback<()>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Entry {
    pub label: String,
    pub description: String,
    pub action: Msg,
    pub valid_classes: Vec<String>,
}

#[derive(Debug)]
pub enum CommandLineMsg {
    Click,
    Key(KeyboardEvent),
    Input(String),
}

impl Component for CommandLine {
    type Message = CommandLineMsg;
    type Properties = CommandLineProperties;

    fn create(ctx: &Context<Self>) -> Self {
        let mut c = Self {
            all_entries: ctx.props().entries.clone(),
            valid_entries: vec![],
            selected_command_index: 0,
            value: ctx.props().value.clone(),
        };
        c.update_valid_entries();
        c
    }

    fn changed(&mut self, ctx: &Context<Self>, _old_props: &CommandLineProperties) -> bool {
        self.all_entries = ctx.props().entries.clone();
        self.value = ctx.props().value.clone();
        self.update_valid_entries();
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let props = ctx.props();
        let value = &self.value;
        let enabled = props.enabled;
        let valid_entries = &self.valid_entries;
        let selected_entry = valid_entries.get(self.selected_command_index);
        let selected_entry_suffix = selected_entry
            .cloned()
            .map(|v| v.label)
            .unwrap_or_default()
            .strip_prefix(value)
            .map(|v| v.to_string())
            .unwrap_or_default();
        let selected = enabled;
        // TODO: Fuzzy search.
        let entries: Vec<_> = if selected {
            valid_entries
                .iter()
                .enumerate()
                .map(|(i, v)| {
                    let value_string = v.label.clone();
                    let value_suffix = value_string.strip_prefix(value).unwrap_or_default();

                    // let node = v.to_node();
                    let action = v.action.clone();
                    let onselect = props.onselect.clone();
                    let onclick = Callback::from(move |e: MouseEvent| {
                        // Avoid moving the focus away from the input.
                        e.prevent_default();
                        onselect.emit(action.clone());
                    });
                    let mut classes_item = vec!["block", "border"];
                    if i == self.selected_command_index {
                        classes_item.push("selected");
                    }
                    // Avoid re-selecting the node, we want to move to next.
                    html! {
                        <span
                          class={ classes_item.join(" ") }
                          onmousedown={ onclick }
                        >
                          <span class="font-bold">{ value.clone() }</span>
                          { value_suffix }
                        </span>
                    }
                })
                .collect()
        } else {
            vec![]
        };
        let classes_dropdown = vec!["absolute", "z-10", "bg-white", "w-64"];
        // let id = view::command_input_id(&path);
        let style = if !value.is_empty() {
            format!("width: {}ch;", value.len())
        } else {
            "width: 0.1ch;".to_string()
        };
        let rows = value.split('\n').count();
        // XXX: Chrome inspector CSS color editor.
        let placeholder = if value.is_empty() && !selected && !props.placeholder.is_empty() {
            Some(html! {
                // <div class="placeholder">{ props.placeholder.clone() }</div>
                <span>{ props.placeholder.clone() }</span>
            })
        } else {
            None
        };
        let suffix = if selected {
            selected_entry_suffix
        } else {
            "".to_string()
        };
        let class = vec![
            "inline-flex",
            "w-full",
            "bg-transparent",
            "resize-none",
            "overflow-hidden",
        ];
        // let errors = vec![];
        // if !errors.is_empty() {
        //     class.push("error");
        // }

        // let editing = if model.mode == crate::types::Mode::Edit {
        let dropdown = if enabled && selected {
            html! {
                <div class={ classes_dropdown.join(" ") }>
                    { for entries }
                </div>
            }
        } else {
            html! {}
        };
        let suffix = if enabled && selected {
            html! {
                <span class="completion">{ suffix }</span>
            }
        } else {
            html! {}
        };
        let mut classes = if enabled && selected {
            match selected_entry {
                Some(entry) => entry.valid_classes.clone().join(" "),
                None => "".to_string(),
            }
        } else {
            "".to_string()
        };
        classes.push_str(" flex");

        let oninput = props.oninput.clone();
        let oninput = ctx.link().callback(move |e: InputEvent| {
            let v = get_value_from_input_event(e);
            oninput.emit(v.clone());
            CommandLineMsg::Input(v)
        });

        let onclick = ctx.link().callback(|_| CommandLineMsg::Click);
        let onfocus = ctx.link().callback(|_| CommandLineMsg::Click);
        let onkeydown = ctx
            .link()
            .callback(move |e: KeyboardEvent| CommandLineMsg::Key(e));

        html! {
            <span
              onclick={ onclick }
              onfocus={ onfocus }
            >
                <span>
                    <span class={ classes } style="width: fit-content;">
                        <textarea
                            rows={ format!("{}", rows) }
                            ref={ ctx.props().input_node_ref.clone() }
                            //   id={ id }
                            class={ class }
                            type="text"
                            oninput={ oninput }
                            onkeydown={ onkeydown }
                            disabled={ !enabled }
                            //   onfocus={ onfocus }
                            value={ value.to_string() }
                            style={ style }
                            //   disabled={ model.mode != crate::types::Mode::Edit }
                            autocomplete="off"
                        >
                        </textarea>
                        { for placeholder }
                        { suffix }
                    </span>
                    { dropdown }
                </span>
            </span>
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            CommandLineMsg::Click => {
                let input_node = ctx.props().input_node_ref.clone();
                input_node
                    .cast::<HtmlInputElement>()
                    .unwrap()
                    .focus()
                    .unwrap();
                true
            }
            CommandLineMsg::Input(value) => {
                self.value = value;
                self.update_valid_entries();
                true
            }
            CommandLineMsg::Key(e) => {
                log::debug!("key: {:?}", e.key());
                let key = e.key();
                if key != "Escape" {
                    // Otherwise it will bubble up to the model root.
                    e.stop_propagation();
                }
                let props = ctx.props();
                let entries = &self.valid_entries;
                let selected_command_index = self.selected_command_index;
                if e.shift_key() {
                    return false;
                }
                match key.as_ref() {
                    "Backspace" => {
                        if self.value.is_empty() {
                            props.ondelete.emit(());
                        }
                    }
                    "ArrowUp" => {
                        if !entries.is_empty() {
                            self.selected_command_index = if selected_command_index > 0 {
                                selected_command_index - 1
                            } else {
                                entries.len() - 1
                            }
                        }
                    }
                    "ArrowDown" => {
                        if !entries.is_empty() {
                            self.selected_command_index =
                                if selected_command_index < entries.len() - 1 {
                                    selected_command_index + 1
                                } else {
                                    0
                                }
                        }
                    }
                    "Enter" => {
                        e.prevent_default();
                        let selected_entry = entries.get(selected_command_index).cloned();
                        if let Some(selected_entry) = selected_entry {
                            let action = selected_entry.action;
                            props.onselect.emit(action);
                        } else {
                            props.onenter.emit(());
                        }
                    }
                    _ => {}
                };
                true
            }
        }
    }
}

impl CommandLine {
    fn update_valid_entries(&mut self) {
        self.valid_entries = self
            .all_entries
            .clone()
            .into_iter()
            .filter(|v| v.label.starts_with(&self.value))
            .collect();
    }
}
