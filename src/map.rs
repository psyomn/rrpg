use tile::{Tile};

pub struct Map {
    tiles: Vec<Tile>,
}

impl Map {
    fn new() -> Map {
        Map { tiles: vec!() }
    }
}

