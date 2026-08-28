fn main() {
    let x1 = convert_to_celsius(90);
    let x2 = convert_to_farheneit(30);
    let x3 = fibonacci(10);

    println!("x1 = {x1}");
    println!("x2 = {x2}");
    println!("x3 = {x3}");
}

fn convert_to_celsius(value: i32) -> i32 {
    (value - 32) * 5/9
}

fn convert_to_farheneit(value: i32) -> i32 {
    (value * 5/9) + 32
}

fn fibonacci(index : i32) -> i32 {
    if index <= 1 { index } else { fibonacci(index-1) + fibonacci(index-2) }
}