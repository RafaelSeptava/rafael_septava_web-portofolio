use yew::prelude::*;
use stylist::{yew::styled_component};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub title: String,
    pub profile: String,
    pub interest: String,
    pub goals: String,
    pub hard_skills: String,
    pub soft_skills: String,
}

#[styled_component(Title)]
pub fn title(props: &Props) -> Html {
    html! {
         <header>
            <h1 class="title">{&props.title}</h1>
        <nav>
            <a href="#">{&props.profile}</a>
            <a href="#interest" class="interest">{&props.interest}</a>
            <a href="#goals" class="goals">{&props.goals}</a>
            <a href="#hard_skills" class="hard_skills">{&props.hard_skills}</a>             
            <a href="#soft_skills" class="soft_skills">{&props.soft_skills}</a>
        </nav>
        </header>
    }
}