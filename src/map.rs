use tile::{Tile};

pub struct Map {
    tiles: Vec<Tile>,
    x: u32,
    y: u32,
}

impl Map {
    pub fn new() -> Map {
        let mut m = Map {
            tiles: vec!(),
            x: 10,
            y: 10,
        };

        m.make_default_tiles();

        m
    }

    fn make_default_tiles(&self) -> Vec<Tile> {
        let mut v : Vec<Tile> = vec!();
        for _ in 0 .. self.x {
            for _ in 0 .. self.y {
                v.push(Tile::new());
            }
        }
        v
    }
}

