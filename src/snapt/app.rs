use std::io::Error;

use crate::snapt::control;
use crate::snapt::interface::base as base_interface;
use crate::snapt::registration;
use crate::hotkeys;

pub const APP_NAME: &str = "Snapt";

pub struct App { }

impl App {
    pub fn new() -> App {
        App { }
    }

    pub fn run(&mut self) {
        let did_register = registration::register_app_instance();

        if did_register {
            let interface_result = self.start_app_interface();
    
            if let Ok(interface) = interface_result {
                hotkeys::start_monitoring_keys();
    
                self.main_loop(&interface);
    
                self.end_app_interface(interface);
            }
        }
    }

    fn main_loop(&self, interface: &base_interface::Interface) {
        loop {
            if control::get_do_exit() {
                break;
            }

            let was_message_handled = self.handle_interface_messages(&interface);

            if !was_message_handled {
                break;
            }
        }
    }

    fn start_app_interface(&mut self) -> Result<base_interface::Interface, Error> {
        unsafe {
            base_interface::create_app_interface(self as *mut App)
        }
    }    
    
    fn end_app_interface(&self, interface: base_interface::Interface) {
        unsafe {
            base_interface::destroy_app_interface(interface);
        }
    }

    fn handle_interface_messages(&self, instance: &base_interface::Interface) -> bool {
        unsafe {
            base_interface::handle_messages(instance.window)
        }
    }
}

