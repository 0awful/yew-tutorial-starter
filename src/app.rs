use stylist::{yew::styled_component, Style};
use yew::prelude::*;
use yew_router::prelude::*;

const STYLE_FILE: &str = include_str!("app.css");

use crate::router::{switch, Route};

#[styled_component(App)]
pub fn app() -> Html {
    let stylesheet: Style = Style::new(STYLE_FILE).unwrap();

    //let first_load = use_state(|| true);

    //use_effect(move || {
    //// this code will run on
    //  - first render
    //  - all re-renders
    //if *first_load {
    // will only run on first load, if we set first load to false

    //first_load.set(false);
    //}

    //|| {}
    //});

    html! {
        <div class={stylesheet}>
            <BrowserRouter>
                <Switch<Route> render={switch} />
            </BrowserRouter>
        </div>
    }
}
