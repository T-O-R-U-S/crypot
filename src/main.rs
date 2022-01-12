use std::fmt::format;
use std::fs::read;
use std::io::{stdin, Write};

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let input = format!("{:.12}", input);
    println!("{}", input);
}
