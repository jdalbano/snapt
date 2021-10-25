use device_query::{DeviceQuery, DeviceState, Keycode};

use crate::hotkeys::profile::*;
use crate::snapt::app;

const INPUT_TIME_BUFFER_MS: u64 = 180;

pub fn start(hotkey_profile: Profile) {    
    std::thread::spawn(move || {
        let device_state = DeviceState::new();
        
        loop {
            if app::get_do_exit() {
                break;
            }

            monitor_keys(&device_state, &hotkey_profile)
        }
    });
}

fn monitor_keys(device_state: &DeviceState, hotkey_profile: &Profile) {
    if !app::get_do_pause() {
        let keys: Vec<Keycode> = device_state.get_keys();
        let did_process = hotkey_profile.process_incoming_keys(&keys);

        if did_process {
            wait_for_input_time_buffer();
        }
    }
}

fn wait_for_input_time_buffer() {
    let input_time_buffer = std::time::Duration::from_millis(INPUT_TIME_BUFFER_MS);
    std::thread::sleep(input_time_buffer);
}