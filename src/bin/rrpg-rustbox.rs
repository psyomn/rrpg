extern crate rrpg;
extern crate rustbox;

use std::error::Error;
use std::default::Default;

use rustbox::{Color, RustBox};
use rustbox::Key;

use rrpg::entity::{Entity};
use rrpg::battle::{Battle};
use rrpg::levelers::fighter_leveler;

fn main() {
    let rustbox = match RustBox::init(Default::default()) {
        Ok(v) => v,
        Err(e) => panic!("{:?}", e),
    };

    let mut s : String = "Hello world".to_string();

    loop {
        rustbox.print(1, 1, rustbox::RB_BOLD, Color::White, Color::Black, s.as_ref());
        rustbox.print(1, 3, rustbox::RB_BOLD, Color::White, Color::Black, "Press 'q' to quit");
        rustbox.present();

        match rustbox.poll_event(false) {
            Ok(rustbox::Event::KeyEvent(key)) => {
                match key {
                    Some(Key::Char('q')) => { break; },
                    _ => { s = "Wrong key!!".to_string(); },
                }
            },

            Err(e) => { },
            _ => {},

        }
    }
}
