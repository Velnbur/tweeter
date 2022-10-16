use gloo_net::http::Request;
use tweeter_schemas::users::{CreateUser, CreateUserRequest};
use tweeter_schemas::{auth_keys::AuthKeysResponse, users::UserResponse};
use yew::prelude::*;
use yewdux::prelude::use_store;

use crate::components::buttons::SubmitButton;
use crate::components::form::InputText;
use crate::components::tabs::{Tab, TabGroup};
use crate::config::API_REGISTER;
use crate::states::UserState;

#[derive(Clone, PartialEq)]
pub enum Tabs {
    GenerateKeys,
    RegisterKeys,
}

pub struct FormContent {
    username: Option<String>,
    private_key: Option<String>,
    public_key: Option<String>,
}

impl FormContent {
    pub fn username(value: Option<String>) -> FormContent {
        Self {
            username: value,
            private_key: None,
            public_key: None,
        }
    }

    pub fn private_key(value: Option<String>) -> FormContent {
        Self {
            username: None,
            private_key: value,
            public_key: None,
        }
    }

    pub fn public_key(value: Option<String>) -> FormContent {
        Self {
            username: None,
            private_key: None,
            public_key: value,
        }
    }
}

pub struct Register {
    current_tab: Tabs,
    username: String,
    public_key: String,
    private_key: String,
    state: State,
}

#[derive(Clone)]
pub enum State {
    Input,
    Send,
    SuccessGenerateKeys(AuthKeysResponse),
    SuccessRegister(UserResponse),
    Failure,
}

pub enum Msg {
    SwitchTab(Tabs),
    UpdateContent(FormContent),
    UpdateState(State),
}

impl Component for Register {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            current_tab: Tabs::GenerateKeys,
            username: String::default(),
            public_key: String::default(),
            private_key: String::default(),
            state: State::Input,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::SwitchTab(tab) => {
                self.current_tab = tab;
                true
            }
            Msg::UpdateContent(form) => {
                match form.username {
                    Some(input) => {
                        self.username.push_str(input.as_str());
                    }
                    None => {
                        self.username.pop();
                    }
                };

                match form.private_key {
                    Some(input) => {
                        self.private_key.push_str(input.as_str());
                    }
                    None => {
                        self.private_key.pop();
                    }
                };

                match form.public_key {
                    Some(input) => {
                        self.public_key.push_str(input.as_str());
                    }
                    None => {
                        self.public_key.pop();
                    }
                };

                true
            }
            Msg::UpdateState(state) => match state {
                State::Input => {
                    self.state = state;
                    false
                }
                State::Send => {
                    self.state = state;
                    ctx.link()
                        .send_future(Self::create_user(self.username.clone()));
                    true
                }
                State::SuccessGenerateKeys(resp) => {
                    self.state = State::SuccessGenerateKeys(resp.clone());

                    let (_, dispatch) = use_store::<UserState>();
                    dispatch.set(UserState::from_auth_keys(resp.clone()));

                    self.private_key = resp.data.attributes.private_key.clone();
                    log::debug!("Private key: {}", self.private_key);
                    self.public_key = resp.data.attributes.public_key.clone();
                    log::debug!("Public key: {}", self.public_key);
                    true
                }
                State::SuccessRegister(_) => todo!(),
                State::Failure => {
                    self.state = state;
                    true
                }
            },
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        match self.state {
            State::Input => self.register_form(ctx),
            State::Send => self.loading_view(ctx),
            State::SuccessGenerateKeys(_) => todo!(),
            State::SuccessRegister(_) => todo!(),
            State::Failure => html! {
                {"Something went wrong"}
            },
        }
    }
}

