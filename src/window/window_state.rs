
pub enum WindowState {
    Left,
    Right,
    Top,
    Bottom,
    Full,
}

impl std::fmt::Display for WindowState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { 
        write!(f, "{}",
            match self {
                WindowState::Left => "Left",
                WindowState::Right => "Right",
                WindowState::Top => "Top",
                WindowState::Bottom => "Bottom",
                WindowState::Full => "Full",
            })
    }
}