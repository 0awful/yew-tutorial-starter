use crate::components::atoms::custom_button::CustomButton;
use crate::components::atoms::text_input::TextInput;
use gloo::console::log;
use stylist::{yew::styled_component, Style};
use yew::prelude::*;

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

    let button_count_state = use_state(|| 0_u32);

    let username_state = use_state(|| "".to_owned());
    let cloned_username_state = username_state.clone();
    let username_changed = Callback::from(move |username| {
        cloned_username_state.set(username);
    });
    let cloned_button_count_state = button_count_state.clone();
    let button_clicked = Callback::from(move |_| {
        let count: u32 = *cloned_button_count_state;
        cloned_button_count_state.set(count + 1);
    });
    html! {
        <>
            <div class={styles}>
                <TextInput name="username" handle_onchange={username_changed} />
                <CustomButton label="submit" onclick={button_clicked}/>
            </div>
            <p>{"username: "}{&*username_state}</p>
            <p>{"count: "}{&*button_count_state}</p>
        </>
    }
}
