extern crate rustbox;

use self::rustbox::{Color, RustBox};
use self::rustbox::Key;

use std::error::Error;
use std::default::Default;
use entity::{Entity};
use battle::{Battle};
use levelers::fighter_leveler;

pub fn start() {
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

