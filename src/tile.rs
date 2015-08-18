use entity::{Entity};

use rrpg_rustbox;

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

    pub fn add_entity(&mut self, e: Entity) -> () {
        self.entities.push(e);
    }
}

