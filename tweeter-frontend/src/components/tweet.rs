use tweeter_models::user::User;
use yew::function_component;
use yew::html;
use yew::Html;
use yew::Properties;

use tweeter_models::tweet::Tweet;

#[derive(Properties, PartialEq)]
pub struct TweetProps {
    pub tweet: Tweet,
    pub user: Option<User>,
}

#[function_component(TweetComponent)]
pub fn tweet(TweetProps { tweet, user }: &TweetProps) -> Html {
    let view_user = if let Some(_user) = user {
        html! {
            <div class="col">
                <div class="row">
                {
                    match &_user.image_url {
                        Some(url) => html! {
                            <img src={ url.clone() }/>
                        },
                        None => html! {
                            { "default img" }
                        }
                    }
                }
                </div>
                <div class="row">
                    { _user.username.clone() }
                </div>
            </div>
        }
    } else {
        Html::default()
    };

    html! {
        <div class="container">
            <div class="row">
                { view_user }
                <div class="col">
                    <p> { "Text: " } { tweet.text.clone() } </p>
                    <a href={ format!("/user/{}", tweet.user_id) }>
                        <p> { "User: " } { tweet.user_id.clone() } </p>
                    </a>
                </div>
            </div>
        </div>
    }
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
                 class="list-group-item">
                    <TweetComponent tweet={tweet.clone()} user={user.clone()} />
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
