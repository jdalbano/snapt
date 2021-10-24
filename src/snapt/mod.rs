pub mod control;
pub mod resources;
mod app;
mod interface;
mod registration;

pub fn run() {
    let mut app = app::App::new();
    app.run();
}