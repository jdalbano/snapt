use std::ffi::{OsStr};
use std::io::Error;
use std::mem;
use std::os::windows::ffi::OsStrExt;
use std::ptr::null_mut;

use winapi::um::*;
use winapi::shared::minwindef;
use winapi::shared::windef;

use crate::snapt::app;
use crate::snapt::app::App;
use crate::snapt::control;
use crate::snapt::resources;

const CLASS_NAME: &str = "interface";
const NOTIFICATION_ID: u32 = 3434773434;
const NOTIFICATION_CALLBACK: u32 = winuser::WM_APP + 1;
const PAUSED_TEXT: &str = " (Paused)";
const WM_COMMAND: u32 = winuser::WM_COMMAND as u32;

pub struct Interface {
    pub window: windef::HWND,
    pub notification: shellapi::NOTIFYICONDATAW,
}

pub unsafe fn create_app_interface(app_instance: *mut App) -> Result<Interface, Error> {
    let class_name = OsStr::new(CLASS_NAME).encode_wide().chain(Some(0).into_iter()).collect::<Vec<u16>>();

    let module = libloaderapi::GetModuleHandleW(null_mut());    
    let wnd_class = create_wnd_class(&class_name, module);

    winuser::RegisterClassW(&wnd_class);

    let window = create_window_handle(&class_name, module);
    bind_app_instance_to_window(app_instance, window);

    let mut notification = create_notification( window, module);
    add_notification(&mut notification);

    if window.is_null() {
        Err(Error::last_os_error())
    } else {
        Ok(Interface { window, notification })
    }
}

pub unsafe fn destroy_app_interface(mut app_interface: Interface) {
    remove_notification(&mut app_interface.notification);
    winuser::PostMessageW(app_interface.window, winuser::WM_CLOSE, 0, 0);
}

unsafe fn add_notification(notification: &mut shellapi::NOTIFYICONDATAW) {
    shellapi::Shell_NotifyIconW(shellapi::NIM_ADD, notification);
    shellapi::Shell_NotifyIconW(shellapi::NIM_SETVERSION, notification);
}

unsafe fn remove_notification(notification: &mut shellapi::NOTIFYICONDATAW) {
    shellapi::Shell_NotifyIconW(shellapi::NIM_DELETE, notification); 
}

unsafe fn modify_notification(hwnd: windef::HWND) {
    let module = winuser::GetWindowLongPtrW(hwnd, winuser::GWLP_HINSTANCE);
    let mut notification = create_notification(hwnd, module as minwindef::HMODULE);
    shellapi::Shell_NotifyIconW(shellapi::NIM_MODIFY, &mut notification);
}

pub unsafe fn handle_messages(window: windef::HWND) -> bool {
    let mut message: mem::MaybeUninit<winuser::MSG> = mem::MaybeUninit::uninit();

    if winuser::GetMessageW(message.as_mut_ptr() as *mut winuser::MSG, window, 0, 0 ) > 0 {
        winuser::TranslateMessage(message.as_ptr() as *const winuser::MSG);    
        winuser::DispatchMessageW(message.as_ptr() as *const winuser::MSG);
        true
    } else {
        false
    }
}

