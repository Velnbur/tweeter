use yew::prelude::*;

mod components;
mod models;

use components::tweet::TweetListComponent;
use models::tweet::Tweet;

#[function_component(App)]
fn app() -> Html {
    let tweets = vec![
        Tweet {
            text: "some text1".to_string(),
            author: "some author2".to_string(),
            signature: "sldjfksdjflkj".to_string(),
        },
        Tweet {
            text: "some text2".to_string(),
            author: "some author2".to_string(),
            signature: "asdjfsaldjf".to_string(),
        },
    ];
    html! {
        <div>
            <p> { "Tweets" } </p>
            <TweetListComponent tweets={tweets.clone()} />
        </div>
    }
}

fn main() {
    yew::start_app::<App>();
}
