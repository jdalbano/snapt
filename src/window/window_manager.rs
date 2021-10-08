use winapi;

use crate::window::window_transform::*;

type HWND = winapi::shared::windef::HWND__;
type RECT = winapi::shared::windef::RECT;
type MONTIORINFO = winapi::um::winuser::MONITORINFO;
type HMONITOR = winapi::shared::windef::HMONITOR;

pub enum WindowState {
    None,
    Left,
    Right,
    Top,
    Bottom,
    Full,
}

impl std::fmt::Display for WindowState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { 
        write!(f, "{}",
            match self {
                WindowState::None => "None",
                WindowState::Left => "Left",
                WindowState::Right => "Right",
                WindowState::Top => "Top",
                WindowState::Bottom => "Bottom",
                WindowState::Full => "Full",
            })
    }
}

pub fn process_window_state_change(state: WindowState) {
    if let WindowState::None = state {
        return
    }

    unsafe{                
        let window = winapi::um::winuser::GetForegroundWindow();
        let rect = &mut RECT { left: 0, right:0, top: 0, bottom: 0 } as *mut RECT;
    
        let did_succeed = winapi::um::winuser::GetWindowRect(window, rect);
    
        if did_succeed != 0 {
            let pos_i = WindowTransform::new((*rect).left, (*rect).top);

            let size_result = get_screen_size(&mut *window);

            if let Ok(size_i) = size_result {
                let transform_result = get_transform_for_window_state(&size_i, state);
                
                if let Ok((pos_f, size_f)) = transform_result {
                    change_window_to_state(&mut *window, pos_i, size_i, pos_f, size_f);
                }
            }
        }
    }
}

unsafe fn change_window_to_state(window: &mut HWND, pos_i: WindowTransform, size_i: WindowTransform, pos_f: WindowTransform, size_f: WindowTransform) {
    if pos_i.x != pos_f.x || pos_i.y != pos_f.y || size_i.x != size_f.x || size_i.y != size_f.y {                
        winapi::um::winuser::SetWindowPos(window, winapi::um::winuser::HWND_TOP, pos_f.x, pos_f.y, size_f.x, size_f.y, winapi::um::winuser::SWP_SHOWWINDOW);
    }
}

fn get_transform_for_window_state(screen_size: &WindowTransform, state: WindowState) -> Result<(WindowTransform, WindowTransform), ()>  {
    let half_cx = screen_size.x / 2;
    let half_cy = screen_size.y / 2;

    match state {
        WindowState::Left => Ok((WindowTransform::new(0, 0), WindowTransform::new(half_cx, screen_size.y))),
        WindowState::Right => Ok((WindowTransform::new(half_cx, 0), WindowTransform::new(half_cx, screen_size.y))),
        WindowState::Top => Ok((WindowTransform::new(0, 0), WindowTransform::new(screen_size.x, half_cy))),
        WindowState::Bottom => Ok((WindowTransform::new(0, half_cy), WindowTransform::new(screen_size.x, half_cy))),
        WindowState::Full => Ok((WindowTransform::new(0, 0), WindowTransform::new(screen_size.x, screen_size.y))),
        WindowState::None => Err(()),
    }
}

unsafe fn get_screen_size(window: &mut HWND) -> Result<WindowTransform, ()> {
    //let monitor_info_result = get_current_monitor_info(window);

   //if let Ok(monitor_info) = monitor_info_result {
        //monitor_info.
        let display = winapi::um::winuser::GetDesktopWindow();
        let rect = &mut RECT { left: 0, right:0, top: 0, bottom: 0 } as *mut RECT;
    
        let did_rect_succeed = winapi::um::winuser::GetWindowRect(display, rect);
    
        if did_rect_succeed != 0 {
            Ok(WindowTransform::new((*rect).right - (*rect).left, (*rect).bottom - (*rect).top))
        }
        else {
            Err(())
        }
    // }
    // else {
    //     Err(())
    // }
}

unsafe fn get_current_monitor_info(window: &mut HWND) -> Result<MONTIORINFO, ()> {
    let monitor = winapi::um::winuser::MonitorFromWindow(window, winapi::um::winuser::MONITOR_DEFAULTTONEAREST);

    let monitor_info = &mut MONTIORINFO {
        cbSize: 0,
        rcMonitor: RECT { left: 0, right:0, top: 0, bottom: 0 },
        rcWork: RECT { left: 0, right:0, top: 0, bottom: 0 },
        dwFlags: 0,
    };

    let did_monitor_info_succeed = winapi::um::winuser::GetMonitorInfoA(monitor, monitor_info);

    if did_monitor_info_succeed != 0 {
        Ok(*monitor_info)
    }
    else {
        Err(())
    }
}