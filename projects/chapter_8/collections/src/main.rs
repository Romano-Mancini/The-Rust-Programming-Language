fn main() {
    let mut v: Vec<i32> = Vec::new();
    let _v2 = vec![1, 2, 3];

    v.push(7);
    v.push(8);
    v.push(9);

    let _first_method = &v[0]; // panics if over the length
    let _second_method = v.get(0); // returns Option

    // returns (im)mutable references, thus change code accordingly
    for i in &v {
        println!("{i}");
    }

    for i in &mut v {
        *i += 50;
    }

    for i in &v {
        println!("{i}");
    }

    let mut s = String::from("Hello");
    s.push_str(" world");
    s.push('!');
    println!("{s}");

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s4 = format!("{s1}-{s2}-{s3}");

    println!("{s4}");

    let s5 = String::from("Здравствуйте");

    for c in s5.chars() {
        println!("{c}");
    }
    println!("{s5}");
}
