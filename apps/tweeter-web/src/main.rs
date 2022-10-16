mod components;
mod config;
mod pages;
mod requests;
mod states;

use yew::prelude::*;
use yew_router::prelude::*;

use components::navbar::Navbar;
use components::Footer;
use pages::list::List;
use pages::register::Register;
use pages::tweet::Tweet;
use pages::user::User;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/list")]
    List,
    #[at("/register")]
    Register,
    #[at("/tweet/:id")]
    Tweet { id: i64 },
    #[at("/user/:pub_key")]
    User { pub_key: String },
}

fn switch(routes: &Route) -> Html {
    match routes {
        Route::Home => html! {}, // FIXME:
        Route::List => html! { <List /> },
        Route::Tweet { id } => html! {
            <Tweet tweet_id={ id.clone() } />
        },
        Route::User { pub_key } => html! {
            <User user_id={ pub_key.clone() } />
        },
        Route::Register => html! { <Register /> },
    }
}

#[function_component(App)]
fn app() -> Html {
    html! {
      <div>
        <Navbar />

        <div class="container p-3">
          <div class="row">
            <div class="col">
              {"left side bar"}
            </div>
            <div class="col-6">
              <BrowserRouter>
                <Switch<Route> render={Switch::render(switch)} />
              </BrowserRouter>
            </div>
            <div class="col">
              {"right side bar"}
            </div>
          </div>
        </div>

        <Footer />
      </div>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::new(config::LOG_LEVEL));
    yew::start_app::<App>();
}
