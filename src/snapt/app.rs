use std::io::Error;

use crate::snapt::control;
use crate::snapt::interface;
use crate::snapt::interface::Interface;
use crate::hotkeys;

pub const APP_NAME: &str = "Snapt";

pub struct App { }

impl App {
    pub fn new() -> App {
        App { }
    }

    pub fn run(&mut self) {
        let interface_result = self.start_app_interface();

        if let Ok(interface) = interface_result {
            hotkeys::start_monitoring_keys();

            self.main_loop(&interface);

            self.end_app_interface(interface);
        }
    }

    fn main_loop(&self, interface: &Interface) {
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

