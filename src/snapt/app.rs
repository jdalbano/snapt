use std::io::Error;

use device_query::{DeviceQuery, DeviceState, Keycode};

use crate::snapt::interface;
use crate::snapt::interface::Interface;
use crate::hotkeys;

pub const APP_NAME: &str = "Snapt";

pub struct App {
    pub do_pause: bool,
    do_exit: bool,
}

impl App {
    pub fn new() -> App {
        App { 
            do_pause: false,
            do_exit: false,
        }
    }

    pub fn run(&mut self) {
        let interface_result = self.start_app_interface();

        if let Ok(interface) = interface_result {
            self.main_loop(&interface);

            self.end_app_interface(interface);
        }
    }

    pub fn pause_app(&mut self) {
        self.do_pause = true;
    }

    pub fn resume_app(&mut self) {
        self.do_pause = false;
    }

    pub fn exit_app(&mut self) {
        self.do_exit = true;
    }

    fn main_loop(&self, interface: &Interface) {
        self.start_monitoring_keys();

        loop {
            if self.do_exit {
                break;
            }

            let was_message_handled = self.handle_interface_messages(&interface);

            if !was_message_handled {
                break;
            }
        }
    }

    fn start_monitoring_keys(&self) {
        std::thread::spawn(|| {
            let do_pause = false;
            let device_state = DeviceState::new();
            let hotkey_profile = &hotkeys::hotkey_loader::load_hotkey_profile();
            
            loop {
                if !do_pause {
                    let keys: Vec<Keycode> = device_state.get_keys();
                    let _did_process = hotkey_profile.process_incoming_keys(&keys);   
                }
            }
        });
    }

    fn start_app_interface(&mut self) -> Result<Interface, Error> {
        unsafe {
            interface::create_app_interface(self as *mut App)
        }
    }    
    
    fn end_app_interface(&self, interface: Interface) {
        unsafe {
            interface::destroy_app_interface(interface);
        }
    }

    fn handle_interface_messages(&self, instance: &Interface) -> bool {
        unsafe {
            interface::handle_messages(instance.window)
        }
    }
}

