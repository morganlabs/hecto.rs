use std::io::{self, stdout};
use termion::{event::Key, input::TermRead, raw::IntoRawMode};

pub struct Editor {
    should_quit: bool,
}

impl Default for Editor {
    fn default() -> Self {
        Self { should_quit: false }
    }
}

impl Editor {
    pub fn run(&mut self) {
        // Enter terminal "raw" mode
        let _stdout = stdout().into_raw_mode().unwrap();

        loop {
            if self.should_quit {
                break;
            }

            if let Err(err) = self.process_keypress() {
                Self::die(&err);
            }
        }
    }

    fn process_keypress(&mut self) -> Result<(), io::Error> {
        let pressed_key = Self::read_key()?;
        match pressed_key {
            Key::Ctrl('q') => self.should_quit = true,
            _ => (),
        }

        Ok(())
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
