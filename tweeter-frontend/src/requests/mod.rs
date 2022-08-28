use core::fmt;
use std::{
    error::Error,
    fmt::{Debug, Display, Formatter},
};

use tweeter_schemas::tweets::TweetListResponse;
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response};

use crate::config::API_GET_TWEETS_URL;

#[derive(Debug)]
pub struct FetchError {
    err: JsValue,
}

impl Display for FetchError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        Debug::fmt(&self.err, f)
    }
}

impl Error for FetchError {}

impl From<JsValue> for FetchError {
    fn from(value: JsValue) -> Self {
        Self { err: value }
    }
}

pub enum FetchState<T> {
    Fetching,
    Success(T),
    Failed(FetchError),
}

pub async fn fetch_tweets() -> Result<TweetListResponse, FetchError> {
    let mut opts = RequestInit::new();
    opts.method("GET");
    opts.mode(RequestMode::Cors);

    let req = Request::new_with_str_and_init(API_GET_TWEETS_URL, &opts)?;

    let window = gloo_utils::window();
    let resp_value = JsFuture::from(window.fetch_with_request(&req)).await?;

    let resp: Response = resp_value.dyn_into().unwrap();

    let tweets = JsFuture::from(resp.text()?).await?;
    let tweets: TweetListResponse = tweets.into_serde().unwrap();

    Ok(tweets)
}
