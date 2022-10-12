use yew::prelude::*;

pub enum Tab {
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
    current_tab: Tab,
    username: String,
    public_key: String,
    private_key: String,
}

pub enum Msg {
    SwitchTab(Tab),
    UpdateContent(FormContent),
}

impl Component for Register {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            current_tab: Tab::GenerateKeys,
            username: String::default(),
            public_key: String::default(),
            private_key: String::default(),
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let generate_keys_active = match self.current_tab {
            Tab::GenerateKeys => Some("active"),
            Tab::RegisterKeys => None,
        };
        let add_existing_active = match self.current_tab {
            Tab::GenerateKeys => None,
            Tab::RegisterKeys => Some("active"),
        };
        html! {
          <>
            <h1> {"Register new account"} </h1>
            <div class="btn-group" role="group">
              <button
               type="button"
               class={classes!("btn", "btn-primary", generate_keys_active)}
               onclick={ctx.link().callback(|_| Msg::SwitchTab(Tab::GenerateKeys))}
              >
                { "Generate keys" }
              </button>
              <button
               type="button"
               class={classes!("btn", "btn-primary", add_existing_active)}
               onclick={ctx.link().callback(|_| Msg::SwitchTab(Tab::RegisterKeys))}
              >
                { "Register existing keys" }
              </button>
            </div>
          {
            match self.current_tab {
                Tab::GenerateKeys => self.view_generate_keys_tab(ctx),
                Tab::RegisterKeys => self.view_add_existing_keys_tab(ctx),
            }
          }
          </>
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
        }
    }
}

impl Register {
    fn view_generate_keys_tab(&self, ctx: &Context<Self>) -> Html {
        html! {
          <form>
            <div class="form-group">
            {
                self.view_username_input(ctx)
            }
            <small id="username-help" class="form-text text-muted">
                { "Enter unique username that will be displayed" }
            </small>
            </div>
            <button type="submit" class="btn btn-primary">
              { "Generate identity" }
            </button>
          </form>
        }
    }

    fn view_add_existing_keys_tab(&self, ctx: &Context<Self>) -> Html {
        html! {
          <form>
            <div class="form-group">
              {
                  self.view_username_input(ctx)
              }
              <label for="public-key-input">
                { "Public key" }
              </label>
              <input
               type="text"
               class="form-control"
               id="public-key-input"
               placeholder="Enter public key"
               value={self.public_key.clone()}
               oninput={ctx.link().callback(|event: InputEvent| {
                 Msg::UpdateContent(FormContent::public_key(event.data()))
               })}
              />
              <label for="private-key-input">
                { "Private key" }
              </label>
              <input
               type="text"
               class="form-control"
               id="private-key-input"
               placeholder="Enter private key"
               value={self.private_key.clone()}
               oninput={ctx.link().callback(|event: InputEvent| {
                 Msg::UpdateContent(FormContent::private_key(event.data()))
               })}
              />
              <small id="private-key-info" class="form-text text-muted">
                { "Your private key won't be sent anywhere, used only for creating signs" }
              </small>
            </div>
            <button type="submit" class="btn btn-primary">
              { "Register" }
            </button>
          </form>
        }
    }

    fn view_username_input(&self, ctx: &Context<Self>) -> Html {
        html! {
          <>
            <label for="username-input">
                { "Username" }
            </label>
            <input
              type="text"
              class="form-control"
              id="username-input"
              placeholder="Enter username"
              value={self.username.clone()}
              oninput={ctx.link().callback(|event: InputEvent| {
                Msg::UpdateContent(FormContent::username(event.data()))
              })}
            />
          </>
        }
    }
}
