use tweeter_models::{tweet::Tweet, user::User};
use tweeter_schemas::{query::Pagination, tweets::TweetListResponse};
use yew::prelude::*;
use yew_router::{
    prelude::{History, HistoryListener, Link, Location},
    scope_ext::RouterScopeExt,
};

use crate::{
    components::tweet::TweetListComponent,
    requests::{fetch_tweets, FetchState},
    Route,
};

pub struct List {
    state: FetchState<TweetListResponse>,
    page: u64,
    _listener: HistoryListener,
}

pub enum Msg {
    SetFetchState(FetchState<TweetListResponse>),
    RefreshTweets,
}

impl Component for List {
    type Message = Msg;

    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let link = ctx.link().clone();
        let listener = ctx.link().history().unwrap().listen(move || {
            link.send_message(Msg::RefreshTweets);
        });

        let page = Self::current_page(ctx);

        ctx.link().send_future(Self::fetch_tweets(true, page));

        Self {
            state: FetchState::Fetching,
            page,
            _listener: listener,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::SetFetchState(state) => {
                self.state = state;
                true
            }
            Msg::RefreshTweets => {
                self.page = Self::current_page(ctx);
                ctx.link().send_future(Self::fetch_tweets(true, self.page));
                ctx.link()
                    .send_message(Msg::SetFetchState(FetchState::Fetching));
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class="container">
                <div>
            {
                match &self.state {
                    FetchState::Fetching => html! {
                        { "fetching" }
                    },
                    FetchState::Failed(err) => html! {
                        { err.to_string() }
                    },
                    FetchState::Success(content) => {
                        Self::view_list(content)
                    },
                }
            }
                </div>

                { self.view_navs(ctx) }
            </div>
        }
    }
}

impl List {
    fn view_navs(&self, _ctx: &Context<Self>) -> Html {
        let prev_enabled = if self.page > 0 { "" } else { "disabled" };

        html! {
        <div class="text-center m-4">
            <div class="btn-group" role="group">
                <Link<Route, Pagination>
                    classes={classes!("btn", "btn-secondary", prev_enabled)}
                    to={Route::List}
                    query={Some(Pagination{
                        number: if self.page > 0 {
                            self.page.clone() - 1
                        } else { 0 },
                        ..Pagination::default()
                    })}
                >
                    { "Prev" }
                </Link<Route, Pagination>>
                <Link<Route, Pagination>
                    classes={classes!("btn", "btn-secondary")}
                    to={Route::List}
                    query={Some(Pagination{
                        number: self.page.clone() + 1,
                        ..Pagination::default()
                    })}
                >
                    { "Next" }
                </Link<Route, Pagination>>
            </div>
        </div>
        }
    }

    async fn fetch_tweets(user: bool, page: u64) -> Msg {
        let mut pagination = Pagination::default();
        pagination.number = page;

        match fetch_tweets(user, pagination).await {
            Ok(tweets) => Msg::SetFetchState(FetchState::Success(tweets)),
            Err(err) => Msg::SetFetchState(FetchState::Failed(err)),
        }
    }

    fn view_list(resp: &TweetListResponse) -> Html {
        let tweets = match Vec::<Tweet>::try_from(resp.to_owned()) {
            Ok(val) => val,
            Err(err) => {
                log::error!("failed to parse tweets list: {err}");
                return html! {
                    { "Failed to get tweet list, something went totally wrong" }
                };
            }
        };

        let users = resp.include.to_owned().and_then(|_users| {
            Some(
                _users
                    .into_iter()
                    .map(|user| User::from(user))
                    .collect::<Vec<User>>(),
            )
        });

        html! {
            <TweetListComponent tweets={tweets.clone()} users={users.clone()} />
        }
    }

    fn current_page(ctx: &Context<Self>) -> u64 {
        let location = ctx.link().location().unwrap();

        location
            .query::<Pagination>()
            .map(|it| it.number)
            .unwrap_or(0)
    }
}
