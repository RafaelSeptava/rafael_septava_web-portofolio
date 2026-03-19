use yew::prelude::*;
use stylist::{yew::styled_component, Style};

mod components;

use components::atoms::title::Title;
use components::atoms::profile::Profile;
use components::atoms::interest::Interest;
use components::atoms::goals::Goals;
use components::atoms::hard_skills::HardSkills;
use components::atoms::soft_skills::SoftSkills;

const STYLE_FILE: &str = include_str!("style.css");

#[styled_component(App)]
pub fn app() -> Html {
    

    let stylesheet = Style::new(STYLE_FILE.to_string()).expect("CSS file not found");

    html! {
        <div class={stylesheet}>
            <Title  title="Rafael Septava Web" profile="Profile" interest="Interest" goals="Goals" 
                    hard_skills="Hard Skills" soft_skills="Soft Skills"/>
            <Profile    image="images/RafaelSeptava.jpeg" 
                        fullname="Rafael Putra Septava" nickname="Afel" age="20" gender="Male" 
                        profession="Software Engineering student in Telkom University Purwokerto" 
                        address="Purbalingga, Central Java, Indonesia"
                        />
            <Interest   linux="images/linux.png" vscode="images/vscode.png" />
            <Goals      arch="images/arch.png" rust="images/rust.png" />
            <HardSkills html="HTML" css="CSS" js="JavaScript" rust="Rust" c="C" c_plus_plus="C++"
                        html_img="images/html.png" css_img="images/css.png" js_img="images/js.png" 
                        rust_img="images/rust.png" c_img="images/c.png" c_plus_plus_img="images/c++.png" />
            <SoftSkills responsibility="Responsibility" time_management="Time Management" discipline="Discipline" 
                        creativity="Creativity"  problem_solving="Problem Solving"/>
            
        </div>
    }
}