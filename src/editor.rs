use std::io::{self, stdout};
use termion::{event::Key, input::TermRead, raw::IntoRawMode};

pub struct Editor {}

impl Default for Editor {
    fn default() -> Self {
        Self {}
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
                    _ => println!("{:?}\r", key),
                },
                Err(err) => Self::die(err),
            };
        }
    }

    fn handle_character(key: Key, c: char) {
        // Do NOT print ASCII control characters
        if c.is_control() {
            println!("{:?} \r", c as u8);
        } else {
            println!("{:?} ({})\r", key, c as u8);
        }
    }

    fn die(e: std::io::Error) {
        panic!("{e}");
    }
}