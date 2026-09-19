#![allow(unused)]

struct MyPointer {
    data: String,
}

impl Drop for MyPointer {
    fn drop(&mut self) {
        println!("Dropped!");
    }
}

fn main() {
    let x = MyPointer {
        data: String::from("Hello world"),
    };
}
