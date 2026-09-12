fn find_greatest<T>(arr: &[T]) -> T {
    let mut greatest = &arr[0];

    for i in arr {
        if i > greatest {
            greatest = i;
        }
    }

    greatest
}

#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn new(x: T, y: T) -> Self {
        Point { x: x, y: y }
    }
}

fn main() {
    let vector: Vec<i32> = vec![10, 20, 30];
    println!("{}", find_greatest(&vector));

    let point = Point::new(10, 20);
    println!("{point:?}");
}
