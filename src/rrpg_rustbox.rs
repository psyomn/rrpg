extern crate rustbox;
use self::rustbox::{Color, RustBox};

use tile::Tile;
use map::Map;

pub trait Rustboxify {
    fn rustboxify(&self, r: &RustBox, x: usize, y: usize) -> ();
}

impl Rustboxify for Tile {
    /// TODO this needs cleaning up
    fn rustboxify(&self, r: &RustBox, x: usize, y: usize) -> () {
        let v: (rustbox::Style, Color, Color, String) = match self.entities().iter().count() {
            0 => (rustbox::RB_BOLD, Color::Green, Color::Black, "`".to_string()),
            v @ 1...2 => (rustbox::RB_NORMAL, Color::Green, Color::Black, format!("{}", v)),
            v @ 3...4 => (rustbox::RB_NORMAL, Color::Yellow, Color::Black, format!("{}", v)),
            v => (rustbox::RB_BOLD, Color::Red, Color::Black, format!("{}", v)),
        };

        let (style, fg, bg, txt) = v;

        r.print(x, y, style, fg, bg, txt.as_ref());
    }
}

impl Rustboxify for Map {
    fn rustboxify(&self, r: &RustBox, x: usize, y: usize) -> () {
        let bld = rustbox::RB_BOLD;
        let (red, black) = (Color::Red, Color::Black);
        r.print(x, y, bld, red, black, "Map");
        match self.tiles().iter().nth(2) {
            Some(v) => v.rustboxify(&r, x+1, y+1),
            None => {},
        }
    }
}
