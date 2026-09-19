struct Node {
    value: i32,
    children: RefCell<Vec<Rc<Node>>>,
    parents: RefCell<Vec<Rc<Weak<Node>>>>,
}

fn main() {}
