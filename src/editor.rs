use std::io::{self, stdout};
use termion::event::Key;
use termion::input::TermRead; // This allows us to do `io::stdin().keys()`
use termion::raw::IntoRawMode;

// The `pub` keyword allows us to access things from other files when imported

// A struct is a collection of variables, and functions can be added with
// `impl`s (below).
pub struct Editor {}

// This allows us to add functions (methods) to structs.
impl Editor {
    // This is a static function
    pub fn new() -> Self {
        return Self {};
    }

    pub fn run(&self) {
        let _stdout = stdout().into_raw_mode().unwrap();

        for key in io::stdin().keys() {
            let key = match key {
                Ok(k) => k,
                Err(e) => return die(e),
            };

            match key {
                // Single character presses
                Key::Char(c) => {
                    if c.is_control() {
                        println!("{:?}\r", c as u8);
                    } else {
                        println!("{:?} ({})\r", c as u8, c);
                    }
                }
                // `q` is pressed whilst holding down Ctrl
                Key::Ctrl('q') => break,
                // Print anything else
                // _ is required here as Match is exhaustive
                _ => println!("{:?}\r", key),
            }
        }
    }
}

fn die(e: std::io::Error) {
    panic!("{:?}", e);
}
