use std::cell::RefCell;
use std::fmt::Display;
use std::rc::Rc;

type NodeRef<T> = Rc<RefCell<Node<T>>>;

struct LinkedList<T> {
    head: Option<NodeRef<T>>,
    tail: Option<NodeRef<T>>,
}

struct Node<T> {
    data: T,
    next: Option<NodeRef<T>>,
    prev: Option<NodeRef<T>>,
}

struct Iter<T> {
    next: Option<NodeRef<T>>,
    prev: Option<NodeRef<T>>,
}

impl<T> LinkedList<T>
where
    T: std::cmp::Eq,
    T: std::hash::Hash,
    T: std::clone::Clone,
    T: std::cmp::PartialOrd,
    T: std::fmt::Debug,
{
    fn new() -> Self {
        Self {
            head: None,
            tail: None,
        }
    }

    fn append(&mut self, new_value: T) {
        if let Some(tail) = self.tail.as_ref().cloned() {
            let new_node = Rc::new(RefCell::new(Node {
                data: new_value,
                next: None,
                prev: Some(tail.clone()),
            }));
            tail.borrow_mut().next = Some(new_node.clone());
            self.tail = Some(new_node.clone());
        } else {
            self.head = Some(Rc::new(RefCell::new(Node {
                data: new_value,
                next: None,
                prev: None,
            })));
            self.tail = self.head.as_ref().cloned();
        }
    }

    fn iter(&self) -> Iter<T> {
        Iter {
            next: self.head.as_ref().cloned(),
            prev: self.tail.as_ref().cloned(),
        }
    }

    fn is_palindrome(&self) -> bool {
        let mut iter = self.iter();
        let mut front = iter.next();
        let mut back = iter.next_back();
        while front.is_some() && back.is_some() {
            if front.unwrap().borrow().data != back.unwrap().borrow().data {
                return false;
            }
            front = iter.next();
            back = iter.next_back();
        }
        true
    }
}

impl<'a, T> Iterator for Iter<T> {
    type Item = NodeRef<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(cur) = self.next.as_ref().cloned() {
            self.next = cur.borrow().next.clone();
            if Rc::ptr_eq(&cur, self.prev.as_ref().unwrap()) {
                return None;
            }
            return Some(cur.clone());
        }
        None
    }
}

impl<'a, T> DoubleEndedIterator for Iter<T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if let Some(cur) = self.prev.as_ref().cloned() {
            self.prev = cur.borrow().prev.clone();
            if Rc::ptr_eq(self.next.as_ref().unwrap(), &cur) {
                return None;
            }
            return Some(cur.clone());
        }
        None
    }
}

impl<T: Display> Display for LinkedList<T> {
    fn fmt(&self, w: &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
        write!(w, "[")?;
        let mut node = self.head.clone();
        while let Some(n) = node {
            write!(w, "{}", n.borrow().data)?;
            node = n.borrow().next.clone();
            if node.is_some() {
                write!(w, ", ")?;
            }
        }
        write!(w, "]")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_iterator_forward_reverse() {
        let vec_forward: Vec<i32> = vec![0, 1, 2, 3, 4];
        let vec_reverse: Vec<i32> = vec![4, 3, 2, 1, 0];

        let mut list_forward = LinkedList::<i32>::new();
        let mut list_reverse = LinkedList::<i32>::new();

        for value in vec_forward.iter() {
            list_forward.append(*value);
        }
        for value in vec_reverse.iter() {
            list_reverse.append(*value);
        }

        for (i, value) in list_forward.iter().enumerate() {
            assert_eq!(vec_forward[i], value.borrow().data);
        }
        for (i, value) in list_reverse.iter().enumerate() {
            assert_eq!(vec_reverse[i], value.borrow().data);
        }

        for (i, value) in list_forward.iter().rev().enumerate() {
            assert_eq!(vec_reverse[i], value.borrow().data);
        }
        for (i, value) in list_reverse.iter().rev().enumerate() {
            assert_eq!(vec_forward[i], value.borrow().data);
        }
    }
    #[test]
    fn test_is_palindrome() {
        let mut list1 = LinkedList::<i32>::new();
        list1.append(1);
        list1.append(2);
        list1.append(3);
        list1.append(2);
        list1.append(1);
        assert_eq!(list1.is_palindrome(), true);

        let mut list2 = LinkedList::<i32>::new();
        list2.append(1);
        list2.append(2);
        list2.append(3);
        list2.append(2);
        list2.append(1);
        list2.append(1);
        assert_eq!(list2.is_palindrome(), false);

        let mut list3 = LinkedList::<i32>::new();
        list3.append(1);
        list3.append(2);
        list3.append(2);
        list3.append(1);
        assert_eq!(list3.is_palindrome(), true);

        let mut list4 = LinkedList::<i32>::new();
        list4.append(1);
        list4.append(1);
        list4.append(2);
        list4.append(2);
        assert_eq!(list4.is_palindrome(), false);
    }

}

fn main() {
    let mut list = LinkedList::<i32>::new();
    list.append(6);
    list.is_palindrome();
}
