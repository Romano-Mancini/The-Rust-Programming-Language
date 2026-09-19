use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
        for i in 10..20 {
            println! {"Number {i} from spawned thread"};
        }
    });

    for i in 1..9 {
        println! {"Number {i} from main thread"};
    }

    handle.join().unwrap();
}
