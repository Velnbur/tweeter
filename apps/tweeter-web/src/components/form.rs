use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct InputProps {
    pub placeholder: String,
    pub value: String,
    pub label: String,
    pub oninput: Callback<InputEvent>,
    pub help_text: Option<String>,
}

#[function_component(InputText)]
pub fn input_text(
    InputProps {
        placeholder,
        value,
        label,
        oninput,
        help_text,
    }: &InputProps,
) -> Html {
    html! {
        <div class="form-group">
            <label>
                {label}
            </label>
            <input
                type="text"
                class="form-control"
                placeholder={placeholder.clone()}
                value={value.clone()}
                oninput={oninput}
            />
            {
                match help_text {
                    Some(content) => html!{
                        <small class="form-text text-muted">
                            {content}
                        </small>
                    },
                    None => html!{},
                }
            }
        </div>
    }
}
