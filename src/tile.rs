use entity::{Entity};

pub struct Tile {
    entities: Vec<Entity>,
}

impl Tile {
    fn new() -> Tile {
        Tile { entities: vec!() }
    }
}

