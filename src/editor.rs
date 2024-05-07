use crate::Terminal;
use std::io;
use termion::event::Key;

pub struct Editor {
    terminal: Terminal,
    should_quit: bool,
}

#[allow(clippy::derivable_impls)]
impl Default for Editor {
    fn default() -> Self {
        Self {
            terminal: Terminal::default(),
            should_quit: false,
        }
    }
}

impl Editor {
    /// Run the editor environment.
    pub fn run(&mut self) {
        loop {
            if let Err(err) = self.refresh_screen() {
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

    ///
    ///
    /// # Errors
    ///
    /// It is considered an error if ``Terminal::flush()`` fails to flush
    /// stdout.
    fn refresh_screen(&self) -> Result<(), std::io::Error> {
        Terminal::clear_screen();
        Terminal::goto(0, 0);

        if self.should_quit {
            println!("Goodbye!\r");
            return Ok(());
        }

        self.draw_rows();
        Terminal::goto(0, 0);
        Terminal::flush()
    }

    /// Processes the keypresses input into the terminal.
    /// Manages things like keyboard commands.
    fn process_keypress(&mut self) -> Result<(), io::Error> {
        let pressed_key = Terminal::read_key()?;

        #[allow(clippy::single_match)]
        match pressed_key {
            Key::Ctrl('q') => self.should_quit = true,
            _ => (),
        }

        Ok(())
    }

    /// Draws rows of text into the terminal.
    fn draw_rows(&self) {
        for _ in 0..self.terminal.size().height {
            println!("~\r");
        }
    }

    /// Clears the screen and panics!
    fn die(e: &std::io::Error) {
        print!("{}", termion::clear::All);
        panic!("{e}");
    }
}
