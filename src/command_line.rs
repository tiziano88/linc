use yew::prelude::*;

pub struct CommandLine {
    props: Props,
    link: ComponentLink<Self>,
    input: String,
}

#[derive(PartialEq, Clone, Properties, Debug)]
pub struct Props {
    pub options: Vec<String>,
    pub on_change: Callback<String>,
}

#[derive(Debug)]
pub enum Msg {
    /// Sent when the user selects a new option.
    Selected(String),
    /// When typing.
    Input(String),
}

impl yew::Component for CommandLine {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: yew::ComponentLink<Self>) -> Self {
        Self {
            props,
            link,
            input: "".to_string(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> yew::ShouldRender {
        match msg {
            Msg::Input(s) => {
                self.input = s;
            }
            Msg::Selected(x) => {}
        };
        true
    }

    fn change(&mut self, _props: Self::Properties) -> yew::ShouldRender {
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
        let options = self
            .props
            .options
            .iter()
            .filter(|v| v.starts_with(&self.input))
            .map(|v| {
                let s = v.clone();
                let callback = self.link.callback(move |_| Msg::Input(s.clone()));
                html! {<div onclick=callback>{v}</div>}
            });
        let oninput = self.link.callback(move |e: InputData| Msg::Input(e.value));
        html! {
            <div>
              { "COMMAND" }
              <input class=command_class oninput=oninput />
              { for options }
            </div>
        }
    }
}
