mod hotkey;
mod listener;
mod loader;
mod profile;

pub fn start_monitoring_keys() {
    let hotkey_profile: profile::Profile = loader::load_hotkey_profile();
    listener::start(hotkey_profile);
}