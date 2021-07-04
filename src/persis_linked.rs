use std::rc::Rc;

pub struct List<T> {
    head: Link<T>,
}

type Link<T> = Option<Rc<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
}

#[allow(dead_code)]
impl<T> List<T> {
    fn new() -> Self {
        List { head: None, }
    }

    fn append(&self, elem: T) -> List<T> {
        List {
            head: Some(Rc::new(Node {
                elem: elem,
                // Option<T> impl the Clone trait, evaluating <T>.clone() when it's Some<T>
                // which is Rc<Node<T>> here.
                next: self.head.clone(),
            }))
        }
    }

    fn tail(&self) -> List<T> {
        List { head: self.head.as_ref().and_then(|node| {
            node.next.clone()
        })}
    }

    fn head(&self) -> Option<&T> {
        self.head.as_ref().map( |node| &node.elem )
    }

    fn iter(&self) -> Iter<T> {
        Iter { next: self.head.as_deref() }
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut head = self.head.take();
        while let Some(node) = head {
            if let Ok(mut node) = Rc::try_unwrap(node) {
                head = node.next.take();
            } else {
                break;
            }
        }
    }
}

pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_deref();
            &node.elem
        })
    }
}

#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn basics() {
        let list = List::new();
        assert_eq!(list.head(), None);
        
        let list = list.append(1).append(2).append(3);
        assert_eq!(list.head(), Some(&3));

        let list = list.tail();
        assert_eq!(list.head(), Some(&2));

        let list = list.tail();
        assert_eq!(list.head(), Some(&1));

        let list = list.tail();
        assert_eq!(list.head(), None);

        let list = list.tail();
        assert_eq!(list.head(), None);
    }

    #[test]
    fn advanced() {
        let list1 : List<i32> = List::new().append(1);
        let list2 = list1.append(2);
        let list3 = list1.append(3);

        assert_eq!(list2.head(),Some(&2));
        assert_eq!(list2.tail().head(),Some(&1));
        assert_eq!(list3.head(),Some(&3));
        assert_eq!(list3.tail().head(),Some(&1));
    }

    #[test]
    fn iter() {
        let list = List::new().append(1).append(2).append(3);

        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));
    }
}