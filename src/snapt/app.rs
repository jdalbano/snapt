use std::io::Error;

use device_query::{DeviceQuery, DeviceState, Keycode};

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
            self.main_loop(&interface);

            self.end_app_interface(interface);
        }
    }

    fn main_loop(&self, interface: &Interface) {
        self.start_monitoring_keys();

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

    fn start_monitoring_keys(&self) {
        std::thread::spawn(|| {
            let device_state = DeviceState::new();
            let hotkey_profile = &hotkeys::hotkey_loader::load_hotkey_profile();
            
            loop {
                if control::get_do_exit() {
                    break;
                }

                if !control::get_do_pause() {
                    let keys: Vec<Keycode> = device_state.get_keys();
                    let did_process = hotkey_profile.process_incoming_keys(&keys);

                    if did_process {
                        std::thread::sleep_ms(125);
                    }
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

