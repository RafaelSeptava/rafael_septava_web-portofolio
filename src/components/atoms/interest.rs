use yew::prelude::*;
use stylist::{yew::styled_component};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub linux: String,
    pub vscode: String,
}

#[styled_component(Interest)]
pub fn interest(props: &Props) -> Html {
    let tech_user = vec!["Information Technology (IT)", 
                                    "Software Development & Programming", 
                                    "Linux Systems & Open Source Technologies"];
    
    html! {
        <section class="interest" id="interest">
            <div class="content">
                <ul>
                    <p class="title_interest">{"Interest: "}</p>
                    {list_to_html(tech_user)}
                </ul>
                <div class="image_interest">
                    <img src={props.linux.clone()} alt="image Linux"/>
                    <img src={props.vscode.clone()} alt="image VSCode" />
                </div>
            </div>
        </section>
    }
}

fn list_to_html(list: Vec<&str>) -> Vec<Html> {
    list.iter().map(|item| html!{<li>{item}</li>}).collect()
}