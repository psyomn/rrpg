extern crate rustbox;

use self::rustbox::{Color, RustBox};
use self::rustbox::Key;

use std::default::Default;
use cli_constants::{SCREEN_WIDTH, SCREEN_HEIGHT};
use storyline_constants::*;
use cli_storyline;
use map::Map;
use cli::map_drawer;

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
    let mut t : &str;
    let mut map = Map::new();
    let pl = "pressed left ";
    let pr = "pressed right";
    let pu = "pressed up   ";
    let pd = "pressed down ";
    let def = "default";

    t = &def;

    cli_storyline::make_pages(&r, INTRODUCTION.to_string());

    loop {
        clear_screen(&r);
        r.print(1, 1, rustbox::RB_BOLD, Color::White, Color::Black, t);
        make_borders(&r, Color::Blue);
        r.present();
        match r.poll_event(false) {
            Ok(rustbox::Event::KeyEvent(key)) => {
                match key {
                    Some(Key::Enter) => {},
                    Some(Key::Right) => t = &pr,
                    Some(Key::Left) => t = &pl,
                    Some(Key::Down) => t = &pd,
                    Some(Key::Up) => t = &pu,
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
        r.print(3, 3, rustbox::RB_BOLD, Color::Yellow, Color::Black, s.as_ref());
        r.print(7, 5, rustbox::RB_BOLD, Color::White, Color::Black, "New Game");
        r.print(7, 6, rustbox::RB_BOLD, Color::White, Color::Black, "Quit");
        r.print(5, 5 + c, rustbox::RB_BOLD, Color::Red, Color::Black, "*");
        make_borders(&r, Color::Blue);
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

            Err(e) => panic!("{}", e),
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


fn make_borders(r: &RustBox, c: Color) -> () {
    for x in 0..SCREEN_WIDTH {
        r.print(x, 0, rustbox::RB_NORMAL, c, c, " ");
        r.print(x, SCREEN_HEIGHT - 1, rustbox::RB_NORMAL, c, c, " ");
    }
    for y in 0..SCREEN_HEIGHT {
        r.print(0, y, rustbox::RB_NORMAL, c, c, " ");
        r.print(SCREEN_WIDTH - 1, y, rustbox::RB_NORMAL, c, c, " ");
    }
}

