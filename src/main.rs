mod app;
mod functions;
mod sections;

use app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
