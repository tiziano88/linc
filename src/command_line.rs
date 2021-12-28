use crate::types::{get_value_from_input_event, Msg};
use web_sys::HtmlInputElement;
use yew::prelude::*;

pub struct CommandLine {
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
    pub oninput: Callback<String>,
    #[prop_or_default]
    pub onselect: Callback<Msg>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Entry {
    pub label: String,
    pub description: String,
    pub action: Msg,
}

#[derive(Debug)]
pub enum CommandLineMsg {
    Noop,
    Click,
    Key(KeyboardEvent),
    Input(String),
}

impl Component for CommandLine {
    type Message = CommandLineMsg;
    type Properties = CommandLineProperties;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            selected_command_index: 0,
            value: ctx.props().value.clone(),
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let props = ctx.props();
        let value = &self.value;
        let enabled = props.enabled;
        let selected_entry_suffix = props
            .entries
            .get(self.selected_command_index)
            .cloned()
            .map(|v| v.label.clone())
            .unwrap_or_default()
            .strip_prefix(value)
            .map(|v| v.to_string())
            .unwrap_or_default();
        let selected = true;
        let entries: Vec<_> = if selected {
            props
                .entries
                .iter()
                .enumerate()
                .filter_map(|(i, v)| {
                    let value_string = v.label.clone();
                    if value_string.starts_with(value) {
                        let value_suffix = value_string.strip_prefix(value).unwrap_or_default();

                        // let node = v.to_node();
                        let action = v.action.clone();
                        let onselect = props.onselect.clone();
                        let onclick = ctx.link().callback(move |_e: MouseEvent| {
                            onselect.emit(action.clone());
                            CommandLineMsg::Noop
                        });
                        let mut classes_item = vec!["block", "border"];
                        if i == self.selected_command_index {
                            classes_item.push("selected");
                        }
                        // Avoid re-selecting the node, we want to move to next.
                        Some(html! {
                            <span
                              class={ classes_item.join(" ") }
                              onmousedown={ onclick }
                            >
                              <span class="font-bold">{ value.clone() }</span>
                              { value_suffix }
                            </span>
                        })
                    } else {
                        None
                    }
                })
                .collect()
        } else {
            vec![]
        };
        let classes_dropdown = vec!["absolute", "z-10", "bg-white"];
        // let id = view::command_input_id(&path);
        let style = if value.len() > 0 {
            format!("width: {}ch;", value.len())
        } else {
            "width: 0.1ch;".to_string()
        };
        // XXX: Chrome inspector CSS color editor.
        let placeholder = "";
        let placeholder = if placeholder.is_empty() {
            None
        } else {
            Some(html! {
                <div class="placeholder">{ placeholder }</div>
            })
        };
        let suffix = if selected {
            selected_entry_suffix
        } else {
            "".to_string()
        };
        let mut class = vec!["inline-block", "w-full"];
        // let errors = vec![];
        // if !errors.is_empty() {
        //     class.push("error");
        // }

        // let editing = if model.mode == crate::types::Mode::Edit {
        let editing = if enabled && selected {
            html! {
                <>
                    <span class="completion">{ suffix }</span>
                    <div class={ classes_dropdown.join(" ") }>
                        { for entries }
                    </div>
                </>
            }
        } else {
            html! {}
        };

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
                { for placeholder }
                <span>
                    <input
                      ref={ ctx.props().input_node_ref.clone() }
                    //   id={ id }
                      class={ class }
                      type="text"
                      oninput={ oninput }
                      onkeydown={ onkeydown }
                    //   onfocus={ onfocus }
                      value={ value.to_string() }
                      style={ style }
                    //   disabled={ model.mode != crate::types::Mode::Edit }
                      autocomplete="off"
                    />
                    { editing }
                </span>
            </span>
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            CommandLineMsg::Noop => false,
            CommandLineMsg::Click => {
                let input_node = ctx.props().input_node_ref.clone();
                input_node
                    .cast::<HtmlInputElement>()
                    .unwrap()
                    .focus()
                    .unwrap();
                false
            }
            CommandLineMsg::Input(v) => {
                self.value = v;
                true
            }
            CommandLineMsg::Key(e) => {
                let props = ctx.props();
                let entries = props.entries.clone();
                let selected_command_index = self.selected_command_index;
                match e.key().as_ref() {
                    "ArrowUp" => {
                        self.selected_command_index = if selected_command_index > 0 {
                            selected_command_index - 1
                        } else {
                            entries.len() - 1
                        }
                    }
                    "ArrowDown" => {
                        self.selected_command_index = if selected_command_index < entries.len() - 1
                        {
                            selected_command_index + 1
                        } else {
                            0
                        }
                    }
                    "Enter" => {
                        let selected_entry = entries.get(selected_command_index).cloned();
                        if let Some(selected_entry) = selected_entry {
                            let action = selected_entry.action.clone();
                            props.onselect.emit(action);
                        }
                    }
                    _ => {}
                };
                true
            }
        }
    }
}
