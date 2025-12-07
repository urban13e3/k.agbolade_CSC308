use std::thread;
use std::sync::mpsc;

fn main(){
    let (tx,rx) = mpsc::channel();

    let tx1 = tx.clone();
    thread::spawn(move || {
        for i in 1..=5 {
            tx1.send(format!("T1: Message {}", i)).unwrap();
        }
    });

    let tx2 = tx.clone();
    thread::spawn(move || {
        for i in 1..=5 {
            tx2.send(format!("T2: Message {}", i)).unwrap();
        }
    });

    let tx3 = tx.clone();
    thread::spawn(move || {
        for i in 1..=5 {
            tx3.send(format!("T3: Message {}", i)).unwrap();
        }
    });

    for received in rx.iter().take(15) {
        println!("Main received: {}", received);
    }

    println!("Main thread: done!");
}
