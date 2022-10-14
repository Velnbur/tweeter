use yew::prelude::*;
use yew::Callback;

#[derive(Properties, PartialEq)]
pub struct SubmitProps {
    pub text: String,
    pub disabled: Option<()>,
    pub onclick: Callback<MouseEvent>,
}

#[function_component(SubmitButton)]
pub fn submit_button(
    SubmitProps {
        text,
        disabled,
        onclick,
    }: &SubmitProps,
) -> Html {
    let disable_class = disabled.and(Some("disabled"));
    html! {
        <button
            type="submit"
            class={classes!("btn", "btn-primary", disable_class)}
            onclick={onclick}
        >
            {text}
        </button>
    }
}

#[function_component(LoadingButton)]
pub fn loading_button() -> Html {
    html! {
        <button class="btn btn-primary disabled" type="button">
            <span
                class="spinner-border spinner-border-sm"
                role="status"
                aria-hidden="true"
            >
            </span>
                { "Loading..." }
        </button>
    }
}
