use crate::app::App;

mod app;

fn main() {
    yew::Renderer::<App>::new().render();
}
