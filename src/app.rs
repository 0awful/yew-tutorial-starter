use gloo::console::log;
use serde::{Deserialize, Serialize};
use stylist::{style, yew::styled_component};
use yew::prelude::*;

#[derive(Serialize, Deserialize)]
struct Obj {
    a: String,
    b: String,
}

#[styled_component(App)]
pub fn app() -> Html {
    let vals: Obj = Obj {
        a: "awe".to_owned(),
        b: "123".to_owned(),
    };
    // note the lack of a background color here.
    // as the body is not impacted by these styles, and can't be no matter how hard we try,
    // we keep the background-color in index.scss (technically we moved it from index.html, but that was all me not the tutorial)
    let stylesheet = style!(
        r#"
            color: white;
        "#
    )
    .unwrap();
    // tradeoffs here. one, we have more difficulty reusing styles
    // two, we can prevent all accidental reuse
    // if we want to reuse we handle reuse like idiomatic rust

    log!(serde_json::to_string_pretty(&vals).unwrap());
    let messages: Vec<&str> = vec!["some message", "another", "and one more"];

    html! {
        <html class={stylesheet}>
            <div>
                <h1>{"Yolo!"}</h1>
            </div>

            if messages.len() > 0 {
                <ul>
                {list_to_html(messages)}
                </ul>
            } else {
                <p class={css!("color: red; font-size: 75px;")}>{"No messages"}</p>
                // under the hood this makes a new style in the head
                // as does all other css declarations from stylist
            }
        </html>
    }
}

fn list_to_html(list: Vec<&str>) -> Vec<Html> {
    list.iter().map(|item| html! {<li>{item}</li>}).collect() // No semi means implicit return
}
