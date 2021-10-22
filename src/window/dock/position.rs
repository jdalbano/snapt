#[derive(PartialEq)]
pub enum Position {
    Left,
    Right,
    Top,
    Bottom,
    Full,
}

impl std::fmt::Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { 
        write!(f, "{}",
            match self {
                Position::Left => "Left",
                Position::Right => "Right",
                Position::Top => "Top",
                Position::Bottom => "Bottom",
                Position::Full => "Full",
            })
    }
}

impl Position {
    pub fn get_opposite_position(&self) -> Option<Position> {
        match self {
            Position::Left => Some(Position::Right),
            Position::Right => Some(Position::Left),
            Position::Top => Some(Position::Bottom),
            Position::Bottom => Some(Position::Top),
            _ => None,
        }
    }
}