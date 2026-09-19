enum List<T: std::fmt::Display> {
    Cons(T, Box<List<T>>),
    Nil,
}

impl<T: std::fmt::Display> List<T> {
    fn print_list(&self) {
        match self {
            List::Cons(el, x) => {
                println!("{}", el);
                List::print_list(x);
            }
            List::Nil => (),
        }
    }
}

use List::{Cons, Nil};

fn main() {
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    list.print_list();
}
