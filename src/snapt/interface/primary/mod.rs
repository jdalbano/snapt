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
use crate::snapt::resources;

use crate::snapt::interface::base::InterfaceBase;

const CLASS_NAME: &str = "PrimaryInterface";
const NOTIFICATION_ID: u32 = 3434773434;
const NOTIFICATION_CALLBACK: u32 = winuser::WM_APP + 1;
const PAUSED_TEXT: &str = " (Paused)";
const WM_COMMAND: u32 = winuser::WM_COMMAND as u32;

pub struct PrimaryInterface {
    window: windef::HWND,
    notification: shellapi::NOTIFYICONDATAW,
}

impl PrimaryInterface {
    pub fn new() -> Self {
        unsafe {
            try_create_interface().expect("failed to create primary interface")
        }
    }

    fn bind_interface_to_window(&mut self) {
        unsafe {
            winuser::SetWindowLongPtrW(self.window, winuser::GWLP_USERDATA, (self as *mut PrimaryInterface) as isize);
        }
    }

    fn destroy_window(&self) {
        unsafe {
            winuser::PostMessageW(self.window, winuser::WM_CLOSE, 0, 0);
        }
    }
    
    fn add_notification(&mut self) {
        unsafe {
            shellapi::Shell_NotifyIconW(shellapi::NIM_ADD, &mut self.notification);
            shellapi::Shell_NotifyIconW(shellapi::NIM_SETVERSION, &mut self.notification);
        }
    }
    
    fn destroy_notification(&mut self) {
        unsafe {
            shellapi::Shell_NotifyIconW(shellapi::NIM_DELETE, &mut self.notification);
        } 
    }

    fn pause_app(&mut self) {
        app::pause_app();
        self.modify_notification(get_tooltip(true));
    }

    fn resume_app(&mut self) {
        app::resume_app();
        self.modify_notification(get_tooltip(false));
    }

    fn exit_app(&self) {
        app::exit_app();
    }

    fn modify_notification(&mut self, new_tooltip: [u16; 128]) {
        unsafe {
            self.notification.szTip = new_tooltip;
            shellapi::Shell_NotifyIconW(shellapi::NIM_MODIFY, &mut self.notification);
        }
    }
}

impl InterfaceBase for PrimaryInterface {
    fn init(&mut self){
        self.bind_interface_to_window();
        self.add_notification();
    }   

    fn check_should_close(&self) -> bool {
        app::get_do_exit()
    }

    fn handle_messages(&self) -> bool {
        let mut message: mem::MaybeUninit<winuser::MSG> = mem::MaybeUninit::uninit();

        unsafe {
            if winuser::GetMessageW(message.as_mut_ptr() as *mut winuser::MSG, self.window, 0, 0 ) > 0 {
                winuser::TranslateMessage(message.as_ptr() as *const winuser::MSG);    
                winuser::DispatchMessageW(message.as_ptr() as *const winuser::MSG);
                true
            } else {
                false
            }
        }
    }

    fn destroy(&mut self) {
        self.destroy_notification();
        self.destroy_window();
    }
}

unsafe fn try_create_interface() -> Result<PrimaryInterface, Error> {
    let class_name = get_class_name();
    let module = libloaderapi::GetModuleHandleW(null_mut());    
    let wnd_class = create_wnd_class(&class_name, module);

    winuser::RegisterClassW(&wnd_class);

    let window = create_window_handle(&class_name, module);
    let notification = create_notification(window, module);    

    if window.is_null() {
        Err(Error::last_os_error())
    } else {
        Ok(PrimaryInterface { window, notification })
    }
}

fn get_class_name() -> Vec<u16> {
    OsStr::new(CLASS_NAME).encode_wide().chain(Some(0).into_iter()).collect::<Vec<u16>>()
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

fn get_tooltip(is_paused: bool) -> [u16; 128] {
    let tooltip = app::APP_NAME.to_owned() + if is_paused { PAUSED_TEXT } else { "" };

    let tooltip_vec = OsStr::new(&tooltip).encode_wide().chain(Some(0).into_iter()).collect::<Vec<u16>>();
    let mut tooltip_sz: [u16; 128] = [0; 128];
    tooltip_sz[..tooltip_vec.len()].copy_from_slice(&tooltip_vec); 

    tooltip_sz
}

fn get_icon() -> Vec<u16> {
    OsStr::new(resources::MAIN_ICON).encode_wide().chain(Some(0).into_iter()).collect()
}

unsafe fn create_notification(window: windef::HWND, module: minwindef::HMODULE) -> shellapi::NOTIFYICONDATAW {
    let tooltip = get_tooltip(false);    
    let ico_resource = get_icon();

    let mut notification = shellapi::NOTIFYICONDATAW {
        cbSize: mem::size_of::<shellapi::NOTIFYICONDATAW>() as u32,
        hWnd: window,
        uID: NOTIFICATION_ID,
        uCallbackMessage: NOTIFICATION_CALLBACK,
        hIcon: winuser::LoadIconW(module, ico_resource.as_ptr()),
        szTip: tooltip,
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
    let primary_interface_option = (winuser::GetWindowLongPtrW(hwnd, winuser::GWLP_USERDATA) as *mut PrimaryInterface).as_mut();

    if let Some(primary_interface) = primary_interface_option {
        match minwindef::LOWORD(wparam as u32) {
            resources::IDM_PAUSE => { primary_interface.pause_app(); },
            resources::IDM_RESUME => { primary_interface.resume_app(); },
            resources::IDM_EXIT => { primary_interface.exit_app(); },
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
                let command_to_remove = if app::get_do_pause() { resources::IDM_PAUSE } else { resources::IDM_RESUME };
                winuser::RemoveMenu(submenu, command_to_remove as u32, winuser::MF_BYCOMMAND);

                winuser::SetForegroundWindow(hwnd);                
                winuser::TrackPopupMenuEx(submenu, winuser::TPM_LEFTALIGN, point.x, point.y, hwnd, null_mut());
            }
        }

        winuser::DestroyMenu(context_menu);
    }    
}
