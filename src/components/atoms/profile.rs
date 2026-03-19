use yew::prelude::*;
use stylist::{yew::styled_component};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub fullname: String,
    pub nickname: String,
    pub age: String,
    pub gender: String,
    pub profession: String,
    pub address: String,
    pub image: String,
}

#[styled_component(Profile)]
pub fn profile(props: &Props) -> Html {
    html! {
        <section class="profile">
            <div class="image_profile">
                <img src={props.image.clone()} alt="image profile"/>
            </div>
            <div class="data_profile">
                <p>{"I'm "}{&props.fullname}</p>
                <p>{"You can call me "}{&props.nickname}</p>
                    <ul>
                        <li>{"Age (Currently): "}{&props.age}</li>
                        <li>{"Gender: "}{&props.gender}</li>
                        <li>{"Profession: "}{&props.profession}</li>
                        <li>{"Address: "}{&props.address}</li>
                    </ul>
                <div class="social_media">
                    <a href="https://www.instagram.com/rafaelseptava/" target="_blank">
                        <i class="fab fa-instagram"></i>
                    </a>
                    <a href="https://github.com/RafaelSeptava">
                        <i class="fab fa-github"></i>
                    </a>
                    <a href="https://wa.me/+6282133453216">
                        <i class="fab fa-whatsapp"></i>
                    </a>
                </div>
            </div>
        </section>
    }
}