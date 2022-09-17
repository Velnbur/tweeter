mod components;
mod config;
mod pages;
mod requests;

use pages::list::List;
use pages::tweet::Tweet;
use pages::user::User;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/list/")]
    List,
    #[at("/tweet/:id")]
    Tweet { id: i64 },
    #[at("/user/:pub_key")]
    User { pub_key: String },
}

fn switch(routes: &Route) -> Html {
    match routes {
        Route::List => html! { <List /> },
        Route::Tweet { id } => html! { <Tweet tweet_id={ id.clone() } /> },
        Route::User { pub_key } => html! {
            <User user_id={ pub_key.clone() } />
        },
    }
}

#[function_component(Main)]
fn app() -> Html {
    html! {
        <div class="containter">
            <nav class="navbar navbar-default">
                <div class="container-fluid">
                    <div class="navbar-header">
                        <a class="navbar-brand" href="/"> { "Tweeter" } </a>
                    </div>
                    <ul class="nav navbar-nav">
                         <li class="nav-item">
                            <a class="nav-link" href="/list/"> { "List" } </a>
                        </li>
                    </ul>
                </div>
            </nav>
            <div class="container main">
                <BrowserRouter>
                    <Switch<Route> render={Switch::render(switch)} />
                </BrowserRouter>
            </div>
        </div>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::new(log::Level::Debug));
    yew::start_app::<Main>();
}
