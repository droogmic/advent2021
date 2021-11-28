use yew::prelude::*;
use yew::Properties;

use crate::days;

#[derive(Properties, Clone, PartialEq)]
pub struct DayProps {
    pub index: usize,
    pub title: String,
}

pub enum Msg {
    Run,
}

pub struct Day {
    // `ComponentLink` is like a reference to a component.
    // It can be used to send messages to the component
    link: ComponentLink<Self>,
    props: DayProps,
}

impl Component for Day {
    type Message = Msg;
    type Properties = DayProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link, props }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Run => {
                log::info!("Running Something",);
                log::info!("{}", days::day0::part1("1 + 2".into()));
                true
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        // Should only return "true" if new properties are different to
        // previously received properties.
        // This component has no properties so we will always return "false".
        false
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <h2>{"Day "}{self.props.index}{": "}<em>{&self.props.title}</em></h2>
                {
                    for Self::get_examples().iter().enumerate().map(|(idx, example)| html!{
                        <h3>
                            {"Example "}{idx}{": "}
                            <pre>{example}</pre>
                            <button onclick=self.link.callback(|_| Msg::Run)>{ "run" }</button>
                        </h3>
                    })
                }
            </div>
        }
    }
}

impl Day {
    fn get_examples() -> Vec<String> {
        return vec!["1 + 2".into()];
    }
}
