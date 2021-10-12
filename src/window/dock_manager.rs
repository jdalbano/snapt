use std::mem::size_of;
use winapi;

use crate::window::window_transform::*;
use crate::window::dock_position::*;

type HWND = winapi::shared::windef::HWND__;
type RECT = winapi::shared::windef::RECT;
type MONTIORINFO = winapi::um::winuser::MONITORINFO;
type LPVOID = winapi::shared::minwindef::LPVOID;

const CHANGE_THRESHOLD: i32 = 1;

pub fn process_window_dock_change(dock_position: DockPosition) {
    unsafe{                
        let window = winapi::um::winuser::GetForegroundWindow();

        let can_window_be_resized = check_if_window_can_be_resized(&mut *window);

        if can_window_be_resized {
            change_window_dock_position(&mut *window, dock_position);
        }
    }
}

unsafe fn check_if_window_can_be_resized(window: &mut HWND) -> bool {
    let window_style = winapi::um::winuser::GetWindowLongA(window, winapi::um::winuser::GWL_STYLE);
    
    (window_style & winapi::um::winuser::WS_MAXIMIZEBOX as i32) != 0
}

unsafe fn change_window_dock_position(window: &mut HWND, dock_position: DockPosition) {
    restore_window(window);

    let (did_window_bounds_succeed, window_bounds) = get_window_bounds(window);
    let (did_shadow_bounds_succeed, shadow_bounds) = get_shadow_bounds(window);

    if did_window_bounds_succeed && did_shadow_bounds_succeed {
        let screen_transform_result = get_screen_transforms(window);

        if let Ok(screen_transform) = screen_transform_result {
            let shadow_offset_transform = get_shadow_offsets(window_bounds, shadow_bounds);

            let initial_transform = get_initial_window_transform(&window_bounds, &shadow_bounds, &shadow_offset_transform);
            
            let final_transform_result = get_transform_for_dock_position(dock_position, screen_transform, shadow_offset_transform);
            
            if let Ok(final_transform) = final_transform_result {
                set_window_transform(window, initial_transform, final_transform);
            }
        }
    }
}

fn get_initial_window_transform(window_bounds: &RECT, shadow_bounds: &RECT, shadow_offset_transform: &WindowTransform) -> WindowTransform {
    WindowTransform::new(
        window_bounds.left, 
        window_bounds.top,
        shadow_bounds.right - window_bounds.left + shadow_offset_transform.size_x + shadow_offset_transform.pos_x, 
        shadow_bounds.bottom - window_bounds.top + shadow_offset_transform.size_y + shadow_offset_transform.pos_y)
}

unsafe fn restore_window(window: &mut HWND) {
    winapi::um::winuser::ShowWindow(window, winapi::um::winuser::SW_SHOWNOACTIVATE);
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

fn get_shadow_offsets(window_rect: RECT, shadow_rect: RECT) -> WindowTransform {
    let pos_x = window_rect.left - shadow_rect.left;
    let pos_y = window_rect.top - shadow_rect.top;

    WindowTransform::new(
        pos_x, 
        pos_y,
        window_rect.right - shadow_rect.right - pos_x, 
        window_rect.bottom - shadow_rect.bottom - pos_y)
}

fn get_transform_for_dock_position(dock_position: DockPosition, screen_transform: WindowTransform, shadow_offset_transform: WindowTransform) -> Result<WindowTransform, ()>  {
    let (mut pos_correction, mut size_correction_x, mut size_correction_y) = (0, 0, 0);

    let has_shadow_offset = shadow_offset_transform.pos_x != 0;

    if has_shadow_offset {
        pos_correction -= 1;
        size_correction_x += 1;
        size_correction_y += 1;
    }
    
    let half_cx = screen_transform.size_x / 2;
    let half_cy = screen_transform.size_y / 2;

    let dock_position_result =
        match dock_position {
            DockPosition::Left => Some((
                screen_transform.pos_x + shadow_offset_transform.pos_x + pos_correction, 
                screen_transform.pos_y + shadow_offset_transform.pos_y, 
                half_cx + shadow_offset_transform.size_x + size_correction_x * 2, 
                screen_transform.size_y + shadow_offset_transform.size_y + size_correction_y)),
            DockPosition::Right => Some((
                screen_transform.pos_x + half_cx + shadow_offset_transform.pos_x, 
                screen_transform.pos_y + shadow_offset_transform.pos_y, 
                half_cx + shadow_offset_transform.size_x + size_correction_x, 
                screen_transform.size_y + shadow_offset_transform.size_y + size_correction_y)),
            DockPosition::Top => Some((
                screen_transform.pos_x + shadow_offset_transform.pos_x + pos_correction, 
                screen_transform.pos_y + shadow_offset_transform.pos_y, 
                screen_transform.size_x + shadow_offset_transform.size_x + size_correction_x * 2, 
                half_cy + shadow_offset_transform.size_y + size_correction_y)),
            DockPosition::Bottom => Some((
                screen_transform.pos_x + shadow_offset_transform.pos_x + pos_correction, 
                screen_transform.pos_y + half_cy + shadow_offset_transform.pos_y, 
                screen_transform.size_x + shadow_offset_transform.size_x + size_correction_x * 2, 
                half_cy + shadow_offset_transform.size_y + size_correction_y)),
            DockPosition::Full => Some((
                screen_transform.pos_x + shadow_offset_transform.pos_x + pos_correction, 
                screen_transform.pos_y + shadow_offset_transform.pos_y, 
                screen_transform.size_x + shadow_offset_transform.size_x + size_correction_x * 2, 
                screen_transform.size_y + shadow_offset_transform.size_y + size_correction_y)),
        };
    
    if let Some((pos_x, pos_y, size_x, size_y)) = dock_position_result {
        Ok(WindowTransform::new(pos_x, pos_y, size_x, size_y))
    }
    else {
        Err(())
    }
}

unsafe fn get_screen_transforms(window: &mut HWND) -> Result<WindowTransform, ()> {
    let monitor_info_result = get_current_monitor_info(window);

   if let Ok(monitor_info) = monitor_info_result {
        let work_area: RECT = monitor_info.rcWork;
        Ok(
            WindowTransform::new(
                work_area.left, 
                work_area.top,
                work_area.right - work_area.left,
                work_area.bottom - work_area.top))
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

unsafe fn set_window_transform(window: &mut HWND, initial_transform: WindowTransform, final_transform: WindowTransform) {
    let has_pos_x_changed = (initial_transform.pos_x - final_transform.pos_x).abs() > CHANGE_THRESHOLD;
    let has_pos_y_changed = (initial_transform.pos_y - final_transform.pos_y).abs() > CHANGE_THRESHOLD;
    let has_size_x_changed = (initial_transform.size_x - final_transform.size_x).abs() > CHANGE_THRESHOLD;
    let has_size_y_changed = (initial_transform.size_y - final_transform.size_y).abs() > CHANGE_THRESHOLD;

    if has_pos_x_changed || has_pos_y_changed || has_size_x_changed || has_size_y_changed {
        winapi::um::winuser::SetWindowPos(window, winapi::um::winuser::HWND_TOP, final_transform.pos_x, final_transform.pos_y, final_transform.size_x, final_transform.size_y, winapi::um::winuser::SWP_SHOWWINDOW);
        winapi::um::winuser::SetActiveWindow(window);
    }
}