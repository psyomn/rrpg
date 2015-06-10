extern crate rustbox;

use self::rustbox::{Color, RustBox};
use self::rustbox::Key;

use std::error::Error;
use std::default::Default;
use entity::{Entity};
use battle::{Battle};
use levelers::fighter_leveler;
use cli_constants::{SCREEN_WIDTH, SCREEN_HEIGHT};
use cli_storyline;

enum GameState { GameStart, GameExit }

pub fn start() {
    let rustbox = match RustBox::init(Default::default()) {
        Ok(v) => v,
        Err(e) => panic!("{:?}", e),
    };

    match welcome_screen(&rustbox) {
        GameState::GameExit => return,
        _ => {},
    }

    start_game(&rustbox);
}

fn start_game(r: &RustBox) {
    cli_storyline::make_pages(&r, "It was a gloomy night and what a bad one at that".to_string());
    loop {
        clear_screen(&r);
        r.print(1, 1, rustbox::RB_BOLD, Color::White, Color::Black, "HELLO WORLD");
        r.present();
        match r.poll_event(false) {
            Ok(rustbox::Event::KeyEvent(key)) => {
                match key {
                    Some(Key::Enter) => {},
                    _ => break,
                }
            },
            _ => return,
        }
    }
}

fn welcome_screen(r: &RustBox) -> GameState {
    let mut s : String = "Welcome to RRPG!".to_string();
    let mut c : usize = 0;

    loop {
        clear_screen(&r);
        r.print(1, 1, rustbox::RB_BOLD, Color::Yellow, Color::Black, s.as_ref());
        r.print(3, 3, rustbox::RB_BOLD, Color::White, Color::Black, "New Game");
        r.print(3, 4, rustbox::RB_BOLD, Color::White, Color::Black, "Quit");
        r.print(1, 3 + c, rustbox::RB_BOLD, Color::Red, Color::Black, "*");
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

    clear_screen(&r);

    return if c == 0 { GameState::GameStart } else { GameState::GameExit };
}

pub fn clear_screen(r: &RustBox) {
    _clear_screen(&r, SCREEN_WIDTH, SCREEN_HEIGHT);
}

fn _clear_screen(r: &RustBox, w: usize, h: usize) {
    for i in 0..w {
        for j in 0..h {
            r.print(i, j, rustbox::RB_NORMAL, Color::Black, Color::Black, " ");
        }
    }
}

