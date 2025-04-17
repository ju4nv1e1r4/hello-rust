use std::thread;
mod hello_from_thread;
use hello_from_thread::hello_thread;
mod channels;
use channels::coms;
use channels::counter;
use channels::duration_example;

fn main() {
    let handle1 = thread::spawn(|| {
        hello_thread(1);
    });

    let handle2 = thread::spawn(|| {
        hello_thread(2);
    });

    handle1.join().unwrap();
    handle2.join().unwrap();

    println!("Hello from main thread");

    // Using catch to handle with errors on threads
    let result = thread::spawn(|| {
        // Simulating a panic
        let _ = Err::<&str, &str>("something went wrong");
    }).join();

    match result {
        Ok(value) => println!("Thread completed successfully: {:?}", value),
        Err(error) => println!("Thread panicked with error: {:?}", error),
    }

    // Thread triggering
    let result1 = thread::spawn(|| {
        42
    }).join().unwrap();

    let result2 = thread::spawn(move || {
        result1 * 2
    }).join().unwrap();

    println!("Result from thread 2: {:?}", result2);

    // Communication with channels
    coms();

    // Counter example
    counter();
    
    // Duration example
    duration_example();
}
