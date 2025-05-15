use std::io::{self, BufRead};
use anyhow::{Error, Ok, Result};

pub fn greeting_user() -> Result<String> {
    println!("Heey my fella! I am a simple Rust program created by some Rust student. Tell me your name:");
    let stdin = io::stdin();
    let mut buffer = String::new();
    stdin.lock().read_line(&mut buffer)?; 
    
    // NEW (for me, for you, for us): the "?" operator is used to propagate errors
    // More about the "?" operator: https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html#the-question-mark-operator

    if buffer.trim().to_lowercase() == "" {
        return Err(Error::msg("Okay cowboy! I see you don't want to tell me your name. Please, restart the program and help this student to learn Rust making his program works!"));
    } else {
        Ok("".to_string() + "Hello " + &buffer.trim() + "! I am a simple Rust program created by some Rust student. I am here to show how this student is evolvng in Rust (or almost). ")
    }
}