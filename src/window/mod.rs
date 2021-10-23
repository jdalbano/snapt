mod dock;
mod monitor;
mod window_transform;

use dock::position::Position as DockPosition;

pub fn dock_left() {
    dock::manager::process_window_dock_change(DockPosition::Left);
}

pub fn dock_right(){
    dock::manager::process_window_dock_change(DockPosition::Right);
}

pub fn dock_top(){
    dock::manager::process_window_dock_change(DockPosition::Top);
}

pub fn dock_bottom(){
    dock::manager::process_window_dock_change(DockPosition::Bottom);
}

pub fn dock_full(){
    dock::manager::process_window_dock_change(DockPosition::Full);
}

pub fn dock_top_left() {
    dock::manager::process_window_dock_change(DockPosition::TopLeft);
}

pub fn dock_top_right(){
    dock::manager::process_window_dock_change(DockPosition::TopRight);
}
pub fn dock_bottom_left() {
    dock::manager::process_window_dock_change(DockPosition::BottomLeft);
}

pub fn dock_bottom_right(){
    dock::manager::process_window_dock_change(DockPosition::BottomRight);
}