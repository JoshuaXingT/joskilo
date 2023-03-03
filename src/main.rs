use std::io::{stdin, stdout, Read};

use termion::raw::IntoRawMode;

fn main() {
    let _stdout = stdout().into_raw_mode().unwrap();
    for b in stdin().bytes() {
        let b = b.unwrap();
        let c = b as char;
        if c.is_control() {
            println!("{:?}\r", b);
        } else {
            println!("{:?} ({})\r", b, c);
        }
        if c == 'q' {
            break;
        }
    }
}
