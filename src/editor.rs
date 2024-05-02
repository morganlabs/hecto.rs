use std::io::{self, stdout, Write};
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
            if let Err(err) = Self::clear_screen() {
                Self::die(&err);
            }

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
            Key::Char(c) => println!("{c}\r"),
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

    fn clear_screen() -> Result<(), io::Error> {
        print!("{}{}", termion::clear::All, termion::cursor::Goto(1, 1));

        // Flush stdout
        // The stdout may buffer some values and not print them out directly
        // Flushing the stdout forces it to print all buffered values
        io::stdout().flush()
    }

    fn die(e: &std::io::Error) {
        panic!("{e}");
    }
}
