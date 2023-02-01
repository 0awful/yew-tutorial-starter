use crate::components::atoms::custom_button::CustomButton;
use crate::components::atoms::text_input::TextInput;
use gloo::console::log;
use std::ops::Deref;
use stylist::{yew::styled_component, Style};
use yew::prelude::*;

#[derive(Default, Clone)]
struct State {
    pub username: String,
    pub count: u32,
}

#[styled_component(CustomForm)]
pub fn custom_div() -> Html {
    let styles: Style = Style::new(
        r#"
            * {
                color: black;
            }
        "#,
    )
    .expect("Failed to create style");

    let state = use_state(|| State::default());

    let cloned_state = state.clone();
    let username_changed = Callback::from(move |username| {
        let mut data = cloned_state.deref().clone();
        data.username = username;
        cloned_state.set(data);
    });

    let cloned_state = state.clone();
    let button_clicked = Callback::from(move |_| {
        let mut data = cloned_state.deref().clone();
        data.count += 1;
        cloned_state.set(data)
    });
    html! {
        <>
            <div class={styles}>
                <TextInput name="username" handle_onchange={username_changed} />
                <CustomButton label="submit" onclick={button_clicked}/>
            </div>
            <p>{"username: "}{&state.username}</p>
            <p>{"count: "}{&state.count}</p>
        </>
    }
}
