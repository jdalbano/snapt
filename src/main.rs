#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app;
mod hotkeys;
mod window;

fn main() {
    app::run();
}