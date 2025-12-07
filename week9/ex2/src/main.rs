use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    // Shared counter wrapped in Arc + Mutex
    let counter = Arc::new(Mutex::new(0));

    let mut handles = vec![];

    for _ in 0..10 {
        let counter_clone = Arc::clone(&counter);

        let handle = thread::spawn(move || {
            let mut num = counter_clone.lock().unwrap();
            *num += 3; // Each thread adds 3
        });

        handles.push(handle);
    }


    for h in handles {
        h.join().unwrap();
    }

    println!("Final counter = {}", *counter.lock().unwrap()); 
}