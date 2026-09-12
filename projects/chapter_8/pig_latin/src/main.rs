fn to_pig_latin(s: &String) -> String {
    if s.len() == 0 {
        return String::new();
    }

    let start: char = s.chars().next().unwrap();
    if start == 'a' || start == 'e' || start == 'i' || start == 'o' || start == 'u' {
        let clone = s.clone();
        return format!("{clone}-hay");
    } else {
        let to_return = s[1..].to_string();
        return format!("{to_return}-{start}ay");
    }
}

fn main() {
    let s1 = String::from("first");
    let s2 = String::from("apple");

    let r1 = to_pig_latin(&s1);
    let r2 = to_pig_latin(&s2);

    println!("{r1}");
    println!("{r2}");
}
