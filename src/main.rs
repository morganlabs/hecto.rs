use std::io::{self, stdout, Read};
use termion::raw::IntoRawMode;

fn main() {
    // This is bound to a variable, and is not being used
    // That is because we are putting the terminal into raw mode
    // Due to how ownership in Rust works, if something is not owned,
    // it is removed. To keep the terminal in raw mode, this value must be
    // owned. This has the benefit of us not having to manually unset raw
    // mode when we leave the program!
    let _stdout = stdout().into_raw_mode().unwrap();

    for b in io::stdin().bytes() {
        let b = b.unwrap();
        let c = b as char;

        // "control" characters are ASCII characters that are non-printable.
        if c.is_control() {
            println!("{:?} \r", b);
        } else {
            println!("{:?} ({})\r", b, c);
        }

        if c == 'q' {
            break;
        }
    }
}
