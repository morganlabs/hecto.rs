use std::io::{self, stdout, Write};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::{IntoRawMode, RawTerminal};

pub struct Terminal {
    _stdout: RawTerminal<std::io::Stdout>,
    size: Size,
}

pub struct Size {
    pub width: u16,
    pub height: u16,
}

impl Default for Terminal {
    fn default() -> Self {
        let size = termion::terminal_size().expect("Failed to get terminal size.");

        Self {
            _stdout: stdout().into_raw_mode().expect("Failed to enter raw mode."),
            size: Size {
                width: size.0,
                height: size.1,
            },
        }
    }
}

impl Terminal {
    #[must_use]
    /// Returns the terminal's size.
    pub fn size(&self) -> &Size {
        &self.size
    }

    /// Clears the terminal's screen
    ///
    /// # Errors
    ///
    /// It is considered an error if ``Terminal::flush()`` fails to flush
    /// stdout.
    pub fn clear_screen() {
        print!("{}", termion::clear::All);
    }

    /// Flushes the stdout.
    ///
    /// # Errors
    ///
    /// It is considered an error if ``Terminal::flush()`` fails to flush
    /// stdout.
    pub fn flush() -> Result<(), std::io::Error> {
        io::stdout().flush()
    }

    /// Sets the cursor's position in the terminal.
    /// Must be within the bounds of the terminal, of course!
    pub fn goto(x: u16, y: u16) {
        let x = x.saturating_add(1);
        let y = y.saturating_add(1);
        print!("{}", termion::cursor::Goto(x, y));
    }

    /// A continuous loop. If a key is pressed and a value is returned, the key
    /// is returned
    ///
    /// # Errors
    ///
    /// It fails to get a key? Idk man...
    pub fn read_key() -> Result<Key, std::io::Error> {
        loop {
            if let Some(key) = io::stdin().lock().keys().next() {
                return key;
            }
        }
    }
}
