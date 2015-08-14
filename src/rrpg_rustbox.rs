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
    /// This is the loop that treats the vector as a 2d vector, and invokes the draw on each tile
    fn rustboxify(&self, r: &RustBox, x: usize, y: usize) -> () {
        for h in 0..self.max_viewable_h() {
            for w in 0..self.max_viewable_w() {
                let curpos = (h * w) + w;
                match self.tiles().iter().nth(curpos) {
                    Some(t) => t.rustboxify(r, w, h),
                    None => {},
                }
            }
        }
    }
}
