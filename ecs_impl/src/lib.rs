use specs::prelude::*;

pub struct WCharacter(pub components::Character);
pub struct WPosition(pub components::Position);

impl Component for WCharacter {
    //TODO optimize type
    type Storage = VecStorage<Self>;
}

impl Default for WCharacter {
    fn default() -> Self {
        WCharacter(components::Character::default())
    }
}

impl Component for WPosition {
    //TODO optimize type
    type Storage = VecStorage<Self>;
}

impl Default for WPosition {
    fn default() -> Self {
        WPosition(components::Position::default())
    }
}
