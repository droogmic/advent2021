use std::collections::HashMap;
use std::rc::Rc;

use yew::prelude::*;
use yew::services::reader::{File, FileData, ReaderService, ReaderTask};
use yew::Properties;

#[derive(Properties, Clone)]
pub struct DayProps {
    pub day_num: usize,
    pub title: &'static str,
    pub example: String,
    #[prop_or_default]
    pub show_input: bool,
    pub both_func: Rc<dyn Fn(&str) -> (String, String)>,
    pub text_format: (&'static str, &'static str),
    #[prop_or_default]
    pub messages: Vec<String>,
}

type FileName = String;

pub enum Msg {
    RunExample,
    File(Option<File>),
    Loaded(FileData),
    Collapse,
}

pub struct Day {
    // `ComponentLink` is like a reference to a component.
    // It can be used to send messages to the component
    link: ComponentLink<Self>,
    props: DayProps,
    tasks: HashMap<FileName, ReaderTask>,
}

impl Component for Day {
    type Message = Msg;
    type Properties = DayProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            props,
            tasks: HashMap::default(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::RunExample => {
                log::info!("Running Example");
                let day_func = self.props.both_func.clone();
                let result = day_func(&self.props.example);
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
            Msg::File(Some(file)) => {
                let file_name = file.name().clone();
                let task = {
                    let callback = self.link.callback(move |data| Msg::Loaded(data));
                    ReaderService::read_file(file, callback).unwrap()
                };
                self.tasks.insert(file_name, task);
                false
            }
            Msg::File(None) => {
                log::warn!("file upload failed");
                false
            }
            Msg::Loaded(file) => {
                let s = match std::str::from_utf8(&file.content) {
                    Ok(v) => v,
                    Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
                }
                .to_owned();
                self.props.example = s;
                let _ = self.tasks.remove(&file.name).expect("no file removed");
                true
            }
            Msg::Collapse => {
                self.props.show_input = !self.props.show_input;
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
                    <h3 type="button" class="collapsible" onclick=self.link.callback(|_| Msg::Collapse)>
                        {"Example: "}
                        <input type="file" onchange=self.link.callback(move |value| {
                                if let ChangeData::Files(files) = value {
                                    assert_eq!(files.length(), 1);
                                    let file = files
                                        .get(0)
                                        .unwrap();
                                    return Msg::File(Some(file))
                                }
                                Msg::File(None)
                            })
                        />
                    </h3>
                    {
                        if self.props.show_input {
                            html! {
                                <pre>{&self.props.example}</pre>
                            }
                        } else {
                            html! {
                                <pre>{self.props.example.lines().next().unwrap()}{"\n..."}</pre>
                            }
                        }
                    }
                    <button onclick=self.link.callback(|_| Msg::RunExample)>{ "run" }</button>
                    {
                        for self.props.messages.iter().map(|message| {
                            html! {
                                <p>{message}</p>
                            }
                        })
                    }
            </div>
        }
    }
}
