use tweeter_models::user::User;
use yew::function_component;
use yew::html;
use yew::Html;
use yew::Properties;

use tweeter_models::tweet::Tweet;

use crate::config::DEFAULT_PROFILE_PICTURE;

#[derive(Properties, PartialEq)]
pub struct TweetProps {
    pub tweet: Tweet,
    pub user: Option<User>,
}

#[function_component(TweetItemComponent)]
pub fn tweet(TweetProps { tweet, user }: &TweetProps) -> Html {
    html! {
        <div class="container">
            <div class="row">
                { view_user(user.clone()) }
                <div class="col text-wrap p-3">
                    <p> { "Text: " } { tweet.text.clone() } </p>
                </div>
            </div>
        </div>
    }
}

fn view_user(user: Option<User>) -> Html {
    if let Some(_user) = user {
        let image_url = match &_user.image_url {
            Some(url) => url.clone(),
            None => DEFAULT_PROFILE_PICTURE.to_string(),
        };

        return html! {
            <div class="col-2 m-2">
                <div class="row-5">
                    <img src={ image_url }
                     alt="profile picture"
                     class="rounded float-start img-fluid profile-image"
                    />
                </div>
                <div class="row-1">
                    <p class="text-center">
                        { _user.username.clone() }
                    </p>
                </div>
            </div>
        };
    }

    Html::default()
}

#[derive(Properties, PartialEq)]
pub struct TweetListProps {
    pub tweets: Vec<Tweet>,
    pub users: Option<Vec<User>>,
}

#[function_component(TweetListComponent)]
pub fn tweet_list(TweetListProps { tweets, users }: &TweetListProps) -> Html {
    let tweets = tweets
        .into_iter()
        .map(|tweet| {
            let user = users.clone().and_then(|_users| {
                _users
                    .into_iter()
                    .find(|user| user.public_key == tweet.user_id)
            });

            html! {
                <a href={ format!("/tweet/{}", tweet.id) }
                 class="list-group-item p-2">
                    <TweetItemComponent tweet={tweet.clone()} user={user.clone()} />
                </a>
            }
        })
        .collect::<Html>();

    html! {
         <div class="list-group">
            { tweets.clone() }
        </div>
    }
}
