enum List<T: std::fmt::Display> {
    Cons(T, Rc<List<T>>),
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

use std::rc::Rc;
use List::{Cons, Nil};

fn main() {
    let a = Rc::new(Cons(1, Rc::new(Cons(2, Rc::new(Cons(3, Rc::new(Nil)))))));
    let b = Cons(10, Rc::clone(&a));
    let c = Cons(20, Rc::clone(&a));

    a.print_list();
    b.print_list();
    c.print_list();
}
