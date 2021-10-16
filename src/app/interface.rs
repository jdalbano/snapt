use std::ffi::{OsStr};
use std::io::Error;
use std::mem;
use std::os::windows::ffi::OsStrExt;
use std::ptr::null_mut;

use winapi::um::*;
use winapi::shared::minwindef;
use winapi::shared::windef;

use crate::app::snapt;

const CLASS_NAME: &str = "window";
const NOTIFICATION_ID: u32 = 3434773434;
const NOTIFICATION_CALLBACK: u32 = winuser::WM_APP + 1;

pub struct Interface {
    pub window: windef::HWND,
    pub notification: shellapi::NOTIFYICONDATAW,
}

pub unsafe fn create_app_interface() -> Result<Interface, Error> {
    let class_name = OsStr::new(CLASS_NAME).encode_wide().chain(Some(0).into_iter()).collect::<Vec<u16>>();
    let app_name = OsStr::new(snapt::APP_NAME).encode_wide().chain(Some(0).into_iter()).collect::<Vec<u16>>();

    let module = libloaderapi::GetModuleHandleW(null_mut());    
    let wnd_class = get_wnd_class(&class_name, module);

    winuser::RegisterClassW(&wnd_class);

    let window = get_window_handle(&class_name, &app_name, module);
    let notification = get_notification(app_name, window, module);

    if window.is_null() {
        Err( Error::last_os_error() )
    } else {
        Ok( Interface { window, notification } )
    }
}

pub unsafe fn add_notification(notification: &mut shellapi::NOTIFYICONDATAW) {
    shellapi::Shell_NotifyIconW(shellapi::NIM_ADD, notification);
    shellapi::Shell_NotifyIconW(shellapi::NIM_SETVERSION, notification);
}

pub unsafe fn remove_notification(notification: &mut shellapi::NOTIFYICONDATAW) {
    shellapi::Shell_NotifyIconW(shellapi::NIM_DELETE, notification); 
}

pub unsafe fn handle_message(window: windef::HWND) -> bool {
    let mut message: mem::MaybeUninit<winuser::MSG> = mem::MaybeUninit::uninit();

    if winuser::GetMessageW(message.as_mut_ptr() as *mut winuser::MSG, window, 0, 0 ) > 0 {
        winuser::TranslateMessage(message.as_ptr() as *const winuser::MSG);    
        winuser::DispatchMessageW(message.as_ptr() as *const winuser::MSG);
        true
    } else {
        false
    }
}

unsafe extern "system" fn wnd_proc(hwnd: windef::HWND, msg: u32, wparam: minwindef::WPARAM, lparam: minwindef::LPARAM) -> minwindef::LRESULT {
    match msg {
        NOTIFICATION_CALLBACK => {
            match minwindef::LOWORD(lparam as u32) as u32 {
                winuser::WM_CONTEXTMENU => handle_wnd_proc_notification_context_menu(hwnd, msg, wparam, lparam),
                _ => handle_wnd_proc_default(hwnd, msg, wparam, lparam)
            }
        }
        _ => handle_wnd_proc_default(hwnd, msg, wparam, lparam)
    }
}

unsafe fn handle_wnd_proc_notification_context_menu(hwnd: windef::HWND, msg: u32, wparam: minwindef::WPARAM, lparam: minwindef::LPARAM) -> minwindef::LRESULT  {
    print!("right click on notification!!!");
    0
}

unsafe fn handle_wnd_proc_default(hwnd: windef::HWND, msg: u32, wparam: minwindef::WPARAM, lparam: minwindef::LPARAM) -> minwindef::LRESULT {
    let result = winuser::DefWindowProcW(hwnd, msg, wparam, lparam);
    result
}

unsafe fn get_wnd_class(class_name: &Vec<u16>, module: minwindef::HMODULE) -> winuser::WNDCLASSW {
    winuser::WNDCLASSW {
        style : winuser::CS_OWNDC | winuser::CS_HREDRAW | winuser::CS_VREDRAW,
        lpfnWndProc : Some(wnd_proc),
        hInstance : module,
        lpszClassName : class_name.as_ptr(),
        cbClsExtra : 0,	
        cbWndExtra : 0,
        hIcon: null_mut(),
        hCursor: null_mut(),
        hbrBackground: null_mut(),
        lpszMenuName: null_mut(),
    }
}

unsafe fn get_window_handle(class_name: &Vec<u16>, app_name: &Vec<u16>, module: minwindef::HMODULE) -> windef::HWND {
    winuser::CreateWindowExW(
        0,
        class_name.as_ptr(),
        app_name.as_ptr(),
        winuser::WS_BORDER,
        winuser::CW_USEDEFAULT,
        winuser::CW_USEDEFAULT,
        winuser::CW_USEDEFAULT,
        winuser::CW_USEDEFAULT,
        null_mut(),
        null_mut(),
        module,
        null_mut())
}

unsafe fn get_notification(app_name: Vec<u16>, window: windef::HWND, module: minwindef::HMODULE) -> shellapi::NOTIFYICONDATAW {
    let mut tooltip_sz: [u16; 128] = [0; 128];
    tooltip_sz[..app_name.len()].copy_from_slice(&app_name); 
    
    let ico_resource: Vec<u16> = OsStr::new("main_icon").encode_wide().chain(Some(0).into_iter()).collect();

    let mut notification = shellapi::NOTIFYICONDATAW {
        cbSize: mem::size_of::<shellapi::NOTIFYICONDATAW>() as u32,
        hWnd: window,
        uID: NOTIFICATION_ID,
        uCallbackMessage: NOTIFICATION_CALLBACK,
        hIcon: winuser::LoadIconW(module, ico_resource.as_ptr()),
        szTip: tooltip_sz,
        uFlags: shellapi::NIF_MESSAGE | shellapi::NIF_ICON | shellapi::NIF_TIP,
        ..Default::default()
    };

    let u_version = notification.u.uVersion_mut();
    *u_version = shellapi::NOTIFYICON_VERSION_4;

    notification
}