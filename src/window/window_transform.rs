use winapi::shared::windef::RECT;

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

    pub fn to_rect(&self) -> RECT {
        RECT { left: self.pos_x, right: self.pos_x + self.size_x, top: self.pos_y, bottom: self.pos_y + self.size_y }
    }
}

impl PartialEq for WindowTransform {
    fn eq(&self, other: &Self) -> bool {
        self.pos_x == other.pos_x &&
        self.pos_y == other.pos_y &&
        self.size_x == other.size_x &&
        self.size_y == other.size_y
    }
}

impl Eq for WindowTransform {}