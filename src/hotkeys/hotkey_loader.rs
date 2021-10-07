use device_query::Keycode;

use crate::hotkeys::hotkey_profile::*;

pub fn load_hotkey_profile() -> HotkeyProfile {
    load_default_hotkey_profile()
}

fn load_default_hotkey_profile() -> HotkeyProfile {
    HotkeyProfile::new(
        get_left_hotkey_default(),
        get_right_hotkey_default(),
        get_top_hotkey_default(),
        get_bottom_hotkey_default(),
        get_full_hotkey_default(),
    )
}

fn get_left_hotkey_default() -> Vec<Keycode> {
    vec!(Keycode::LControl, Keycode::LAlt, Keycode::Left)
}

fn get_right_hotkey_default() -> Vec<Keycode> {
    vec!(Keycode::LControl, Keycode::LAlt, Keycode::Right)
}

fn get_top_hotkey_default() -> Vec<Keycode> {
    vec!(Keycode::LControl, Keycode::LAlt, Keycode::Up)
}

fn get_bottom_hotkey_default() -> Vec<Keycode> {
    vec!(Keycode::LControl, Keycode::LAlt, Keycode::Down)
}

fn get_full_hotkey_default() -> Vec<Keycode> {
    vec!(Keycode::LControl, Keycode::LAlt, Keycode::Space)
}