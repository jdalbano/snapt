use std::io::Error;

use device_query::{DeviceQuery, DeviceState, Keycode};

use crate::app::interface as app_interface;
use crate::app::interface::Interface as AppInterface;
use crate::hotkeys;

pub const APP_NAME: &str = "Snapt";

pub struct Snapt {
    do_pause: bool,
}

impl Snapt {
    pub fn new() -> Snapt {
        Snapt { 
            do_pause: false, 
        }
    }

    pub fn run(&self) {
        let interface_result = self.start_app_interface();

        if let Ok(mut interface) = interface_result {
            self.setup_notification(&mut interface);

            self.main_loop(interface);
        }

        self.close_app();
    }

    fn main_loop(&self, interface: AppInterface) {
        self.start_monitoring_keys();

        loop {
            let was_message_handled = self.handle_interface_message(&interface);

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
                if !(&do_pause) {
                    let keys: Vec<Keycode> = device_state.get_keys();
                    let _did_process = hotkey_profile.process_incoming_keys(&keys);   
                } 
            }
        });
    }

    fn start_app_interface(&self) -> Result<AppInterface, Error> {
        unsafe {
            app_interface::create_app_interface()
        }
    }

    fn setup_notification(&self, instance: &mut AppInterface) {
        unsafe {
            app_interface::add_notification(&mut instance.notification);
        }
    }

    fn handle_interface_message(&self, instance: &AppInterface) -> bool {
        unsafe {
            app_interface::handle_message(instance.window)
        }
    }

    fn close_app(&self) {
        unsafe {
            
        }
    }

    fn pause_main_loop(&mut self) {
        self.do_pause = true;
    }

    fn resume_main_loop(&mut self) {
        self.do_pause = false;
    }
}

