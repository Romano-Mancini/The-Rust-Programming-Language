fn main() {
    println!("Hello, world!");
    print_measurement(10, 'C');
    
    let x = plus_one(return_five());
    println!("The returned value is {x}");
}

fn print_measurement(value: i32, unit_label: char) {
    println!("The obtained measurement is of {value} {unit_label}.")
}

fn return_five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

// better to use the idiomatic style of returning the last expression
//fn return_five_2() -> i32 {
//    return 5
//}