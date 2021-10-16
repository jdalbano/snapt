pub mod resources;
mod interface;
mod snapt;

pub fn run() {
    let mut app = snapt::Snapt::new();
    app.run();
}