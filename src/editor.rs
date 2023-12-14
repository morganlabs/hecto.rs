use std::io::{self, stdout};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

pub struct Editor {
    should_quit: bool,
}

impl Editor {
    pub fn new() -> Self {
        return Self { should_quit: false };
    }

    // Since we're mutating `self` here if Ctrl+q is pressed, `self` must be
    // mutable
    pub fn run(&mut self) {
        let _stdout = stdout().into_raw_mode().unwrap();

        loop {
            // If we recieve an Error from process_keypress, assign that value
            if let Err(error) = self.process_keypress() {
                die(error);
            };

            // If self.should_quit is true, the program has been told to quit
            // without using panic! or any other methods of quitting. This is
            // the "best" way to exit the program(?)
            if self.should_quit {
                break;
            }
        }
    }

    fn process_keypress(&mut self) -> Result<(), std::io::Error> {
        let key = self.read_key()?;

        match key {
            Key::Ctrl('q') => self.should_quit = true,
            _ => (),
        }

        return Ok(());
    }

    fn read_key(&self) -> Result<Key, std::io::Error> {
        loop {
            if let Some(key) = io::stdin().lock().keys().next() {
                return key;
            }
        }
    }
}

fn die(e: std::io::Error) {
    panic!("{:?}", e);
}
