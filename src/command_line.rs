use yew::prelude::*;

pub struct CommandLine<T: Clone + 'static> {
    props: Props<T>,
    link: ComponentLink<Self>,
    selected_index: usize,
}

#[derive(PartialEq, Clone, Properties, Debug)]
pub struct Props<T: Clone> {
    pub values: Vec<String>,
    pub on_select: Callback<T>,
    pub on_input: Callback<String>,
}

#[derive(Debug)]
pub enum Msg {
    /// Sent when the user selects a new option.
    Selected(String),
    /// When typing.
    Input(String),
    CommandKey(KeyboardEvent),
}

impl<T: Clone + 'static> yew::Component for CommandLine<T> {
    type Message = Msg;
    type Properties = Props<T>;

    fn create(props: Self::Properties, link: yew::ComponentLink<Self>) -> Self {
        let original_value = props.base_value.clone();
        let displayed_value = original_value.clone();
        Self {
            props,
            link,
            selected_index: 0,
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
        self.selected_index = 0;
        true
    }
    fn view(&self) -> yew::Html {
        let command_class = vec![
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
        let onkeypress = self
            .link
            .callback(move |e: KeyboardEvent| Msg::CommandKey(e));
        let values = self.props.values.iter().enumerate().map(|(i, v)| {
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
              { for values }
            </div>
        }
    }
}

impl<T: Clone> CommandLine<T> {
    fn emit_selected(&self) {
        self.props.on_change.emit(self.displayed_value.clone());
    }
}
