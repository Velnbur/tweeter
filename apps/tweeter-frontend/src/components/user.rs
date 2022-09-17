use tweeter_models::user::User;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub user: User,
}

#[function_component(UserComponent)]
pub fn user(Props { user }: &Props) -> Html {
    html! {
        <div class="user">
            <p> { "User: " } { user.username.clone() } </p>
            <p> { "Public key: "} { user.public_key.clone() } </p>
        {
            match &user.image_url {
                Some(url) => html! {
                    <img src={ url.clone() }/>
                },
                None => html! {
                    { "default img" }
                }
            }
        }
        </div>
    }
}
