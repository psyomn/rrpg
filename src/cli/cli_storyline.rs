extern crate rustbox;
use self::rustbox::{Color, RustBox};

use cli::cli;
use cli::cli_constants::{SCREEN_WIDTH, SCREEN_HEIGHT};

const X_OFFSET: usize = 1;
const Y_OFFSET: usize = 1;

/// Given some text, render most of it using the max dimensions of the drawing space.
/// If there's too much text, then wait for user input, to go to the next page, until everything is
/// read.
pub fn make_pages(r: &RustBox, page_text: String) -> () {
    let x_offset : usize = 1;
    let mut curr_line : String = "".to_string();
    let mut lines : Vec<String> = vec!();
    let words = page_text.split(" ").collect::<Vec<&str>>();

    /* This will build the lines as they need to be displayed */
    for w in words {
        if x_offset + w.len() + curr_line.len() + 1 < SCREEN_WIDTH {
            /* +1 for space between words */
            curr_line = curr_line + &w.to_string() + " ";
        }
        else {
            lines.push(curr_line.clone());
            curr_line.clear();
            curr_line = w.to_string() + " ";
        }
    }

    if curr_line != "" {
        lines.push(curr_line.clone());
    }

    draw_pages(&r, lines);
}

fn draw_pages(r: &RustBox, curr_page: Vec<String>) {
    let mut count = Y_OFFSET;
    let mut it = curr_page.iter();
    let mut done : bool = false;

    while !done {
        cli::clear_screen(&r);
        loop {
            if count + Y_OFFSET >= SCREEN_HEIGHT { break; }
            match it.next() {
                Some(line) =>
                    r.print(X_OFFSET, count, rustbox::RB_BOLD, Color::White, Color::Black,
                            line.as_ref()),
                None => { done = true; break },
            }
            count += 1;
        }
        count = Y_OFFSET;
        r.present();

        match r.poll_event(false) {
            _ => continue,
        }
    }
}

