use std::ptr::null_mut;

use winapi::shared::minwindef;
use winapi::shared::windef;
use winapi::um::winuser;

pub struct Info {
    pub monitor_rects: Option<Vec::<windef::RECT>>,
}

impl Info {
    pub fn new() -> Self {
        Info { monitor_rects: None }
    }

    pub fn prepare_monitor_info(&mut self) {
        self.monitor_rects = None;

        unsafe {
            winuser::EnumDisplayMonitors(null_mut(), null_mut(), Some(monitor_enum_proc), (self as *mut Info) as isize);
        }
    }

    fn add_monitor_rect(&mut self, rect: windef::LPRECT) {
        unsafe {
            if let Some(combined_rect) = &self.monitor_rects {
                let mut rects_concat = combined_rect.clone();
                rects_concat.push(*rect);

                self.monitor_rects = Some(rects_concat);
            } else {
                self.monitor_rects = Some(vec!(*rect));
            }
        }
    }
}

unsafe extern "system" fn monitor_enum_proc(_hmonitor: windef::HMONITOR, _hdc: windef::HDC, lprect: windef::LPRECT, lparam: minwindef::LPARAM) -> i32 {
    let info = &mut *(lparam as *mut Info);
    info.add_monitor_rect(lprect);

    true as i32
}