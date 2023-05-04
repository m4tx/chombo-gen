mod app;
mod hand;
mod hand_example;
mod hand_generator;
mod input;
mod select;

use app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
