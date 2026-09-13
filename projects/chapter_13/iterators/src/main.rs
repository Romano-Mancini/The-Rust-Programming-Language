fn main() {
    let vector: Vec<i32> = vec![1, 2, 3];
    let vector2: Vec<i32> = vector.iter().map(|x| x + 1).collect();

    println!("{vector:#?}");
    println!("{vector2:#?}");
}
