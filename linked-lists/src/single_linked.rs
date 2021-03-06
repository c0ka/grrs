use std::mem;


/// One-direction linked stack with a head attribute pointing to it's latest aka top node.
pub struct List<T> {
    head: Link<T>,
}

/// Pointer to the next node or a None value.
/// This is a Box type pointer, so it can be fixed length.
type Link<T> = Option<Box<Node<T>>>;

/// Node of stack, including a element and a pointer to the previous node.
struct Node<T> {
    elem: T,
    next: Link<T>,
}

#[allow(dead_code)]
impl<T> List<T> {
    /// Construct a List without node attached, so the head pointer is None.
    pub fn new() -> Self {
        List { head: None }
    }

    pub fn peek(&self) -> Option<&T> {
        // create a new Option<&Box<Node>>, and map the it's content which is &Box<Node>.
        // Notice the auto deref when evaluating the node's elem.
        self.head.as_ref().map(|node| {
            &node.elem
        })
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| {
            &mut node.elem
        })
    }    

    /// Create a new node with its next attr pointing to the list's head's value.
    /// Change the list's head pointing to the new created node.
    pub fn push(&mut self, elem: T) {
        let new_node = Node {
            elem: elem, 
            // Takes the option value(Link<T>) out, leaving a None in its place.
            next: self.head.take(),
        };
        self.head = Some(Box::new(new_node));
    }

    pub fn pop(&mut self) -> Option<T> {
        // node.next is moved here
        self.head.take().map(|node| {
            self.head = node.next;
            node.elem
        })
    }

    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }

    pub fn iter(&self) -> Iter<T> {
        // head is an Option wrap the Box pointing to Node, as_deref() will deref the Option's content
        // that's deref the Box and return &Node.
        Iter { next: self.head.as_deref() }
    }

    pub fn iter_mut(&mut self) -> IterMut<T> {
        IterMut { next: self.head.as_deref_mut() }
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut cur_link = self.head.take();

        while let Some(mut boxed_node) = cur_link {
            cur_link = mem::replace(&mut boxed_node.next, None);
        }
    }
}

pub struct IntoIter<T>(List<T>);

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item>{
        self.0.pop()
    }
}

/// Iter of the stack holding a "next" attribute pointing to the next value should be returned.
pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            // the alternative:
            // self.next = node.next.as_ref().map::<&Node<T>, _>(|node| &node);
            self.next = node.next.as_deref();
            &node.elem
        })
    }
}

pub struct IterMut<'a, T> {
    next: Option<&'a mut Node<T>>,
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        // take out the Option<&mut Node<T>>
        self.next.take().map(|node| {
            self.next = node.next.as_deref_mut();
            &mut node.elem
        })
    }
}


#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn basics() {
        let mut list = List::new();

        assert_eq!(list.pop(), None);
        assert_eq!(list.peek(), None);

        list.push(1);
        list.push(2);
        list.push(3);
        
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));

        list.push(4);
        list.push(5);
        
        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), Some(4));
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);       
    }

    #[test]
    fn peek() {
        let mut list = List::new();
        assert_eq!(list.peek(), None);
        assert_eq!(list.peek_mut(), None);
        list.push(1); list.push(2); list.push(3);

        assert_eq!(list.peek(), Some(&3));
        assert_eq!(list.peek_mut(), Some(&mut 3));

        list.peek_mut().map(|elem| {
            *elem = 42
        });

        assert_eq!(list.peek(), Some(&42));
        assert_eq!(list.pop(), Some(42));
    }

    #[test]
    fn into_iter() {
        let mut list = List::new();   
        list.push(1); list.push(2); list.push(3);

        let mut iter = list.into_iter();
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn iter() {
        let mut list = List::new();
        list.push(1); list.push(2); list.push(3);

        let mut iter1 = list.iter();
        let mut iter2 = list.iter();
        assert_eq!(iter1.next(), Some(&3));
        assert_eq!(iter2.next(), Some(&3));
        assert_eq!(iter1.next(), Some(&2));
        assert_eq!(iter1.next(), Some(&1));
        assert_eq!(iter2.next(), Some(&2));
        assert_eq!(iter2.next(), Some(&1));
    }

    #[test]
    fn iter_mut() {
        let mut list = List::new();
        list.push(1); list.push(2); list.push(3);

        let mut iter_mut1 = list.iter_mut();
        assert_eq!(iter_mut1.next(), Some(&mut 3));
        assert_eq!(iter_mut1.next(), Some(&mut 2));
        assert_eq!(iter_mut1.next(), Some(&mut 1));
    }

}

