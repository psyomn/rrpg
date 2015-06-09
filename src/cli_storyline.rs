extern crate rustbox;
use self::rustbox::{Color, RustBox};
use self::rustbox::Key;

use std::error::Error;
use std::default::Default;

use cli;
use cli_constants::{SCREEN_WIDTH, SCREEN_HEIGHT};

/// Given some text, render most of it using the max dimensions of the drawing space.
/// If there's too much text, then wait for user input, to go to the next page, until everything is
/// read.
fn make_pages(r: &RustBox, page_text: String) -> () {
    let mut x_offset : usize = 1;
    let mut y_offset : usize = 1;
    let mut curr_line : String = "".to_string();
    let mut lines : Vec<String> = vec!();
    let words = page_text.split(" ").collect::<Vec<&str>>();
    let mut ix = 0;
    let mut ix = 0;

    cli::clear_screen(&r);

    /* This will build the lines as they need to be displayed */
    lines.push("".to_string());
    for w in words {
        ix = lines.len();

        if x_offset + w.len() + curr_line.len() < SCREEN_WIDTH {
            curr_line = curr_line + &w.to_string();
        }
        else {
            lines.push(curr_line.clone());
            curr_line.clear();
            curr_line = w.to_string();
        }
    }
}
