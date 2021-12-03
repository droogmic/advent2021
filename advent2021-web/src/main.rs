use std::rc::Rc;

use yew::prelude::*;

use advent2021_lib::get_days;

mod web;

// Use `wee_alloc` as the global allocator.
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

pub enum Msg {
    Run,
}

pub struct Model {
    // `ComponentLink` is like a reference to a component.
    // It can be used to send messages to the component
    link: ComponentLink<Self>,
    days: days::Days,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            days: days::Days {},
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Run => {
                log::info!("Running Something",);
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
        let day_funcs = get_days();
        html! {
            <div>
                <h1>{"Advent of Code"}</h1>
                {
                    for day_funcs.iter().map(|(day_num, day_func)| html!{
                        <web::Day day_num=day_num title=day.title.clone()/>
                    })
                }
            </div>
        }
    }
}

fn main() {
    console_log::init_with_level(log::Level::Trace).expect("logging failed");
    log::trace!("Initializing yew...");
    yew::start_app::<Model>();
}
