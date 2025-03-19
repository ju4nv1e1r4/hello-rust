fn main() {
    let mut my_string = String::from("Hello, World! I'm learning rust.");
    
    println!("This is the content from my_string: {}", my_string);
    println!("This is the length of the content from my_string: {}", my_string.len());
    for token in my_string.split_whitespace() {
        println!("{}", token)
    };
    println!("Is my_string empty?\nA: {}", my_string.is_empty());
    println!("It contains 'rust'?\nA: {}", my_string.contains("rust"));
    
    my_string.push_str("\nWhat's Next?");
    println!("{}", my_string);
}

/*
Output>>

    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.01s
     Running `target/debug/strings`
This is the content from my_string: Hello, World! I'm learning rust.
This is the length of the content from my_string: 32
Hello,
World!
I'm
learning
rust.
Is my_string empty?
A: false
It contains 'rust'?
A: true
Hello, World! I'm learning rust.
What's Next?

*/
