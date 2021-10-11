mod window_manager;
mod window_state;
mod window_transform;

use window_state::*;

pub fn dock_left() {
    window_manager::process_window_state_change(WindowState::Left);
}

pub fn dock_right(){
    window_manager::process_window_state_change(WindowState::Right);
}

pub fn dock_top(){
    window_manager::process_window_state_change(WindowState::Top);
}

pub fn dock_bottom(){
    window_manager::process_window_state_change(WindowState::Bottom);
}

pub fn dock_full(){
    window_manager::process_window_state_change(WindowState::Full);
}