use std::ptr::NonNull;
use std::marker::PhantomData;
use std::mem;



pub struct LinkedList<T> {
  head: Option<NonNull<Node<T>>>,
  tail: Option<NonNull<Node<T>>>,
  len: usize,
  // acting as if LinkedList holds data Box<Node<T>>
  marker: PhantomData<Box<Node<T>>>,
}

struct Node<T> {
  next: Option<NonNull<Node<T>>>,
  prev: Option<NonNull<Node<T>>>,
  element: T,
}

#[allow(dead_code)]
impl<T> Node<T> {
  fn new(element: T) -> Self {
    Node { next: None, prev: None, element }
  }

  fn into_element(self: Box<Self>) -> T {
    self.element
  }
}

#[allow(dead_code)]
impl<T> LinkedList<T> {
  pub const fn new() -> Self {
    LinkedList { head: None, tail: None, len: 0, marker: PhantomData }
  }

  pub fn append(&mut self, other: &mut Self) {
    match self.tail {
      None => mem::swap(self, other),
      Some(mut tail) => {
        if let Some(mut other_head) = other.head.take() {
          unsafe {
            tail.as_mut().next = Some(other_head);
            other_head.as_mut().prev = Some(tail);
          }
          self.tail = other.tail.take();
          self.len += mem::replace(&mut other.len, 0);
        }
      }
    }
  }

  pub fn front(&self) -> Option<&T> {
    unsafe {
      let node = self.head.as_ref();
      node.map(|node| &node.as_ref().element )
    }
  }

  fn push_front_node(&mut self, mut node: Box<Node<T>>) {
    unsafe {
      node.next = self.head;
      node.prev = None;
      let node = Some(Box::leak(node).into());

      match self.head {
        None => self.tail = node,
        Some(head) => {
          (*head.as_ptr()).prev = node
        }
      }
    }
  }
}

#[cfg(test)]
mod test {
  // use super::*;

  #[test]
  pub fn basic() {
  }

}