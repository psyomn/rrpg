use entity::{Entity};

pub struct Tile {
    entities: Vec<Entity>,
}

impl Tile {
    pub fn new() -> Tile {
        Tile { entities: vec!() }
    }

    pub fn entities(&self) -> &Vec<Entity> {
        &self.entities
    }
}

