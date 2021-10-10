use std::mem::size_of;
use winapi;

use crate::window::window_transform::*;
use crate::window::window_state::*;

type HWND = winapi::shared::windef::HWND__;
type RECT = winapi::shared::windef::RECT;
type MONTIORINFO = winapi::um::winuser::MONITORINFO;
type LPVOID = winapi::shared::minwindef::LPVOID;

pub fn process_window_state_change(state: WindowState) {
    if let WindowState::None = state {
        return
    }

    unsafe{                
        let window = winapi::um::winuser::GetForegroundWindow();

        let can_window_be_resized = check_if_window_can_be_resized(&mut *window);

        if can_window_be_resized {
            change_window_state(&mut *window, state);
        }
    }
}

unsafe fn check_if_window_can_be_resized(window: &mut HWND) -> bool {
    let window_style = winapi::um::winuser::GetWindowLongA(window, winapi::um::winuser::GWL_STYLE);
    
    (window_style & winapi::um::winuser::WS_MAXIMIZEBOX as i32) != 0
}

unsafe fn change_window_state(window: &mut HWND, state: WindowState) {
    let (did_window_bounds_succeed, window_bounds) = get_window_bounds(window);
    let (did_shadow_bounds_succeed, shadow_bounds) = get_shadow_bounds(window);

    if did_window_bounds_succeed && did_shadow_bounds_succeed {
        let screen_transform_result = get_screen_transforms(window);

        if let Ok((screen_pos, screen_size)) = screen_transform_result {
            let (shadow_pos_offset, shadow_size_offset) = get_shadow_offsets(window_bounds, shadow_bounds);

            restore_window(window);
            
            let pos_i = WindowTransform::new(window_bounds.left, window_bounds.top);
            let size_i = WindowTransform::new(window_bounds.right + shadow_size_offset.x, window_bounds.bottom + shadow_size_offset.y);
            
            let transform_result = get_transform_for_window_state(screen_pos, screen_size, shadow_pos_offset, shadow_size_offset, state);
            
            if let Ok((pos_f, size_f)) = transform_result {
                set_window_pos_and_size(window, pos_i, size_i, pos_f, size_f);
            }
        }
    }
}

unsafe fn restore_window(window: &mut HWND) {
    winapi::um::winuser::ShowWindow(window, winapi::um::winuser::SW_NORMAL);
}

unsafe fn get_window_bounds(window: &mut HWND) -> (bool, RECT) {
    let window_rect = &mut RECT { left: 0, right:0, top: 0, bottom: 0 } as *mut RECT;
    let window_rect_result = winapi::um::winuser::GetWindowRect(window, window_rect);

    (window_rect_result != 0, *window_rect)
}

unsafe fn get_shadow_bounds(window: &mut HWND) -> (bool, RECT) {
    let shadow_rect = &mut RECT { left: 0, right:0, top: 0, bottom: 0 } as *mut RECT;
    let shadow_rect_result = winapi::um::dwmapi::DwmGetWindowAttribute(
        window,
        winapi::um::dwmapi::DWMWA_EXTENDED_FRAME_BOUNDS, 
        shadow_rect as LPVOID, 
        size_of::<RECT>() as u32);

    (shadow_rect_result == 0, *shadow_rect)
}

fn get_shadow_offsets(window_rect: RECT, shadow_rect: RECT) -> (WindowTransform, WindowTransform) {
    let shadow_pos_offset = WindowTransform::new(window_rect.left - shadow_rect.left, window_rect.top - shadow_rect.top);
    let shadow_size_offset = WindowTransform::new(window_rect.right - shadow_rect.right + (-1 * shadow_pos_offset.x), window_rect.bottom - shadow_rect.bottom + (-1 * shadow_pos_offset.y));
    (shadow_pos_offset, shadow_size_offset)
}

fn get_transform_for_window_state(screen_pos: WindowTransform, screen_size: WindowTransform, shadow_pos_offset: WindowTransform, shadow_size_offset: WindowTransform, state: WindowState) -> Result<(WindowTransform, WindowTransform), ()>  {
    let half_cx = screen_size.x / 2;
    let half_cy = screen_size.y / 2;

    let state_result =
        match state {
            WindowState::Left => Some((
                screen_pos.x + shadow_pos_offset.x, 
                screen_pos.y + shadow_pos_offset.y, 
                half_cx + shadow_size_offset.x, 
                screen_size.y + shadow_size_offset.y)),
            WindowState::Right => Some((
                screen_pos.x + half_cx + shadow_pos_offset.x, 
                screen_pos.y + shadow_pos_offset.y, 
                half_cx + shadow_size_offset.x, 
                screen_size.y + shadow_size_offset.y)),
            WindowState::Top => Some((
                screen_pos.x + shadow_pos_offset.x, 
                screen_pos.y + shadow_pos_offset.y, 
                screen_size.x + shadow_size_offset.x, 
                half_cy + shadow_size_offset.y)),
            WindowState::Bottom => Some((
                screen_pos.x + shadow_pos_offset.x, 
                screen_pos.y + half_cy + shadow_pos_offset.y, 
                screen_size.x + shadow_size_offset.x, 
                half_cy + shadow_size_offset.y)),
            WindowState::Full => Some((
                screen_pos.x + shadow_pos_offset.x, 
                screen_pos.y + shadow_pos_offset.y, 
                screen_size.x + shadow_size_offset.x, 
                screen_size.y + shadow_size_offset.y)),
            WindowState::None => None,
        };
    
    if let Some((pos_x, pos_y, size_x, size_y)) = state_result {
        Ok((WindowTransform::new(pos_x, pos_y), WindowTransform::new(size_x, size_y)))
    }
    else {
        Err(())
    }
}

unsafe fn get_screen_transforms(window: &mut HWND) -> Result<(WindowTransform, WindowTransform), ()> {
    let monitor_info_result = get_current_monitor_info(window);

   if let Ok(monitor_info) = monitor_info_result {
        let work_area: RECT = monitor_info.rcWork;
        Ok((
            WindowTransform::new(work_area.left, work_area.top),
            WindowTransform::new(work_area.right - work_area.left, work_area.bottom - work_area.top)))
    }
    else {
        Err(())
    }
}

unsafe fn get_current_monitor_info(window: &mut HWND) -> Result<MONTIORINFO, ()> {
    let monitor = winapi::um::winuser::MonitorFromWindow(window, winapi::um::winuser::MONITOR_DEFAULTTONEAREST);

    let monitor_info = &mut MONTIORINFO {
        cbSize: size_of::<MONTIORINFO>() as u32,
        rcMonitor: RECT { left: 0, right:0, top: 0, bottom: 0 },
        rcWork: RECT { left: 0, right:0, top: 0, bottom: 0 },
        dwFlags: 0,
    } as *mut MONTIORINFO;

    let did_monitor_info_succeed = winapi::um::winuser::GetMonitorInfoA(monitor, monitor_info);

    if did_monitor_info_succeed != 0 {
        Ok(*monitor_info)
    }
    else {
        Err(())
    }
}

unsafe fn set_window_pos_and_size(window: &mut HWND, pos_i: WindowTransform, size_i: WindowTransform, pos_f: WindowTransform, size_f: WindowTransform) {
    if pos_i.x != pos_f.x || pos_i.y != pos_f.y || size_i.x != size_f.x || size_i.y != size_f.y {                
        winapi::um::winuser::SetWindowPos(window, winapi::um::winuser::HWND_TOP, pos_f.x, pos_f.y, size_f.x, size_f.y, winapi::um::winuser::SWP_SHOWWINDOW);
    }
}