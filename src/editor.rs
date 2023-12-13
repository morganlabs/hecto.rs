use std::io::{self, stdout};
use termion::event::Key;
use termion::input::TermRead; // This allows us to do `io::stdin().keys()`
use termion::raw::IntoRawMode;

// The `pub` keyword allows us to access things from other files when imported

// A struct is a collection of variables, and functions can be added with
// `impl`s (below).
pub struct Editor {}

// This allows us to add functions (methods) to structs.
impl Editor {
    // This is a static function
    pub fn new() -> Self {
        return Self {};
    }

    pub fn run(&self) {
        let _stdout = stdout().into_raw_mode().unwrap();

        // This loops forever until explicitly exited with `break` or `return`.
        loop {
            // if let essentially a single match statement arm. We can assign
            // variables to the result of a match arm.
            // The variable is only available within the scope of the match arm.
            // If we recieve an Error from process_keypress, assign that value
            // to `error` and run the code within the block
            if let Err(error) = self.process_keypress() {
                die(error);
            };
        }
    }

    fn process_keypress(&self) -> Result<(), std::io::Error> {
        // The ? means to pass up the error (if there is one) to a higher level
        // This is the same as writing a match statement that returns Err(e)
        // if an error arises
        // This allows the calling function to handle the error as it sees fit
        let key = self.read_key()?;

        // Unlike `if let`, here we must match every possible arm of the match
        // statement. We will populate this with more key combinations
        match key {
            Key::Ctrl('q') => panic!("Exiting..."),
            _ => (),
        }

        return Ok(());
    }

    fn read_key(&self) -> Result<Key, std::io::Error> {
        // This function loops until a return is reached and breaks the loop
        // If the function doesnt initially return a key, it will loop until
        // a key is returned
        loop {
            // If io::stdin().lock().keys() returns any value, return it
            if let Some(key) = io::stdin().lock().keys().next() {
                return key;
            }
        }
    }
}

fn die(e: std::io::Error) {
    panic!("{:?}", e);
}
