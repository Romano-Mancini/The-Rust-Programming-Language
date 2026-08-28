const SECONDS_IN_AN_HOUR: u32 = 60*60;

fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    println!("In an hour there are {SECONDS_IN_AN_HOUR} seconds.");

    let tup : (i32, f64, isize) = (10, 0.1, 300);

    // two ways of deconstructing: pattern matching or with a period.
    let (x,y,z) = tup;
    x1 = tup.0;
    y1 = tup.1;
    z1 = tup.2;

    // array type annotations are written like this, with the type of the elements followed by the length of the array.
    let arr : [i32, 5] = [1,2,3,4,5]
    let first = arr[0];
    let second = arr[1];
    // equivalent to [5,5,5], just more concise syntax for long repeating arrays.
    let arr2 = [5; 3]
}