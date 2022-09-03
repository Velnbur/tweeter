use tweeter_models::tweet::Tweet;
use tweeter_schemas::{query::Pagination, tweets::TweetListResponse};
use yew::prelude::*;
use yew_router::prelude::Link;

use crate::{
    components::tweet::TweetListComponent,
    requests::{fetch_tweets, FetchState},
    Route,
};

pub struct Home {
    list: FetchState<TweetListResponse>,
}

pub enum Msg {
    SetFetchState(FetchState<TweetListResponse>),
    GetTweets,
}

#[derive(Clone, Debug, Eq, PartialEq, Properties)]
pub struct Props {
    pub page: u64,
}

impl Component for Home {
    type Message = Msg;

    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        ctx.link()
            .send_future(Self::fetch_tweets(true, ctx.props().page));
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
                ctx.link()
                    .send_future(Self::fetch_tweets(true, ctx.props().page));
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
                <div>
            {
                match &self.list {
                    FetchState::Fetching => html! {
                        { "fetching" }
                    },
                    FetchState::Failed(err) => html! {
                        { err.to_string() }
                    },
                    FetchState::Success(content) => {
                        Self::render_list(content)
                    },
                }
            }
                </div>

                <div class="navbar">
                    <Link<Route> to={Route::Home { page: ctx.props().page + 1 }}>
                        { "next" }
                    </Link<Route>>
                </div>
            </div>
        }
    }
}

impl Home {
    async fn fetch_tweets(user: bool, page: u64) -> Msg {
        let mut pagination = Pagination::default();
        pagination.number = page;

        match fetch_tweets(user, pagination).await {
            Ok(tweets) => Msg::SetFetchState(FetchState::Success(tweets)),
            Err(err) => Msg::SetFetchState(FetchState::Failed(err)),
        }
    }

    fn render_list(resp: &TweetListResponse) -> Html {
        let tweets = match Vec::<Tweet>::try_from(resp.to_owned()) {
            Ok(val) => val,
            Err(err) => {
                log::error!("failed to parse tweets list: {err}");
                return html! {
                    { "Failed to get tweet list, something went totally wrong" }
                };
            }
        };

        html! {
            <TweetListComponent tweets={tweets.clone()} />
        }
    }
}
