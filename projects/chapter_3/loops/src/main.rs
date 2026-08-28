fn main() {
    let mut counter = 10;
    let result = 'counting_down: loop {
        if counter == 3 {
            break 'counting_down counter;
        } else {
            counter -= 1;
        }
    };
    println!("The result is {result}.");

    let a : [i32; 3] = [10,20,30];
    for element in a {
        println!("{element}");
    }
}

// adding labels is super useful when you have nested loops and you
// specifically want to break from one of the loops