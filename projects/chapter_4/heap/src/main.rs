fn main() {
    let str1 : String = String::from("Hello");
    let str2 : String = String::from("World");
    format_print(&str1, &str2);
}

fn format_print(str1: &String, str2: &String) {
    println!("{} {}!", str1, str2);
}

fn ascii_capitalize(v: &mut Vec<char>) {
    let c = &v[0];
    if c.is_ascii_lowercase() {
        let up = c.to_ascii_uppercase();
        v[0] = up;
    } else {
        println!("Already capitalized: {:?}", v);
    }
}