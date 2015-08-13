use tile::Tile;
use viewport::{Viewport, ViewportBuilder};

pub struct Map {
    tiles: Vec<Tile>,
    x: u32,
    y: u32,
    viewport: Viewport,
}

impl Map {
    pub fn new() -> Map {
        let mut m = Map {
            tiles: vec!(),
            x: 10,
            y: 10,
            viewport: ViewportBuilder::new().default_x().default_y().finalize()
        };
        m.tiles = m.make_default_tiles();
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

    pub fn x(&self) -> u32 { self.x }

    pub fn y(&self) -> u32 { self.y }

    pub fn tiles(&self) -> &Vec<Tile> { &self.tiles }

}

