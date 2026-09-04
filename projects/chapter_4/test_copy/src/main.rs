fn main() {
    let a : i32 = 10;
    let b = a;          // this is a copy: i32 is not on heap
    println!("{a}");
    println!("{b}");

    /*
     */
    let s1 : String = String::from("Hello World");
    let s2 = s1;        // this is a move: string is on heap
    println!("{s1}");
    println!("{s2}");
}
