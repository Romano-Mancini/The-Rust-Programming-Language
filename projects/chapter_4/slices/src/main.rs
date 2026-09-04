fn main() {
    let full_name = "Romano Mancini";
    let name : &str = &full_name[..6];         // still makes full_name lose W and O
    let surname : &str = &full_name[7..];

    println!("{name}");
    println!("{surname}");

    // String literals are implemented as slices of the program's binary
    let s : &str = "Hello, world!";
    println!("{s}");

    let arr = [1,2,3];
    let slc : &[i32] = &arr[1..];
}

