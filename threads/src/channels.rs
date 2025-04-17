use std::thread;
use std::sync::{mpsc, Arc, Mutex};
use std::time::Duration;

pub fn coms() {
    //1st example
    // create a communication channel
    let(tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let msg = String::from("Hello from thread 1");
        tx.send(msg).unwrap();
    });

    let received = rx.recv().unwrap();
    println!("Received: {}", received);
}

pub fn counter() {
    let c: Arc<Mutex<i32>> = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    
    for _ in 0..10 {
        let c: Arc<Mutex<i32>> = Arc::clone(&c);

        let handle = thread::spawn(move || {
            let mut num = c.lock().unwrap();
            *num += 1;
        });
    
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *c.lock().unwrap());
}

pub fn duration_example() {
    let handles: Vec<_> = (0..5).map(|i|{
        thread::spawn(move ||{
            thread::sleep(Duration::from_secs(i as u64));
            println!("Thread {} finished", i);
            i*i
        })
    }).collect();

    let results: Vec<_> = handles.into_iter().map(|handle|
        handle.join().unwrap()
    ).collect();

    println!("Results: {:?}", results);
}