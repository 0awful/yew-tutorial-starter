use crate::app::App;

mod app;

pub mod components;

fn main() {
    yew::Renderer::<App>::new().render();
}
