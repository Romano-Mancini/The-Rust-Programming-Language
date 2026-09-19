use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();
    let tx2 = tx.clone();

    thread::spawn(move || {
        let strings = vec![
            String::from("Hello!"),
            String::from("This"),
            String::from("is"),
            String::from("another"),
            String::from("thread!"),
        ];
        for s in strings {
            tx.send(s).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let strings = vec![
            String::from("Hello! 1"),
            String::from("This 1"),
            String::from("is 1"),
            String::from("another 1"),
            String::from("thread! 1"),
        ];
        for s in strings {
            tx2.send(s).unwrap();
            thread::sleep(Duration::from_millis(500));
        }
    });

    // will go on until each tx is dropped, closing the channel.
    for s in rx {
        println!("{s}");
    }
}
