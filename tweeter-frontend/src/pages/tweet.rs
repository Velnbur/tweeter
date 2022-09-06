use tweeter_models::tweet::Tweet as TweetModel;
use tweeter_schemas::tweets::TweetResponse;
use yew::prelude::*;

use crate::components::tweet::TweetItemComponent;
use crate::requests::{fetch_tweet, FetchState};

#[derive(Clone, Debug, Eq, PartialEq, Properties)]
pub struct Props {
    pub tweet_id: i64,
}

pub enum Msg {
    SetFetchingState(FetchState<TweetResponse>),
    RefreshTweet,
}

pub struct Tweet {
    state: FetchState<TweetResponse>,
}

impl Component for Tweet {
    type Message = Msg;

    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        ctx.link()
            .send_future(Self::fetch_tweet(ctx.props().tweet_id));
        Self {
            state: FetchState::Fetching,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::SetFetchingState(state) => {
                self.state = state;
                true
            }
            Msg::RefreshTweet => {
                ctx.link()
                    .send_future(Self::fetch_tweet(ctx.props().tweet_id));
                ctx.link()
                    .send_message(Msg::SetFetchingState(FetchState::Fetching));
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class="tweet-view">
                <button onclick={ ctx.link().callback(|_| Msg::RefreshTweet) }>
                    { "refresh" }
                </button>
            {
                match &self.state {
                    FetchState::Fetching => html! {
                        {"fetching"}
                    },
                    FetchState::Success(content) => html! {
                        <TweetItemComponent tweet = { TweetModel::try_from(content.clone()).unwrap() } />
                    },
                    FetchState::Failed(_) => todo!(),
                }
            }
            </div>
        }
    }
}

impl Tweet {
    async fn fetch_tweet(id: i64) -> Msg {
        match fetch_tweet(id, true).await {
            Ok(tweet) => Msg::SetFetchingState(FetchState::Success(tweet)),
            Err(err) => Msg::SetFetchingState(FetchState::Failed(err)),
        }
    }
}
