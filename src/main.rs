use crate::app::App;

mod app;

pub mod components;
pub mod router;

fn main() {
    yew::Renderer::<App>::new().render();
}
