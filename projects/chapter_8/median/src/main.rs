use std::collections::HashMap;

fn main() {
    let mut v: Vec<i32> = vec![42, 7, 91, 18, 63, 35, 84, 12, 56, 29, 42];
    v.sort();

    let median = v[v.len() / 2];
    let mut counter = HashMap::new();

    let mut max = 0;
    let mut mode = -1;

    for i in &v {
        let found = counter.entry(i).or_insert(0);
        *found += 1;

        if *found > max {
            max = *found;
            mode = *i;
        }
    }

    println!("Median is: {median}");
    println!("Mode is: {mode}");
}
