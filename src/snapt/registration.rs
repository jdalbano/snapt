use std::ffi::{OsStr};
use std::os::windows::ffi::OsStrExt;
use std::ptr::null_mut;

use winapi::um::errhandlingapi;
use winapi::um::synchapi;
use winapi::shared::winerror;

const APP_REGISTRATION: &str = "Local\\$snapt$";

pub fn register_app_instance() -> bool {
    let app_name = OsStr::new(APP_REGISTRATION).encode_wide().chain(Some(0).into_iter()).collect::<Vec<u16>>();

    unsafe {
        let _registration_handle = synchapi::CreateMutexW(null_mut(), false as i32, app_name.as_ptr());

        if errhandlingapi::GetLastError() == winerror::ERROR_ALREADY_EXISTS {
            return false;
        }
    }

    true
}