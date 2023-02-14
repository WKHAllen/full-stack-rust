#![forbid(unsafe_code)]

mod components;
mod state;

use components::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
