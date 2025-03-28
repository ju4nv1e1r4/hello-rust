use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut create_file = File::create("qwerty.txt")
        .expect("Cannot create file");

    create_file.write_all(b"QWERTY")
        .expect("Cannot write on file");
}
