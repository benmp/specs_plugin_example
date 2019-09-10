use specs::prelude::*;

pub struct Character(pub String);

impl Default for Character {
    fn default() -> Self {
        Character("0".to_string())
    }
}

impl Component for Character {
    //TODO optimize type
    type Storage = VecStorage<Self>;
}

pub struct Position {
    pub x: u16,
    pub y: u16,
}

impl Component for Position {
    type Storage = VecStorage<Self>;
}

impl Default for Position {
    fn default() -> Self {
        Position { x: 0, y: 0 }
    }
}
