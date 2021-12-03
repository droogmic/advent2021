use std::rc::Rc;

use yew::prelude::*;
use yew::Properties;

#[derive(Properties, Clone)]
pub struct DayProps {
    pub day_num: usize,
    pub title: &'static str,
    pub example: &'static str,
    pub both_func: Rc<dyn Fn(&str) -> (String, String)>,
    pub text_format: (&'static str, &'static str),
    #[prop_or_default]
    pub messages: Vec<String>,
}

pub enum Msg {
    RunExample,
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
            Msg::RunExample => {
                log::info!("Running Example");
                let day_func = self.props.both_func.clone();
                let result = day_func(self.props.example);
                let part1 = format!(
                    "Part 1: {}",
                    self.props
                        .text_format
                        .0
                        .replace("{answer}", &result.0.to_string())
                );
                let part2 = format!(
                    "Part 2: {}",
                    self.props
                        .text_format
                        .1
                        .replace("{answer}", &result.1.to_string())
                );
                log::info!("{}", part1);
                log::info!("{}", part2);
                self.props.messages = vec![part1, part2];
                true
            }
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        // Should only return "true" if new properties are different to
        // previously received properties.
        // This component has no properties so we will always return "false".
        if self.props.messages != props.messages {
            self.props.messages = props.messages;
            true
        } else {
            false
        }
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <h2>{"Day "}{self.props.day_num}{": "}<em>{&self.props.title}</em></h2>
                    <h3>
                        {"Example: "}
                        <pre>{self.props.example}</pre>
                        <button onclick=self.link.callback(|_| Msg::RunExample)>{ "run" }</button>
                        {
                            for self.props.messages.iter().map(|message| {
                                html! {
                                    <p>{message}</p>
                                }
                            })
                        }
                    </h3>
            </div>
        }
    }
}
