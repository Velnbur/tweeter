pub mod buttons;
pub mod form;
pub mod navbar;
pub mod tabs;
pub mod tweet;
pub mod user;

use crate::config::GITHUB_LINK;
use yew::prelude::*;

#[function_component(Footer)]
pub fn footer() -> Html {
    html! {
    <footer class="bg-dark text-center text-white fixed-bottom">
      //<!-- Grid container -->
      <div class="container p-4 pb-0">
        //<!-- Section: Social media -->
        <section class="mb-4">
          //<!-- Github -->
          <a class="btn btn-outline-light btn-floating m-1" href={GITHUB_LINK} role="button">
            <i class="bi bi-github"></i>
          </a>
        </section>
        //<!-- Section: Social media -->
      </div>
      //<!-- Grid container -->

      //<!-- Copyright -->
      <div class="text-center p-3" style="background-color: rgba(0, 0, 0, 0.2);">
        { "© 2020 Copyright:" }
        <a class="text-white" href="#">{"Tweeter.com"}</a>
      </div>
      //<!-- Copyright -->
    </footer>
    }
}
