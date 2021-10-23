use device_query::Keycode;

type Callback = fn();

pub struct Hotkey {
    callback: Callback,
    key_combination: Vec<Keycode>,
}

impl Hotkey {
    pub fn new(callback: Callback, key_combination: Vec<Keycode>) -> Self {
        Hotkey{
            callback: callback,
            key_combination: key_combination,
        }
    }

    pub fn execute_callback(&self) {
        (self.callback)();
    }

    pub fn check_if_keys_match(&self, keys: &Vec<Keycode>) -> bool {
        if keys.len() != self.key_combination.len() {
            false
        } else {
            self.key_combination.iter().all(|key| keys.contains(key))
        }
    }
}