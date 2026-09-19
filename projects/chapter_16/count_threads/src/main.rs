use std::{sync::Arc, sync::Mutex, thread};

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _i in 0..10 {
        let counter = Arc::clone(&counter);
        handles.push(thread::spawn(move || {
            let mut x = counter.lock().unwrap();
            *x += 1;
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Counter is {}.", *counter.lock().unwrap());
}
