use std::clone::Clone;
use std::marker::Copy;
use std::sync::{ Arc, Mutex };

#[derive(Clone, Copy)]
struct Control {
    do_pause: bool,
    do_exit: bool,
}

static mut CONTROL: Control = Control { do_pause: false, do_exit: false };

pub fn get_do_pause() -> bool {
    get_locked_control_value(|control| control.do_pause)
}

pub fn get_do_exit() -> bool {
    get_locked_control_value(|control| control.do_exit)
}

pub fn pause_app() {
    set_locked_control_value(|control| control.do_pause = true);
}

pub fn resume_app() {
    set_locked_control_value(|control| control.do_pause = false);
}

pub fn exit_app() {
    set_locked_control_value(|control| control.do_exit = true);
}

fn get_locked_control_value<T>(func: impl Fn(Control) -> T) -> T {
    unsafe {
        let safe_control_wrapper = Arc::new(Mutex::new(CONTROL));
        let control_lock = safe_control_wrapper.lock().expect("control mutex poisoned");
        func(*control_lock)
    }
}

fn set_locked_control_value(func: impl Fn(&mut Control)) {
    unsafe {
        let safe_control_wrapper = Arc::new(Mutex::new(&mut CONTROL));
        let mut control_lock = safe_control_wrapper.lock().expect("control mutex poisoned");
        func(*control_lock)
    }
}