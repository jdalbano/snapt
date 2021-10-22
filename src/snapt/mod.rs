pub mod control;
pub mod resources;
mod app;
mod interface;

pub fn run() {
    let mut app = app::App::new();
    app.run();
}