use std::io::Error;

use device_query::{DeviceQuery, DeviceState, Keycode};

use crate::app::instance as app_instance;
use crate::app::instance::Instance as AppInstance;
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
        let instance_result = self.start_app_instance();

        if let Ok(mut instance) = instance_result {
            self.setup_notification(&mut instance);

            self.main_loop(instance);
        }

        self.close_app();
    }

    fn main_loop(&self, instance: AppInstance) {
        self.start_monitoring_keys();

        loop {
            let was_message_handled = self.handle_instance_message(&instance);

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

    fn start_app_instance(&self) -> Result<AppInstance, Error> {
        unsafe {
            app_instance::create_app_instance()
        }
    }

    fn setup_notification(&self, instance: &mut AppInstance) {
        unsafe {
            app_instance::add_notification(&mut instance.notification);
        }
    }

    fn handle_instance_message(&self, instance: &AppInstance) -> bool {
        unsafe {
            app_instance::handle_message(instance.window)
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

