use tweeter_models::user::User as UserModel;
use tweeter_schemas::users::UserResponse;
use yew::prelude::*;

use crate::components::user::UserComponent;
use crate::requests::{fetch_user, FetchState};

#[derive(Clone, Debug, Eq, PartialEq, Properties)]
pub struct Props {
    pub user_id: String,
}

pub enum Msg {
    SetFetchingState(FetchState<UserResponse>),
    Refresh,
}

pub struct User {
    state: FetchState<UserResponse>,
}

impl Component for User {
    type Message = Msg;

    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        ctx.link()
            .send_future(Self::fetch_user(ctx.props().user_id.clone()));
        Self {
            state: FetchState::Fetching,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::SetFetchingState(state) => {
                self.state = state;
                true
            }
            Msg::Refresh => {
                ctx.link()
                    .send_future(Self::fetch_user(ctx.props().user_id.clone()));
                ctx.link()
                    .send_message(Msg::SetFetchingState(FetchState::Fetching));
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class="user-view">
                <button onclick={ ctx.link().callback(|_| Msg::Refresh) }>
                    { "refresh" }
                </button>
            {
                match &self.state {
                    FetchState::Fetching => html! {
                        {"fetching"}
                    },
                    FetchState::Success(content) => html! {
                        <UserComponent user = {
                            UserModel::try_from(content.clone()).unwrap()
                        } />
                    },
                    FetchState::Failed(_) => todo!(),
                }
            }
            </div>
        }
    }
}

impl User {
    async fn fetch_user(pub_key: String) -> Msg {
        match fetch_user(&pub_key).await {
            Ok(user) => Msg::SetFetchingState(FetchState::Success(user)),
            Err(err) => Msg::SetFetchingState(FetchState::Failed(err)),
        }
    }
}
