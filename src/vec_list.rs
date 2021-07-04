use std::collections::LinkedList;

pub struct MyVec<T> {
  buf: Node<T>,
  len: usize,
}


struct Node<T> {
  ptr: Box<T>,
  cap: usize,
}