mod dock_manager;
mod dock_position;
mod window_transform;

use dock_position::*;

pub fn dock_left() {
    dock_manager::process_window_dock_change(DockPosition::Left);
}

pub fn dock_right(){
    dock_manager::process_window_dock_change(DockPosition::Right);
}

pub fn dock_top(){
    dock_manager::process_window_dock_change(DockPosition::Top);
}

pub fn dock_bottom(){
    dock_manager::process_window_dock_change(DockPosition::Bottom);
}

pub fn dock_full(){
    dock_manager::process_window_dock_change(DockPosition::Full);
}