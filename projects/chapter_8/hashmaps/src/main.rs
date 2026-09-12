use std::collections::HashMap;

fn main() {
    let mut hm = HashMap::new();

    hm.insert(String::from("team1"), 10);
    hm.insert(String::from("team2"), 20);

    let team_name = String::from("team1");
    let score = hm.get(&team_name).copied().unwrap_or(0);

    scores.entry(String::from("Yellow")).or_insert(50); // check if key yellow has value else put yellow -> 50
    for (key, value) in &hm {
        println!("{key}: {value}");
    }
}
