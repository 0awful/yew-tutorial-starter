use crate::components::atoms::custom_button::CustomButton;
use crate::components::atoms::text_input::TextInput;
use crate::components::pages::home::Context;
use serde::{Deserialize, Serialize};
use std::ops::Deref;
use stylist::{yew::styled_component, Style};
use yew::prelude::*;

#[derive(Deserialize, Serialize, Default, Clone)]
pub struct State {
    pub username: String,
    pub language: String,
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub on_submit: Callback<State>,
}

#[styled_component(CustomForm)]
pub fn custom_div(props: &Props) -> Html {
    let styles: Style = Style::new(
        r#"
            * {
                color: black;
            }
        "#,
    )
    .expect("Failed to create style");

    let state = use_state(|| State::default());
    let context = use_context::<Context>();

    let cloned_state = state.clone();
    let username_changed = Callback::from(move |username| {
        cloned_state.set(State {
            username,
            ..cloned_state.deref().clone()
        });
    });

    let cloned_state = state.clone();
    let language_changed = Callback::from(move |language| {
        cloned_state.set(State {
            language,
            ..cloned_state.deref().clone()
        });
    });

    let cloned_state = state.clone();
    let from_on_submit = props.on_submit.clone();
    let on_submit = Callback::from(move |event: SubmitEvent| {
        event.prevent_default();
        from_on_submit.emit(cloned_state.deref().clone())
    });

    html! {
        <>
            <form class={styles} onsubmit={on_submit}>
                <TextInput name="username" handle_onchange={username_changed} />
                <TextInput name="favorite_language" handle_onchange={language_changed} />
                <CustomButton label="submit" />
            </form>
            <p>{"username: "}{context.clone().unwrap_or_default().username}</p>
            <p>{"language: "}{context.clone().unwrap_or_default().language}</p>
        </>
    }
}
