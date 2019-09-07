pub use specs;
use specs::prelude::*;

pub struct Character(pub char);

impl Component for Character {
    type Storage = VecStorage<Self>;
}
