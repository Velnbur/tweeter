use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct InputProps {
    pub placeholder: String,
    pub value: String,
    pub label: String,
    pub oninput: Callback<InputEvent>,
}

#[function_component(InputText)]
pub fn input_text(
    InputProps {
        placeholder,
        value,
        label,
        oninput,
    }: &InputProps,
) -> Html {
    html! {
        <>
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
        </>
    }
}
