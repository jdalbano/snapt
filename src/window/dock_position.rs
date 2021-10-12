pub enum DockPosition {
    Left,
    Right,
    Top,
    Bottom,
    Full,
}

impl std::fmt::Display for DockPosition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { 
        write!(f, "{}",
            match self {
                DockPosition::Left => "Left",
                DockPosition::Right => "Right",
                DockPosition::Top => "Top",
                DockPosition::Bottom => "Bottom",
                DockPosition::Full => "Full",
            })
    }
}