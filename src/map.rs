use tile::Tile;
use viewport::{Viewport, ViewportBuilder};
use std::ops::Range;

pub struct Map {
    tiles: Vec<Tile>,
    x: usize,
    y: usize,
    shift_x: usize,
    shift_y: usize,
    viewport: Viewport,
}

impl Map {
    pub fn new() -> Map {
        let set_x: usize = 10;
        let set_y: usize = 10;

        let mut m = Map {
            tiles: vec!(),
            x: set_x,
            y: set_y,
            shift_x: 0,
            shift_y: 0,
            viewport: ViewportBuilder::new()
                .default_y()
                .default_x()
                .finalize()
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

    /// Height of map
    pub fn x(&self) -> usize { self.x }

    /// Width of map
    pub fn y(&self) -> usize { self.y }

    /// Readonly reference to tiles
    pub fn tiles(&self) -> &Vec<Tile> { &self.tiles }

    /// Maximum possible viewable portion w, with respect to viewport
    pub fn max_viewport_w(&self) -> usize {
        self.viewport.width()
    }

    /// Maximum possible viewable portion h, with respect to viewport
    pub fn max_viewport_h(&self) -> usize {
        self.viewport.height()
    }

    pub fn max_viewable_h(&self) -> usize {
        let h64 = self.y() as u64;
        let vh64 = self.max_viewport_h() as u64;
        let ret = h64 % vh64;
        ret as usize
    }

    pub fn max_viewable_w(&self) -> usize {
        let w64 = self.x() as u64;
        let vw64 = self.max_viewport_w() as u64;
        let ret = w64 % vw64;
        ret as usize
    }

    pub fn shift_x(&self) -> usize {
        self.shift_x
    }

    pub fn shift_y(&self) -> usize {
        self.shift_y
    }
}

