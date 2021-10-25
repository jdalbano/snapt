use crate::hotkeys;
use crate::snapt::interface::primary::PrimaryInterface;
use crate::snapt::interface::base::InterfaceBase;
use crate::snapt::registration;

pub const APP_NAME: &str = "Snapt";

pub struct App { }

impl App {
    pub fn new() -> App {
        App { }
    }

    pub fn run(&mut self) {
        let did_register = registration::register_app_instance();

        if did_register {
            hotkeys::start_monitoring_keys();
            
            let mut primary_interface = PrimaryInterface::new();
            primary_interface.run();
        }
    }
}

