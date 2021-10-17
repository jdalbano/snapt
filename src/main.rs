#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod hotkeys;
mod snapt;
mod window;

fn main() {
    snapt::run();
}