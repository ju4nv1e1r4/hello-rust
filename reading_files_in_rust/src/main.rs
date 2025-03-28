use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut my_file = File::open("mlops_maturity_by_google.txt")
    .expect("Cannot open file");

    let mut content_file = String::new();

    my_file.read_to_string(&mut content_file)
    .expect("Cannot read file");

    println!("Content:\n {content_file}")
}
