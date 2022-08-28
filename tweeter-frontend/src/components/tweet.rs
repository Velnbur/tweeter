use yew::function_component;
use yew::html;
use yew::Html;
use yew::Properties;

use crate::models::tweet::Tweet;

#[derive(Properties, PartialEq)]
pub struct TweetProps {
    pub tweet: Tweet,
}

#[function_component(TweetComponent)]
pub fn tweet(TweetProps { tweet }: &TweetProps) -> Html {
    html! {
        <div>
            <p> { tweet.text.clone() } </p>
            <p> { tweet.author.clone() } </p>
            <p> { tweet.signature.clone() } </p>
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct TweetListProps {
    pub tweets: Vec<Tweet>,
}

#[function_component(TweetListComponent)]
pub fn tweet_list(TweetListProps { tweets }: &TweetListProps) -> Html {
    let tweets = tweets
        .into_iter()
        .map(|tweet| {
            html! {
                <li> <TweetComponent tweet={tweet.clone()} /> </li>
            }
        })
        .collect::<Html>();
    html! {
        <ul>
            { tweets }
        </ul>
    }
}
