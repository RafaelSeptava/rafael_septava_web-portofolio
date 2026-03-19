use yew::prelude::*;
use stylist::{yew::styled_component};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub html: String,
    pub html_img: String,
    pub css: String,
    pub css_img: String,
    pub js: String,
    pub js_img: String,
    pub rust: String,
    pub rust_img: String,
    pub c: String,
    pub c_img: String,
    pub c_plus_plus: String,
    pub c_plus_plus_img: String,
}

#[styled_component(HardSkills)]
pub fn hard_skills(props: &Props) -> Html {
    html! {
        <section class="hard_skills" id="hard_skills">
            <div class="content">
                <p class="skills_title">{"Hard Skills: "}</p>                
                <div class="skills_boxes">
                    <div class="skills_box">
                        <div class="skills_name">{&props.html}</div>
                        <img src={props.html_img.clone()} alt="image HTML"/>
                    </div>
                    <div class="skills_box">
                        <div class="skills_name">{&props.css}</div>
                        <img src={props.css_img.clone()} alt="image CSS"/>
                    </div>
                    <div class="skills_box">
                        <div class="skills_name">{&props.js}</div>
                        <img src={props.js_img.clone()} alt="image JS"/>
                    </div>
                    <div class="skills_box">
                        <div class="skills_name">{&props.rust}</div>
                        <img src={props.rust_img.clone()} alt="image Rust"/>
                    </div>
                    <div class="skills_box">
                        <div class="skills_name">{&props.c}</div>
                        <img src={props.c_img.clone()} alt="image C"/>
                    </div>
                    <div class="skills_box">
                        <div class="skills_name">{&props.c_plus_plus}</div>
                        <img src={props.c_plus_plus_img.clone()} alt="image C++"/>
                    </div>
                </div>
            </div>
        </section>
    }
}