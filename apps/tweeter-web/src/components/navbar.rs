use yew::prelude::*;
use yew_router::prelude::*;

use crate::Route;

#[function_component(Navbar)]
pub fn navbar() -> Html {
    html! {
        <nav class="navbar navbar-expand-lg navbar-dark bg-dark">
          <div class="container-fluid">
            <Link<Route>
              classes="navbar-brand"
              to={Route::Home}
            >
              { "Tweeter" }
            </Link<Route>>
            <div class="collapse navbar-collapse" id="navbarSupportedContent">
              <ul class="navbar-nav me-auto mb-2 mb-lg-0">
                <li class="nav-item">
                  <Link<Route>
                    classes="nav-link active"
                    to={Route::List}
                  >
                    { "Tweets" }
                  </Link<Route>>
                </li>
                <li class="nav-item">
                  <Link<Route>
                    classes="nav-link active"
                    to={Route::Register}
                  >
                    { "Register" }
                  </Link<Route>>
                </li>
              </ul>
              <form class="d-flex">
                <input
                  class="form-control me-2"
                  type="search"
                  placeholder="Search"
                  aria-label="Search"
                />
                <button class="btn btn-outline-success" type="submit">
                  { "Search" }
                </button>
              </form>
            </div>
          </div>
        </nav>
    }
}
