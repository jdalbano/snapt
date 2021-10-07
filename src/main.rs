use device_query::{DeviceQuery, DeviceState, Keycode};

mod hotkey_profile;
mod hotkey_loader;
mod hotkey;
mod window_manager;
mod window_transform;

fn main() {
    let device_state = DeviceState::new();

    let hotkey_profile = &hotkey_loader::load_hotkey_profile();

    loop {
        let keys: Vec<Keycode> = device_state.get_keys();
        let _did_process = hotkey_profile.process_incoming_keys(&keys);
    }
}