use std::io::{Read, self, stdin, stdout};
use termion::raw::IntoRawMode;

fn to_ctrl_byte(c: char) -> u8 {
    let byte = c as u8;
    byte & 0b0001_1111
}

fn die(e: std::io::Error) {
    panic!(e);
}

fn main() {
    let _stdout = stdout().into_raw_mode().unwrap();

    for b in stdin().bytes() {
        match b {
            Ok(b) => {
                let c = b as char;
                match c.is_control() {
                    true => println!("{:#b} \r", b),
                    false => println!("{:#b} ({})\r", b, c),
                }
                if b == to_ctrl_byte('c') {
                    break;
                }
            },
            Err(e) => die(e)
        }

    }
}
