use yewdux::prelude::*;

#[derive(Default, Clone, PartialEq, Eq, Store)]
pub struct UserState {
    logined: bool,
    token: Option<String>,
    public_key: String,
    private_key: String,
}