unsafe fn create_wnd_class(class_name: &Vec<u16>, module: minwindef::HMODULE) -> winuser::WNDCLASSW {
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

unsafe fn create_window_handle(class_name: &Vec<u16>, module: minwindef::HMODULE) -> windef::HWND {
    let app_name = OsStr::new(app::APP_NAME).encode_wide().chain(Some(0).into_iter()).collect::<Vec<u16>>();

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

unsafe fn bind_app_instance_to_window(app_instance: *mut App, window: windef::HWND) {
    winuser::SetWindowLongPtrW(window, winuser::GWLP_USERDATA, (app_instance) as isize);
}

unsafe fn create_notification(window: windef::HWND, module: minwindef::HMODULE) -> shellapi::NOTIFYICONDATAW {
    let tooltip = app::APP_NAME.to_owned() + if control::get_do_pause() { PAUSED_TEXT } else { "" };
    let tooltip_vec = OsStr::new(&tooltip).encode_wide().chain(Some(0).into_iter()).collect::<Vec<u16>>();
    let mut tooltip_sz: [u16; 128] = [0; 128];
    tooltip_sz[..tooltip_vec.len()].copy_from_slice(&tooltip_vec); 
    
    let ico_resource: Vec<u16> = OsStr::new(resources::MAIN_ICON).encode_wide().chain(Some(0).into_iter()).collect();

    let mut notification = shellapi::NOTIFYICONDATAW {
        cbSize: mem::size_of::<shellapi::NOTIFYICONDATAW>() as u32,
        hWnd: window,
        uID: NOTIFICATION_ID,
        uCallbackMessage: NOTIFICATION_CALLBACK,
        hIcon: winuser::LoadIconW(module, ico_resource.as_ptr()),
        szTip: tooltip_sz,
        uFlags: shellapi::NIF_MESSAGE | shellapi::NIF_ICON | shellapi::NIF_TIP | shellapi::NIF_SHOWTIP,
        ..Default::default()
    };

    let u_version = notification.u.uVersion_mut();
    *u_version = shellapi::NOTIFYICON_VERSION_4;

    notification
}

unsafe extern "system" fn wnd_proc(hwnd: windef::HWND, msg: u32, wparam: minwindef::WPARAM, lparam: minwindef::LPARAM) -> minwindef::LRESULT {
    match msg {
        NOTIFICATION_CALLBACK => handle_wnd_proc_notification_callback(hwnd, msg, wparam, lparam),
        WM_COMMAND => handle_wnd_proc_wm_command(hwnd, msg, wparam, lparam),
        _ => handle_wnd_proc_default(hwnd, msg, wparam, lparam)
    }
}

unsafe fn handle_wnd_proc_notification_callback(hwnd: windef::HWND, msg: u32, wparam: minwindef::WPARAM, lparam: minwindef::LPARAM) -> minwindef::LRESULT {
    match minwindef::LOWORD(lparam as u32) as u32 {
        winuser::WM_CONTEXTMENU => {
            let point = windef::POINT { x: minwindef::LOWORD(wparam as u32) as i32, y: minwindef::HIWORD(wparam as u32) as i32 };
            show_context_menu(hwnd, point);
            0
        },
        _ => handle_wnd_proc_default(hwnd, msg, wparam, lparam)
    }
}

unsafe fn handle_wnd_proc_wm_command(hwnd: windef::HWND, msg: u32, wparam: minwindef::WPARAM, lparam: minwindef::LPARAM) -> minwindef::LRESULT {
    let app_instance_option = (winuser::GetWindowLongPtrW(hwnd, winuser::GWLP_USERDATA) as *mut App).as_mut();

    if let Some(_) = app_instance_option {
        match minwindef::LOWORD(wparam as u32) {
            resources::IDM_PAUSE => { control::pause_app(); modify_notification(hwnd); },
            resources::IDM_RESUME => { control::resume_app(); modify_notification(hwnd); },
            resources::IDM_EXIT => control::exit_app(),
            _ => { return handle_wnd_proc_default(hwnd, msg, wparam, lparam); }
        }
    }

    0
}

unsafe fn handle_wnd_proc_default(hwnd: windef::HWND, msg: u32, wparam: minwindef::WPARAM, lparam: minwindef::LPARAM) -> minwindef::LRESULT {
    winuser::DefWindowProcW(hwnd, msg, wparam, lparam)
}

unsafe fn show_context_menu(hwnd: windef::HWND, point: windef::POINT)
{
    let hinst = winuser::GetWindowLongPtrW(hwnd, winuser::GWLP_HINSTANCE);
    let context_menu_option = winuser::LoadMenuW(hinst as minwindef::HINSTANCE, winuser::MAKEINTRESOURCEW(resources::IDC_CONTEXTMENU)).as_mut();

    if let Some(context_menu) = context_menu_option {
        let submenu_option = winuser::GetSubMenu(context_menu, 0).as_mut();

        if let Some(submenu) = submenu_option {
            let app_instance_option = (winuser::GetWindowLongPtrW(hwnd, winuser::GWLP_USERDATA) as *mut App).as_mut();

            if let Some(_) = app_instance_option {
                let command_to_remove = if control::get_do_pause() { resources::IDM_PAUSE } else { resources::IDM_RESUME };
                winuser::RemoveMenu(submenu, command_to_remove as u32, winuser::MF_BYCOMMAND);

                winuser::SetForegroundWindow(hwnd);                
                winuser::TrackPopupMenuEx(submenu, winuser::TPM_LEFTALIGN, point.x, point.y, hwnd, null_mut());
            }
        }

        winuser::DestroyMenu(context_menu);
    }    
}