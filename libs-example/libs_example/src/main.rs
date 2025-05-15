use anyhow::Error;
use first_example::greeting_user;

fn main() {
    let result: Result<String, Error> = greeting_user();
    match result {
        Ok(greeting) => println!("{}", greeting),
        Err(e) => eprintln!("Error: {}", e),
    }
}
