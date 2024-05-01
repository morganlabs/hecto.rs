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

        loop {
            if let Err(err) = Self::process_keypress() {
                Self::die(&err);
            }
        }
    }

    fn process_keypress() -> Result<(), io::Error> {
        let pressed_key = Self::read_key()?;
        match pressed_key {
            Key::Ctrl('q') => panic!("Program ended."),
            _ => Ok(()),
        }
    }

    fn read_key() -> Result<Key, std::io::Error> {
        loop {
            if let Some(key) = io::stdin().lock().keys().next() {
                return key;
            }
        }
    }

    fn die(e: &std::io::Error) {
        panic!("{e}");
    }
}
