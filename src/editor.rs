use std::io::{self, stdout};
use termion::{event::Key, input::TermRead, raw::IntoRawMode};

pub struct Editor {}

impl Default for Editor {
    fn default() -> Self {
        return Self {};
    }
}

impl Editor {
    pub fn run(&self) {
        // Enter terminal "raw" mode
        let _stdout = stdout().into_raw_mode().unwrap();

        // "Listen" for key inputs from stdin
        for key in io::stdin().keys() {
            match key {
                Ok(key) => match key {
                    Key::Char(c) => Self::handle_character(key, c),
                    Key::Ctrl('q') => break,
                    _ => println!("{key:?}\r"),
                },
                Err(err) => Self::die(&err),
            };
        }
    }

    fn handle_character(key: Key, c: char) {
        let b = c as u8;

        // Do NOT print ASCII control characters
        if c.is_control() {
            println!("{b} \r");
        } else {
            println!("{key:?} ({b})\r");
        }
    }

    fn die(e: &std::io::Error) {
        panic!("{e}");
    }
}
