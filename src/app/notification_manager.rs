use std::ffi::{OsStr};
use std::os::windows::ffi::OsStrExt;
use std::mem::{size_of}; //get size of stuff and init with zeros

use winapi;

type NOTIFYICONDATAW = winapi::um::shellapi::NOTIFYICONDATAW;

pub unsafe fn add_notification() {
    let mut nid = get_notification_icon();
    winapi::um::shellapi::Shell_NotifyIconW(winapi::um::shellapi::NIM_ADD, &mut nid); 
    
    winapi::um::shellapi::Shell_NotifyIconW(winapi::um::shellapi::NIM_DELETE, &mut nid); 
}

pub unsafe fn remove_notification() {
    let mut nid = get_notification_icon();
    winapi::um::shellapi::Shell_NotifyIconW(winapi::um::shellapi::NIM_DELETE, &mut nid); 
}

unsafe fn get_notification_icon() -> NOTIFYICONDATAW {
    let hwnd = winapi::um::wincon::GetConsoleWindow();

    let wm_mymessage = winapi::um::winuser::WM_APP + 100;

    let notification_tooltip_str: &str = "Snapt";
    let tooltip_os = OsStr::new(notification_tooltip_str); 
    let tooltip_utf16 = tooltip_os.encode_wide().collect::<Vec<u16>>();

    let mut tooltip_sz: [u16; 128] = [0; 128];
    tooltip_sz[..tooltip_utf16.len()].copy_from_slice(&tooltip_utf16); 
    
    const IMAGE_RESOURCE: &str = "main_icon";
    let ico_resource: Vec<u16> = OsStr::new(IMAGE_RESOURCE).encode_wide(). chain(Some(0).into_iter()).collect();
    let module_handle = winapi::um::libloaderapi::GetModuleHandleA(0 as *const i8);

    NOTIFYICONDATAW {
        cbSize: size_of::<winapi::um::shellapi::NOTIFYICONDATAW>() as u32,
        hWnd: hwnd,
        uID: 1001,
        uCallbackMessage: wm_mymessage,
        hIcon: winapi::um::winuser::LoadIconW(module_handle, ico_resource.as_ptr()),
        szTip: tooltip_sz,
        uFlags: winapi::um::shellapi::NIF_MESSAGE | winapi::um::shellapi::NIF_ICON | winapi::um::shellapi::NIF_TIP,
        ..Default::default()
    }
}
