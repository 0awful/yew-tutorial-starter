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

    // how to do a bunch of classnames. tried a few ways
    // let title = "str" + " " + "something" doesn't work
    // you end up going down a borrowing rabbit hole I can't comprehend
    // let title = format!("{} {}", "string", "another");
    // works but does expect a fixed size.
    // This appears to be the easiest way to join an arbitrary number of strings
    let title = vec!["value", "another"].join(" ");

    let message = Some("I am message");

    html! {
        <>
            <div class="main">
                <h1 class={title}>{"Yolo!"}</h1>
            </div>

            if let Some(message) = message {
                <p>{message}</p>
            } else {
                <p>{"No messages"}</p>
            }
        </>
    }
}
