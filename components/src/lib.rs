pub struct Character(pub String);

impl Default for Character {
    fn default() -> Self {
        Character("0".to_string())
    }
}

pub struct Position {
    pub x: u16,
    pub y: u16,
}

impl Default for Position {
    fn default() -> Self {
        Position { x: 0, y: 0 }
    }
}
