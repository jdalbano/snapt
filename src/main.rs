use device_query::{DeviceQuery, DeviceState, Keycode};

pub mod hotkeys;
pub mod window;

fn main() {
    let device_state = DeviceState::new();

    let hotkey_profile = &hotkeys::hotkey_loader::load_hotkey_profile();

    loop {
        let keys: Vec<Keycode> = device_state.get_keys();
        let _did_process = hotkey_profile.process_incoming_keys(&keys);
    }
}