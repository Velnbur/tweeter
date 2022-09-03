mod components;
mod config;
mod pages;
mod requests;

use pages::home::Home;
use pages::tweet::Tweet;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/:page")]
    Home { page: u64 },
    #[at("/tweet/:id")]
    Tweet { id: i64 },
    // #[at("/user/:pub_key")]
    // User { pub_key: String },
}

fn switch(routes: &Route) -> Html {
    match routes {
        Route::Home { page } => html! { <Home page={ page.clone() } /> },
        Route::Tweet { id } => html! { <Tweet tweet_id= { id.clone() } /> },
    }
}

#[function_component(Main)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={Switch::render(switch)} />
        </BrowserRouter>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::new(log::Level::Debug));
    yew::start_app::<Main>();
}
