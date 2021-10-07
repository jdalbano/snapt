pub struct WindowTransform {
    pub x: i32,
    pub y: i32,
}

impl WindowTransform {
    pub fn new(x: i32, y: i32) -> Self {
        WindowTransform { x, y }
    }
}