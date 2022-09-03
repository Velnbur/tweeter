use core::fmt;
use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;

use tweeter_schemas::{
    query::Pagination,
    tweets::{TweetListResponse, TweetResponse},
};
use url::Url;
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response};

use crate::config::API_TWEETS_URL;

#[derive(Debug)]
pub struct FetchError {
    err: JsValue,
}

impl Display for FetchError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        Debug::fmt(&self.err, f)
    }
}

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

pub async fn fetch_tweets(
    user: bool,
    pages: Pagination, // TODO: add pagination rendering
) -> Result<TweetListResponse, FetchError> {
    let mut url = Url::parse(API_TWEETS_URL).unwrap();

    if user {
        url.query_pairs_mut().append_pair("include", "user");
    }

    let mut opts = RequestInit::new();
    opts.method("GET");
    opts.mode(RequestMode::Cors);

    let req = Request::new_with_str_and_init(url.as_str(), &opts)?;

    let resp = fetch(req).await?;

    let tweets: TweetListResponse = resp.into_serde().unwrap();

    Ok(tweets)
}

pub async fn fetch_tweet(id: i64, user: bool) -> Result<TweetResponse, FetchError> {
    let raw = format!("{}/{}", API_TWEETS_URL, id);

    let mut url = Url::parse(&raw).unwrap();

    if user {
        url.query_pairs_mut().append_pair("include", "user");
    }

    let mut opts = RequestInit::new();
    opts.method("GET");
    opts.mode(RequestMode::Cors);

    let req = Request::new_with_str_and_init(url.as_str(), &opts)?;

    let resp = fetch(req).await?;

    let tweet: TweetResponse = resp.into_serde().unwrap();

    Ok(tweet)
}

async fn fetch(req: Request) -> Result<JsValue, FetchError> {
    let window = gloo_utils::window();
    let resp_value = JsFuture::from(window.fetch_with_request(&req)).await?;

    let resp: Response = resp_value.dyn_into()?;

    let res = JsFuture::from(resp.json()?).await?;

    Ok(res)
}
