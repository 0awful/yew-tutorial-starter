use stylist::{style, yew::styled_component};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub title: String,
    pub status: Status,
}

#[derive(PartialEq)]
pub enum Status {
    Normal,
    Ok,
    Error,
}

impl Status {
    pub fn to_string(&self) -> String {
        match self {
            Status::Normal => "normal".to_owned(),
            Status::Ok => "ok".to_owned(),
            Status::Error => "error".to_owned(),
        }
    }
}

#[styled_component(MainTitle)]
pub fn main_title(props: &Props) -> Html {
    let stylesheet = style!(
        r#"
            .normal {
                color: white;
            }

            .ok {
                color: green;
            }

            .error {
                color: red;
            }
        "#
    )
    .unwrap();
    html! {
        <div class={stylesheet}>
            <h1 class={&props.status.to_string()}>{&props.title}</h1>
        </div>
    }
}
