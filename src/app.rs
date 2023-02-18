use gloo::console::log;
use stylist::{yew::styled_component, Style};
use yew::prelude::*;

const STYLE_FILE: &str = include_str!("app.css");

use crate::components::molecules::custom_form::{CustomForm, State};

#[styled_component(App)]
pub fn app() -> Html {
    let stylesheet: Style = Style::new(STYLE_FILE).unwrap();

    let custom_form_submit = Callback::from(|state: State| {
        log!("username is", state.username);
        log!("lang is", state.language)
    });

    html! {
        <div class={stylesheet}>
            <CustomForm on_submit={custom_form_submit}/>
        </div>
    }
}
