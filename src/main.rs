use std::io::{self, stdout, Read};
use termion::raw::IntoRawMode;

fn main() {
    let _stdout = stdout().into_raw_mode().unwrap();

    for byte in io::stdin().bytes() {
        let byte = byte.unwrap();
        let chara = byte as char;

        // Is an ASCII control character?
        if chara.is_control() {
            println!("{:?} \r", byte);
        } else {
            println!("{:?} ({})\r", byte, chara);
        }

        // Quit with ctrl+q
        if byte == to_ctrl_byte('q') {
            break;
        }
    }
}

// Convert the character to ctrl+character
fn to_ctrl_byte(c: char) -> u8 {
    let byte = c as u8;
    return byte & 0b0001_1111;
}
