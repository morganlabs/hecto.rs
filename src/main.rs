use std::io::{self, stdout, Read};

// This is imported from the Termion crate
// This allows us to enter Raw Mode in the terminal
use termion::raw::IntoRawMode;

fn main() {
    let _stdout = stdout().into_raw_mode().unwrap();

    for b in io::stdin().bytes() {
        let b = b.unwrap(); // Bytecode
        let c = b as char; // Convert bytecode to actual character

        // Test whether or not a character is a "Control Character" in ASCII
        // Control Characters are NON-PRINTABLE, and therefore shouldn't be
        // printed to the screen.
        // `\r` is known as a Carraige Return. This makes sure that the next line
        // is not indented strangely. Disabling this means that all lines are
        // indented more and more over time. It brings the cursor back to the
        // beginning of the line
        if c.is_control() {
            // In this case, we ONLY print the bytecode.
            // When using the `format!` macro, line println! does, "{}" indicates
            // a placeholder for a variable. Whereas {:?} indicates a placeholder
            // for a variable that does NOT implement the Display trait. This means
            // you can print ANYTHING.
            println!("{:?} \r", b);
        } else {
            println!("{:?} ({})\r", b, c);
        }

        if c == 'q' {
            break;
        }
    }
}
