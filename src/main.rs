// Import `self` and `Read` from the I/O "part" of the standard library
// `self` refers to `io` itself, and `Read` is the equivalent of `io::Read`
use std::io::{self, Read};

fn main() {
    // io::stdin() - Is the standard input
    // .bytes() - Gets the bytes of whatever is input. This is only possible to
    //            use when `Read` is in scope. This is because Read is a trait
    //            of io
    for b in io::stdin().bytes() {
        // b.unwrap() - !! DANGEROUS !! Unwraps the value so that it can be accessed
        // as char - Converts the bytes into a character, if possible
        let char = b.unwrap() as char;

        // println!() - A macro to print to the screen
        println!("{}", char);
    }
}

// Currently, the terminal is in what is known as "Cooked" or Canonical Mode
// This means that input is only sent to the program after "Enter" is pressed
// What we need is Raw mode.
// There are libraries to do this in Rust
