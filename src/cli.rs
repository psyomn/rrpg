extern crate rustbox;

use self::rustbox::{Color, RustBox};
use self::rustbox::Key;

use std::error::Error;
use std::default::Default;
use entity::{Entity};
use battle::{Battle};
use levelers::fighter_leveler;

enum GameState { GameStart, GameExit }

pub fn start() {
    let rustbox = match RustBox::init(Default::default()) {
        Ok(v) => v,
        Err(e) => panic!("{:?}", e),
    };

    let ret = welcome_screen(&rustbox);
}

fn welcome_screen(r: &RustBox) -> GameState {
    let mut s : String = "Welcome to rrpg!".to_string();
    let mut c : usize = 0;

    loop {
        clear_screen(&r, 80, 40);
        r.print(1, 1, rustbox::RB_BOLD, Color::White, Color::Black, s.as_ref());
        r.print(3, 3, rustbox::RB_BOLD, Color::White, Color::Black, "New Game");
        r.print(3, 5, rustbox::RB_BOLD, Color::White, Color::Black, "Quit");
        r.print(1, 3 + c * 2, rustbox::RB_BOLD, Color::Red, Color::Black, "*");
        r.present();

        match r.poll_event(false) {
            Ok(rustbox::Event::KeyEvent(key)) => {
                match key {
                    Some(Key::Up)   => { c = 0;},
                    Some(Key::Down) => { c = 1;},
                    Some(Key::Enter) => { break; },
                    _ => { s = "Use up and down keys, and press enter!".to_string(); },
                }
            },

            Err(e) => { },
            _ => {},
        }
    }

    return if c == 0 { GameState::GameStart } else { GameState::GameExit };
}

fn clear_screen(r: &RustBox, w: usize, h: usize) {
    for i in 0..w {
        for j in 0..h {
            r.print(i, j, rustbox::RB_NORMAL, Color::Black, Color::Black, " ");
        }
    }
}

