use std::io::{self, stdout, Write};
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
        print!("{}{}", termion::clear::All, termion::cursor::Goto(1, 1));
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
    // Clear screen on panic
    print!("{}", termion::clear::All);
    panic!("{:?}", e);
}
