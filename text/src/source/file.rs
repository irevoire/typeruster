use std::fs::read_to_string;
use std::io::{stdin, Read};

pub fn from(filename: &str) -> crate::Text {
    let mut s = String::new();
    if filename == "-" {
        stdin().read_to_string(&mut s).expect("Can’t read on stdin");
    } else {
        s = read_to_string(filename).expect("can’t read the provided file");
    }
    crate::Text::new(s, String::from(filename))
}
