use std::io::{stdin, Read};

fn main() {
    for b in stdin().bytes() {
        let c = b.unwrap() as char;
        println!("{}", c);
    }
}
