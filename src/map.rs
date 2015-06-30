use tile::{Tile};

pub struct Map {
    tiles: Vec<Tile>,
    x: u32,
    y: u32,
}

impl Map {
    fn new() -> Map {
        Map {
            tiles: vec!(),
            x: 10,
            y: 10,
        }
    }
}

