use yew::prelude::*;
use stylist::{yew::styled_component};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub arch: String,
    pub rust: String,
}

#[styled_component(Goals)]
pub fn goals(props: &Props) -> Html {
    let tech_user = vec!["Master the Rust programming language", 
                                    "Build scalable Frontend/Backend applications with Rust", 
                                    "Fully adopt Arch Linux for daily development and productivity"];
    
    html! {
        <section class="goals" id="goals">
            <div class="content">
                <ul>
                <p class="title_goals">{"Goals: "}</p>
                    {list_to_html(tech_user)}
                </ul>
                <div class="image_goals">
                    <img src={props.arch.clone()} class="arch" alt="image Arch Linux"/>
                    <img src={props.rust.clone()} class="rust" alt="image Rust Language" />
                </div>
            </div>
        </section>
    }
}

fn list_to_html(list: Vec<&str>) -> Vec<Html> {
    list.iter().map(|item| html!{<li>{item}</li>}).collect()
}