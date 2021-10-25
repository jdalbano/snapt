use winapi::shared::minwindef;
use winapi::shared::windef;

pub trait Notification {
    fn new(window: windef::HWND, module: minwindef::HMODULE, tooltip: String) -> Self;
    
    fn destroy();
}