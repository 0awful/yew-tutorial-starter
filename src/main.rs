use yew::prelude::*;

fn main() {
    yew::Renderer::<App>::new().render();
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <div>
            <h1>{"Yolo"}</h1>
        </div>
    }
}
