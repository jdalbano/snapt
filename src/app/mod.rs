mod snapt;
mod notification_manager;

pub fn run() {
    let app = snapt::Snapt::new();
    app.run();
}