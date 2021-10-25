use std::clone::Clone;
use std::marker::Copy;
use std::sync::{ Arc, Mutex };

use crate::hotkeys;
use crate::snapt::interface::primary::PrimaryInterface;
use crate::snapt::interface::base::InterfaceBase;
use crate::snapt::registration;

pub const APP_NAME: &str = "Snapt";

#[derive(Clone, Copy)]
pub struct App {
    do_pause: bool,
    do_exit: bool,
}

static mut APP: App = App { do_pause: false, do_exit: false };

pub fn run() {
    let did_register = registration::register_app_instance();

    if did_register {
        hotkeys::start_monitoring_keys();
        
        let mut primary_interface = PrimaryInterface::new();
        primary_interface.run();
    }
}

pub fn get_do_pause() -> bool {
    get_locked_control_value(|app| app.do_pause)
}

pub fn get_do_exit() -> bool {
    get_locked_control_value(|app| app.do_exit)
}

pub fn pause_app() {
    set_locked_control_value(|app| app.do_pause = true);
}

pub fn resume_app() {
    set_locked_control_value(|app| app.do_pause = false);
}

pub fn exit_app() {
    set_locked_control_value(|app| app.do_exit = true);
}

fn get_locked_control_value<T>(func: impl Fn(App) -> T) -> T {
    unsafe {
        let safe_app_wrapper = Arc::new(Mutex::new(APP));
        let app_lock = safe_app_wrapper.lock().expect("app mutex poisoned");
        func(*app_lock)
    }
}

fn set_locked_control_value(func: impl Fn(&mut App)) {
    unsafe {
        let safe_app_wrapper = Arc::new(Mutex::new(&mut APP));
        let mut app_lock = safe_app_wrapper.lock().expect("app mutex poisoned");
        func(*app_lock)
    }
}