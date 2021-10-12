pub struct WindowTransform {
    pub pos_x: i32,
    pub pos_y: i32,
    pub size_x: i32,
    pub size_y: i32,
}

impl WindowTransform {
    pub fn new(pos_x: i32, pos_y: i32, size_x: i32, size_y: i32) -> Self {
        WindowTransform { pos_x, pos_y, size_x, size_y }
    }
}