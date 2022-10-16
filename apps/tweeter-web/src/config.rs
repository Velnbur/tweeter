use const_format::concatcp;

pub const API_BASE_URL: &'static str = "http://localhost:80";

pub const API_TWEETS_URL: &'static str = concatcp!(API_BASE_URL, "/api/tweets");
pub const API_USERS_URL: &'static str = concatcp!(API_BASE_URL, "/api/users");

pub const API_REGISTER: &'static str = concatcp!(API_BASE_URL, "/api/auth/register");

pub const GITHUB_LINK: &'static str = "https://github.com/Velnbur/tweeter";
pub const DEFAULT_PROFILE_PICTURE: &'static str = "/img/profile-picture.png";

pub const LOG_LEVEL: log::Level = log::Level::Debug;
