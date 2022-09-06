use const_format::concatcp;

pub const API_BASE_URL: &'static str = "http://localhost:8080";

pub const API_TWEETS_URL: &'static str = concatcp!(API_BASE_URL, "/api/tweets");
pub const API_USERS_URL: &'static str = concatcp!(API_BASE_URL, "/api/users");

pub const DEFAULT_PROFILE_PICTURE: &'static str = "/img/profile-picture.png";
