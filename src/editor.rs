use std::io::{self, stdout, Write}; // Importing the Write trait allows us to
                                    // use the flush method on stdout.
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

    pub fn run(&mut self) {
        let _stdout = stdout().into_raw_mode().unwrap();

        loop {
            // If there is an error clearing the screen, panic
            if let Err(error) = self.clear_screen() {
                die(error);
            }

            if self.should_quit {
                break;
            }

            if let Err(error) = self.process_keypress() {
                die(error);
            };
        }
    }

    fn clear_screen(&self) -> Result<(), std::io::Error> {
        // Use termion as an easy way to clear the screen over escape sequences
        print!("{}", termion::clear::All);

        // Return the flush method on stdout
        return io::stdout().flush();
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
