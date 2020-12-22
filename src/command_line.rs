use yew::prelude::*;

pub struct CommandLine {
    props: Props,
    link: ComponentLink<Self>,
    selected: Option<usize>,
    filtered_values: Vec<String>,
    original_value: String,
    displayed_value: String,
}

#[derive(PartialEq, Clone, Debug)]
pub enum State {
    Empty,
    Invalid,
    Valid,
}

#[derive(PartialEq, Clone, Properties, Debug)]
pub struct Props {
    pub values: Vec<String>,
    pub on_change: Callback<String>,
    pub base_value: String,
    pub state: State,
}

#[derive(Debug)]
pub enum Msg {
    /// Sent when the user selects a new option.
    Selected(String),
    /// When typing.
    Input(String),
    CommandKey(KeyboardEvent),
}

impl yew::Component for CommandLine {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: yew::ComponentLink<Self>) -> Self {
        let original_value = props.base_value.clone();
        let displayed_value = original_value.clone();
        Self {
            props,
            link,
            selected: None,
            original_value,
            displayed_value,
            filtered_values: Vec::new(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> yew::ShouldRender {
        match msg {
            Msg::Input(s) => {
                self.selected = None;
                self.original_value = s;
                self.update_filtered_values();
                self.update_displayed_value();
                self.emit_selected();
            }
            Msg::Selected(x) => {}
            Msg::CommandKey(v) => match v.code().as_ref() {
                "Escape" => {
                    self.selected = None;
                    self.update_displayed_value();
                }
                "Enter" => {
                    self.emit_selected();
                    self.original_value = "".to_string();
                    self.selected = None;
                    self.update_displayed_value();
                }
                "ArrowUp" => {
                    self.selected = if let Some(v) = self.selected {
                        if v == 0 {
                            None
                        } else {
                            Some(v - 1)
                        }
                    } else {
                        None
                    };
                    self.update_displayed_value();
                }
                "ArrowDown" => {
                    self.selected = if let Some(v) = self.selected {
                        if self.filtered_values.len() > 0 && v == self.filtered_values.len() - 1 {
                            Some(v)
                        } else {
                            Some(v + 1)
                        }
                    } else {
                        Some(0)
                    };
                    self.update_displayed_value();
                }
                _ => {
                    log::info!("k: {:?}", v);
                }
            },
        };
        true
    }

    fn change(&mut self, props: Self::Properties) -> yew::ShouldRender {
        self.props = props;
        self.update_filtered_values();
        self.update_displayed_value();
        true
    }
    fn view(&self) -> yew::Html {
        let mut command_class = vec![
            "focus:border-blue-500",
            "focus:ring-1",
            "focus:ring-blue-500",
            "focus:outline-none",
            "text-sm",
            "text-black",
            "placeholder-gray-500",
            "border",
            "border-gray-200",
            "rounded-md",
            "py-2",
            "pl-10",
        ];
        match self.props.state {
            State::Empty => {}
            State::Invalid => command_class.push("bg-red-500"),
            State::Valid => command_class.push("bg-green-500"),
        }
        let onkeypress = self
            .link
            .callback(move |e: KeyboardEvent| Msg::CommandKey(e));
        let options = self.filtered_values.iter().enumerate().map(|(i, v)| {
            let s = v.clone();
            let callback = self.link.callback(move |_| Msg::Input(s.clone()));
            let mut classes = vec!["border", "border-solid", "border-blue-500"];
            if let Some(s) = self.selected {
                if s == i {
                    classes.push("bg-yellow-500");
                }
            }
            html! {
                <div
                  onclick=callback
                  class=classes.join(" ")>{ v }
                </div>
            }
        });
        let oninput = self.link.callback(move |e: InputData| Msg::Input(e.value));
        html! {
            <div class="h-40" onkeydown=onkeypress>
              <input class=command_class oninput=oninput value=self.displayed_value />
              { for options }
            </div>
        }
    }
}

impl CommandLine {
    fn update_filtered_values(&mut self) {
        self.filtered_values = self
            .props
            .values
            .iter()
            .filter(|v| v.starts_with(&self.props.base_value))
            .cloned()
            .collect()
    }
    fn update_displayed_value(&mut self) {
        if let Some(i) = self.selected {
            if let Some(v) = self.filtered_values.get(i) {
                self.displayed_value = v.clone();
            } else {
                self.displayed_value = "".to_string();
            }
        } else {
            self.displayed_value = self.original_value.clone();
        }
    }
    fn emit_selected(&self) {
        self.props.on_change.emit(self.displayed_value.clone());
    }
}
