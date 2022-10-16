use tweeter_schemas::auth_keys::AuthKeysResponse;
use yewdux::prelude::*;

#[derive(Default, Clone, PartialEq, Eq, Store)]
pub struct UserState {
    logined: bool,
    token: Option<String>,
    public_key: String,
    private_key: String,
}

impl UserState {
    pub fn from_auth_keys(resp: AuthKeysResponse) -> Self {
        Self {
            logined: true,
            token: None,
            public_key: resp.data.attributes.public_key,
            private_key: resp.data.attributes.private_key,
        }
    }
}
