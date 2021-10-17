pub mod resources;
mod app;
mod control;
mod interface;

pub fn run() {
    let mut app = app::App::new();
    app.run();
}