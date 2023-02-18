use std::ops::Deref;
use stylist::{yew::styled_component, Style};
use yew::{prelude::*, ContextProvider};

const STYLE_FILE: &str = include_str!("app.css");

use crate::components::molecules::custom_form::{CustomForm, State};

#[derive(Debug, PartialEq, Clone, Default)]
pub struct Context {
    pub username: String,
    pub language: String,
}

#[styled_component(App)]
pub fn app() -> Html {
    let stylesheet: Style = Style::new(STYLE_FILE).unwrap();

    let state = use_state(Context::default);

    let custom_form_submit = {
        let from_state = state.clone();
        Callback::from(move |data: State| {
            let mut state_update = from_state.deref().clone();
            state_update.username = data.username;
            state_update.language = data.language;
            from_state.set(state_update);
        })
    };

    html! {
        <div class={stylesheet}>
            <ContextProvider<Context> context={state.deref().clone()}>
                <CustomForm on_submit={custom_form_submit}/>
            </ContextProvider<Context>>
        </div>
    }
}
