use device_query::Keycode;

use crate::hotkeys::hotkey::*;
use crate::window;

enum KeyResult<'a> {
    Some(&'a Hotkey),
    None,
}

pub struct Profile {
    left_key: Hotkey,
    right_key: Hotkey,
    top_key: Hotkey,
    bottom_key: Hotkey,
    full_key: Hotkey,
}

impl Profile {
    pub fn new(left_keys: Vec<Keycode>, right_keys: Vec<Keycode>, top_keys: Vec<Keycode>, bottom_keys: Vec<Keycode>, full_keys: Vec<Keycode>) -> Self {
        Profile {
            left_key: Hotkey::new(window::dock_left, left_keys),
            right_key: Hotkey::new(window::dock_right, right_keys),
            top_key: Hotkey::new(window::dock_top, top_keys),
            bottom_key: Hotkey::new(window::dock_bottom, bottom_keys),
            full_key: Hotkey::new(window::dock_full, full_keys),
        }
    }

    pub fn process_incoming_keys(&self, incoming_keys: &Vec<Keycode>) -> bool {
        let matching_key = self.get_matching_key_from_incoming_keys(incoming_keys);

        if let KeyResult::Some(key) = matching_key {
            key.execute_callback();
            true
        } else {
            false
        }
    }

    fn get_matching_key_from_incoming_keys(&self, incoming_keys: &Vec<Keycode>) -> KeyResult {
        let keys = self.get_all_keys();
        let matching_key = keys.iter().find(|x| x.check_if_keys_match(incoming_keys));

        if let Option::Some(key) = matching_key {
            KeyResult::Some(key)
        } else {
            KeyResult::None
        }
    }

    fn get_all_keys(&self) -> Vec<&Hotkey> {
        vec!(&self.left_key, &self.right_key, &self.top_key, &self.bottom_key, &self.full_key)
    }
}
