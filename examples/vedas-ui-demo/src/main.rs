#![recursion_limit = "4096"]
mod app;
mod page;
pub use app::App;

fn main() {
    yew::start_app::<App>();
}
