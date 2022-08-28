use tweeter_schemas::tweets::TweetListResponse;
use yew::prelude::*;

mod components;
mod config;
mod models;
mod requests;

use components::tweet::TweetListComponent;
use models::tweet::Tweet;
use requests::{fetch_tweets, FetchState};

pub struct App {
    list: FetchState<TweetListResponse>,
}

pub enum Msg {
    SetFetchState(FetchState<TweetListResponse>),
    GetTweets,
}

impl Component for App {
    type Message = Msg;

    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            list: FetchState::Fetching,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::SetFetchState(state) => {
                self.list = state;
                true
            }
            Msg::GetTweets => {
                ctx.link().send_future(async {
                    match fetch_tweets().await {
                        Ok(tweets) => Msg::SetFetchState(FetchState::Success(tweets)),
                        Err(err) => Msg::SetFetchState(FetchState::Failed(err)),
                    }
                });
                ctx.link()
                    .send_message(Msg::SetFetchState(FetchState::Fetching));
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class="container">
                <button onclick={ctx.link().callback(|_| Msg::GetTweets)}>
                    { "Update list" }
                </button>
            {
                match &self.list {
                    FetchState::Fetching => html! {
                        { "fetching" }
                    },
                    FetchState::Success(content) => html! {
                        <TweetListComponent tweets={content.data.clone().into_iter().map(|tweet| Tweet {
                            text: tweet.attributes.text,
                            author: tweet.relationships.author.data.id,
                            signature: tweet.attributes.signature,
                        }).collect::<Vec<Tweet>>()} />
                    },
                    FetchState::Failed(err) => {
                        log::error!("failed to fetch {err}");
                        html! {
                            { "bug" }
                        }
                    }
                }
            }
            </div>
        }
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<App>();
}
