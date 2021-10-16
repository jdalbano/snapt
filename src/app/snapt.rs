use std::io::Error;

use device_query::{DeviceQuery, DeviceState, Keycode};

use crate::app::instance as app_instance;
use crate::app::instance::Instance as AppInstance;
use crate::hotkeys;

pub const APP_NAME: &str = "Snapt";

pub struct Snapt {
    do_pause: bool,
    do_exit: bool,
}

impl Snapt {
    pub fn new() -> Snapt {
        Snapt { 
            do_pause: false, 
            do_exit: false,
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
        let device_state = DeviceState::new();
    
        let hotkey_profile = &hotkeys::hotkey_loader::load_hotkey_profile();
        

        loop {
            print!("{:?}\n\n", instance.window);
            let was_message_handled = self.handle_instance_message(&instance);
            print!("\n{} message was handled!!!!! \n\n", was_message_handled);

            if !was_message_handled {
                break;
            }

            if !self.do_pause {
                let keys: Vec<Keycode> = device_state.get_keys();

                print!("{:?}\n\n", keys);

                let _did_process = hotkey_profile.process_incoming_keys(&keys);
            }

            if self.do_exit {
                break;
            }
        }
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

    fn exit_main_loop(&mut self) {
        self.do_exit = true;
    }
}