impl Register {
    async fn create_user(username: String) -> Msg {
        let body = CreateUserRequest {
            data: CreateUser::new(username),
        };

        let body = match serde_json::to_string(&body) {
            Ok(body) => body,
            Err(err) => {
                log::error!("Failed to create request {err}");
                return Msg::UpdateState(State::Failure);
            }
        };

        let resp = Request::post(API_REGISTER)
            .body(body)
            .header("Content-Type", "application/json")
            .send()
            .await;

        let resp = match resp {
            Ok(resp) => resp,
            Err(err) => {
                log::error!("failed to get response: {err}");
                return Msg::UpdateState(State::Failure);
            }
        };

        match resp.status() {
            200 => {}
            _ => {
                log::debug!("user creation status code: {}", resp.status());
                return Msg::UpdateState(State::Failure);
            }
        }

        let auth_keys: AuthKeysResponse = match resp.json().await {
            Ok(keys) => keys,
            Err(err) => {
                log::error!("failed to get response: {err}");
                return Msg::UpdateState(State::Failure);
            }
        };

        Msg::UpdateState(State::SuccessGenerateKeys(auth_keys))
    }

    fn register_form(&self, ctx: &Context<Self>) -> Html {
        html! {
            <>
                <h1>{ "Register new account" }</h1>
                <TabGroup<Tabs>
                  active={self.current_tab.clone()}
                >
                    <Tab<Tabs>
                      id={ Tabs::GenerateKeys }
                      label={ "Generate keys" }
                      onclick={ctx.link().callback(|_| {
                          Msg::SwitchTab(Tabs::GenerateKeys)
                      })}
                    >
                        {self.generate_keys_tab(ctx)}
                    </Tab<Tabs>>

                    <Tab<Tabs>
                      id={ Tabs::RegisterKeys }
                      label={ "Register existing keys" }
                      onclick={ctx.link().callback(|_| {
                          Msg::SwitchTab(Tabs::RegisterKeys)
                      })}
                    >
                        {self.add_existing_keys_tab(ctx)}
                    </Tab<Tabs>>

                </TabGroup<Tabs>>
            </>
        }
    }

    fn generate_keys_tab(&self, ctx: &Context<Self>) -> Html {
        html! {
            <form>
                {self.username_input(ctx)}
                {self.submit_btn(ctx)}
            </form>
        }
    }

    fn add_existing_keys_tab(&self, ctx: &Context<Self>) -> Html {
        html! {
            <form>
                {self.username_input(ctx)}
                <InputText
                  label={ "Public key" }
                  placeholder={ "Enter public key" }
                  value={self.public_key.clone()}
                  oninput={ctx.link().callback(|event: InputEvent| {
                      Msg::UpdateContent(FormContent::public_key(event.data()))
                  })}
                />
                <InputText
                    label={ "Private key" }
                    placeholder={ "Enter private key" }
                    value={ self.private_key.clone() }
                    oninput={ctx.link().callback(|event: InputEvent| {
                        Msg::UpdateContent(FormContent::private_key(event.data()))
                    })}
                    help_text={ "Your private key won't be sent anywhere,\
                                 used only for creating signs" }
                />
                {self.submit_btn(ctx)}
            </form>
        }
    }

    fn username_input(&self, ctx: &Context<Self>) -> Html {
        html! {
            <InputText
                label={ "Username" }
                placeholder={ "Enter username" }
                value={self.username.clone()}
                oninput={ctx.link().callback(|event: InputEvent| {
                    Msg::UpdateContent(FormContent::username(event.data()))
                })}
                help_text={ "Enter unique username" }
            />
        }
    }

    fn submit_btn(&self, ctx: &Context<Self>) -> Html {
        html! {
            <SubmitButton
                onclick={ctx.link().callback(|_| Msg::UpdateState(State::Send))}
                text={ "Generate identity" }
            />
        }
    }

    fn loading_view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div class="d-flex justify-content-center">
                <div class="spinner-border m-5" style="width: 3rem; height: 3rem;" role="status">
                    <span class="visually-hidden">
                        { "Generating identity. Please, wait..." }
                    </span>
                </div>
            </div>
        }
    }
}
