use gloo::console::log;
use serde::{Deserialize, Serialize};
use yew::prelude::*;

#[derive(Serialize, Deserialize)]
struct Obj {
    a: String,
    b: String,
}

#[function_component(App)]
pub fn app() -> Html {
    let vals: Obj = Obj {
        a: "awe".to_owned(),
        b: "123".to_owned(),
    };
    log!(serde_json::to_string_pretty(&vals).unwrap());
    html! {
        <div>
            <h1>{"Yolo!"}</h1>
        </div>
    }
}
