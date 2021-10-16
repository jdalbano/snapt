use std::ffi::{OsStr};
use std::os::windows::ffi::OsStrExt;
use std::mem::{size_of};

use winapi::um::*;

const NOTIFICATION_ID: u32 = 3434773434;

pub unsafe fn add_notification() {
    let mut nid = get_notification_icon();
    shellapi::Shell_NotifyIconW(shellapi::NIM_ADD, &mut nid); 
    
    shellapi::Shell_NotifyIconW(shellapi::NIM_DELETE, &mut nid); 
}

pub unsafe fn remove_notification() {
    let mut nid = get_notification_icon();
    shellapi::Shell_NotifyIconW(shellapi::NIM_DELETE, &mut nid); 
}

unsafe fn get_notification_icon() -> shellapi::NOTIFYICONDATAW {
    let hwnd = wincon::GetConsoleWindow();

    let wm_mymessage = winuser::WM_APP + 100;

    let tooltip_os = OsStr::new("Snapt"); 
    let tooltip_utf16 = tooltip_os.encode_wide().collect::<Vec<u16>>();

    let mut tooltip_sz: [u16; 128] = [0; 128];
    tooltip_sz[..tooltip_utf16.len()].copy_from_slice(&tooltip_utf16); 
    
    let ico_resource: Vec<u16> = OsStr::new("main_icon").encode_wide().chain(Some(0).into_iter()).collect();
    let module_handle = libloaderapi::GetModuleHandleA(0 as *const i8);

    shellapi::NOTIFYICONDATAW {
        cbSize: size_of::<shellapi::NOTIFYICONDATAW>() as u32,
        hWnd: hwnd,
        uID: NOTIFICATION_ID,
        uCallbackMessage: wm_mymessage,
        hIcon: winuser::LoadIconW(module_handle, ico_resource.as_ptr()),
        szTip: tooltip_sz,
        uFlags: shellapi::NIF_MESSAGE | shellapi::NIF_ICON | shellapi::NIF_TIP,
        ..Default::default()
    }
}
