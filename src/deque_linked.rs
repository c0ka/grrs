use std::rc::Rc;
use std::cell::RefCell;

pub struct List<T> {
    head: Link<T>,
    tail: Link<T>,
}

type Link<T> = Option<Rc<RefCell<Node<T>>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
    prev: Link<T>,
}

fn compare() {
    let x = 5;
    let y = Box::new(x);

    assert_eq!(5, x);
    assert_eq!(*y, 5);
}