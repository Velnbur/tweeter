use tweeter_schemas::{auth_keys::AuthKeysResponse, users::UserResponse};
use yew::prelude::*;
use yewdux::prelude::use_store;

use crate::components::buttons::{LoadingButton, SubmitButton};
use crate::components::form::InputText;
use crate::components::tabs::{Tab, TabGroup};
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

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::SwitchTab(tab) => {
                self.current_tab = tab;
                true
            }
            Msg::UpdateContent(form) => {
                let mut update = false;

                if let Some(username) = form.username {
                    self.username.push_str(username.as_str());
                    update = true;
                }

                if let Some(private_key) = form.private_key {
                    self.private_key.push_str(private_key.as_str());
                    update = true;
                }

                if let Some(public_key) = form.public_key {
                    self.public_key.push_str(public_key.as_str());
                    update = true;
                }

                update
            }
            Msg::UpdateState(state) => match &state {
                State::Input => {
                    self.state = state;
                    false
                }
                State::Send => {
                    self.state = state;
                    true
                }
                State::SuccessGenerateKeys(resp) => {
                    self.state = state.clone();

                    let (_, dispatch) = use_store::<UserState>();
                    dispatch.set(UserState::from_auth_keys(resp.clone()));

                    self.private_key = resp.data.attributes.private_key.clone();
                    self.public_key = resp.data.attributes.public_key.clone();
                    true
                }
                State::SuccessRegister(_) => todo!(),
            },
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
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
                        {self.view_generate_keys_tab(ctx)}
                    </Tab<Tabs>>
                    <Tab<Tabs>
                      id={ Tabs::RegisterKeys }
                      label={ "Register existing keys" }
                      onclick={ctx.link().callback(|_| {
                          Msg::SwitchTab(Tabs::RegisterKeys)
                      })}
                    >
                        {self.view_add_existing_keys_tab(ctx)}
                    </Tab<Tabs>>
                </TabGroup<Tabs>>
            </>
        }
    }
}

impl Register {
    fn view_generate_keys_tab(&self, ctx: &Context<Self>) -> Html {
        html! {
            <form>
                <div class="form-group">
                {self.view_username_input(ctx)}
                <small id="username-help" class="form-text text-muted">
                    { "Enter unique username" }
                </small>
                </div>
                {self.view_submit_btn(ctx)}
            </form>
        }
    }

    fn view_add_existing_keys_tab(&self, ctx: &Context<Self>) -> Html {
        html! {
              <form>
                  <div class="form-group">
                      {self.view_username_input(ctx)}
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
                      />
                      <small id="private-key-info" class="form-text text-muted">
                          { "Your private key won't be sent anywhere, used only for creating signs" }
                      </small>
                  </div>
                  {self.view_submit_btn(ctx)}
              </form>
        }
    }

    fn view_username_input(&self, ctx: &Context<Self>) -> Html {
        html! {
            <InputText
                label={ "Username" }
                placeholder={ "Enter username" }
                value={self.username.clone()}
                oninput={ctx.link().callback(|event: InputEvent| {
                    Msg::UpdateContent(FormContent::username(event.data()))
                })}
            />
        }
    }

    fn view_submit_btn(&self, ctx: &Context<Self>) -> Html {
        html! {{
              match self.state {
                  State::Input => html! {
                      <SubmitButton
                        onclick={ctx.link().callback(|_| Msg::UpdateState(State::Send))}
                        text={ "Generate identity" }
                      />
                  },
                  State::Send => html! {
                      <LoadingButton />
                  },
                  _ => html! {},
              }
        }}
    }
}
