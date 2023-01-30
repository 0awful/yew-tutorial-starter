use crate::components::atoms::custom_button::CustomButton;
use crate::components::atoms::text_input::TextInput;
use gloo::console::log;
use stylist::{yew::styled_component, Style};
use yew::prelude::*;

#[styled_component(CustomForm)]
pub fn custom_form() -> Html {
    let styles: Style = Style::new(
        r#"
            * {
                color: black;
            }
        "#,
    )
    .expect("Failed to create style");

    let username_state = use_state(|| "".to_owned());
    let cloned_username_state = username_state.clone();
    let username_changed = Callback::from(move |username| {
        cloned_username_state.set(username);
    });
    html! {
        <>
            <form class={styles}>
                <TextInput name="username" handle_onchange={username_changed} />
                <CustomButton label="submit" />
            </form>
            <p>{"username: "}{&*username_state}</p>
        </>
    }
}
