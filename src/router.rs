use yew::{prelude::*, ContextProvider};
use yew_router::prelude::*;

use crate::components::pages::home::Home;

#[derive(Routable, Clone, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/about")]
    About,
}

pub fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <Home/> },
        Route::About => html! { <h1>{"hey"}</h1> },
    }
}
