use yew::prelude::*;
use stylist::{yew::styled_component};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub responsibility: String,
    pub time_management: String,
    pub discipline: String,
    pub creativity: String,
    pub problem_solving: String,
}

#[styled_component(SoftSkills)]
pub fn soft_skills(props: &Props) -> Html {
    html! {
        <section class="soft_skills" id="soft_skills">
            <div class="content">
                <p class="skills_title">{"Soft Skills: "}</p>
                <div class="skills_boxes">
                    <div class="skills_box">
                        <div class="skills_name">{&props.responsibility}</div>
                    </div>
                    <div class="skills_box">
                        <div class="skills_name">{&props.time_management}</div>
                    </div>
                    <div class="skills_box">
                        <div class="skills_name">{&props.discipline}</div>
                    </div>
                    <div class="skills_box">
                        <div class="skills_name">{&props.creativity}</div>
                    </div>
                    <div class="skills_box">
                        <div class="skills_name">{&props.problem_solving}</div>
                    </div>
                </div>
            </div>
        </section>
    }
}