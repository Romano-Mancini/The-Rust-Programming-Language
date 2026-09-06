// you should write Option::Some() and Option::None but since it's in standard library we can avoid it

// documentation to make it better is at https://doc.rust-lang.org/std/option/enum.Option.html
fn main() {
    let some_number: Option<i32> = Some(10);
    let nothing: Option<i32> = None;
}
