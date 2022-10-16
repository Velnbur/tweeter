use std::rc::Rc;

use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct TabProps<T>
where
    T: PartialEq + Clone,
{
    pub id: T,
    pub label: String,
    pub onclick: Callback<MouseEvent>,
    pub active: Option<bool>,
    pub children: Children,
}

#[function_component(Tab)]
pub fn tab<T>(
    TabProps {
        id,
        label,
        onclick,
        active,
        children,
    }: &TabProps<T>,
) -> Html
where
    T: PartialEq + Clone,
{
    let active_class = active.and_then(|act| if act { Some("active") } else { None });
    html! {
        <button
            type="button"
            class={classes!("btn", "btn-primary", active_class)}
            onclick={onclick}
        >
            {label}
        </button>
    }
}

#[derive(Properties, PartialEq)]
pub struct TabGroupProps<T>
where
    T: PartialEq + Clone + 'static,
{
    pub active: T,
    pub children: ChildrenWithProps<Tab<T>>,
}

#[function_component(TabGroup)]
pub fn tab_group<T>(TabGroupProps { children, active }: &TabGroupProps<T>) -> Html
where
    T: PartialEq + Clone + 'static,
{
    let mut active_body = Children::default();

    html! {
        <>
            <div class="btn-group" role="group">
        {
            for children.iter().map(|mut btn| {
                let mut props = Rc::make_mut(&mut btn.props);

                if props.id.eq(active) {
                    props.active = Some(true);
                    active_body = props.children.clone();
                } else {
                    props.active = Some(false);
                }
                btn
            })
        }
            </div>

            {active_body}
        </>
    }
}
