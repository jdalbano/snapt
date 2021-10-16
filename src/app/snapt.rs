use device_query::{DeviceQuery, DeviceState, Keycode};

use crate::window::notification_manager;
use crate::hotkeys;

pub struct Snapt {
    do_pause: bool,
    do_exit: bool,
}

impl Snapt {
    pub fn new() -> Snapt {
        Snapt { do_pause: false, do_exit: false }
    }

    pub fn run(&self) {
        self.start_app();
        self.main_loop();
        self.close_app();
    }

    fn main_loop(&self) {
        let device_state = DeviceState::new();
    
        let hotkey_profile = &hotkeys::hotkey_loader::load_hotkey_profile();
    
        loop {
            if !self.do_pause {
                let keys: Vec<Keycode> = device_state.get_keys();
                let _did_process = hotkey_profile.process_incoming_keys(&keys);
            }

            if self.do_exit {
                break;
            }
        }
    }

    fn start_app(&self) {
        unsafe {
            notification_manager::add_notification();
        }
    }

    fn close_app(&self) {
        unsafe {
            notification_manager::remove_notification();
        }
    }

    fn pause_main_loop(&mut self) {
        self.do_pause = true;
    }

    fn resume_main_loop(&mut self) {
        self.do_pause = false;
    }

    fn exit_main_loop(&mut self) {
        self.do_exit = true;
    }
}

