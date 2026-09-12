/*
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
*/

fn longest<'l1, 'l2, 'l3>(s1: &'l1 str, s2: &'l2 str) -> &'l3 str
where
    'l1: 'l3,
    'l2: 'l3,
{
    if s1.len() > s2.len() {
        s1
    } else {
        s2
    }
}

fn main() {
    let x = String::from("hello");

    let result;

    {
        let y = String::from("this is longer");
        result = longest(&x, &y);
    }

    println!("{}", result);
}
