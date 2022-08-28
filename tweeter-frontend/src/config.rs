use const_format::concatcp;

pub const API_BASE_URL: &'static str = "http://localhost:8000";

pub const API_GET_TWEETS_URL: &'static str = concatcp!(API_BASE_URL, "/api/tweets");
