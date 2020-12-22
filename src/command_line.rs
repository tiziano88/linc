use yew::prelude::*;

pub struct CommandLine {
    props: Props,
    link: ComponentLink<Self>,
    selected: Option<usize>,
    original_value: String,
    viewed_value: String,
}

#[derive(PartialEq, Clone, Debug)]
pub enum State {
    Empty,
    Invalid,
    Valid,
}

#[derive(PartialEq, Clone, Properties, Debug)]
pub struct Props {
    pub options: Vec<String>,
    pub on_change: Callback<String>,
    pub value: String,
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
        let original_value = props.value.clone();
        let viewed_value = original_value.clone();
        Self {
            props,
            link,
            selected: None,
            original_value,
            viewed_value,
        }
    }

    fn update(&mut self, msg: Self::Message) -> yew::ShouldRender {
        match msg {
            Msg::Input(s) => {
                self.props.on_change.emit(s.clone());
                self.selected = None;
                self.original_value = s;
                self.update_from_selected();
            }
            Msg::Selected(x) => {}
            Msg::CommandKey(v) => match v.code().as_ref() {
                "Escape" => {
                    self.selected = None;
                    self.update_from_selected();
                }
                "Enter" => {
                    self.props.on_change.emit(self.viewed_value.clone());
                    self.original_value = "".to_string();
                    self.selected = None;
                    self.update_from_selected();
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
                    self.update_from_selected();
                }
                "ArrowDown" => {
                    self.selected = if let Some(v) = self.selected {
                        if v == self.props.options.len() - 1 {
                            Some(v)
                        } else {
                            Some(v + 1)
                        }
                    } else {
                        Some(0)
                    };
                    self.update_from_selected();
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
        let options = self
            .props
            .options
            .iter()
            .filter(|v| v.starts_with(&self.props.value))
            .enumerate()
            .map(|(i, v)| {
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
              <input class=command_class oninput=oninput value=self.viewed_value />
              { for options }
            </div>
        }
    }
}

impl CommandLine {
    fn update_from_selected(&mut self) {
        if let Some(i) = self.selected {
            self.viewed_value = self.props.options[i].clone();
        } else {
            self.viewed_value = self.original_value.clone();
        }
    }
}
