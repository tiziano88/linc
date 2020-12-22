use yew::prelude::*;

pub struct CommandLine {
    props: Props,
    link: ComponentLink<Self>,
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
}

impl yew::Component for CommandLine {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: yew::ComponentLink<Self>) -> Self {
        Self { props, link }
    }

    fn update(&mut self, msg: Self::Message) -> yew::ShouldRender {
        match msg {
            Msg::Input(s) => {
                self.props.value = s;
            }
            Msg::Selected(x) => {}
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
        let options = self
            .props
            .options
            .iter()
            .filter(|v| v.starts_with(&self.props.value))
            .map(|v| {
                let s = v.clone();
                let callback = self.link.callback(move |_| Msg::Input(s.clone()));
                html! {<div onclick=callback class="border-solid border border-blue-500">{ v }</div>}
            });
        let oninput = self.link.callback(move |e: InputData| Msg::Input(e.value));
        html! {
            <div class="h-40">
              <div>{ "COMMAND" }</div>
              <input class=command_class oninput=oninput value=self.props.value />
              { for options }
            </div>
        }
    }
}
