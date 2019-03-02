use std::cell::RefCell;
use std::fmt::Display;
use std::rc::Rc;

type NodeRef<T> = Rc<RefCell<Node<T>>>;

struct LinkedList<T> {
    head: Option<NodeRef<T>>,
}

struct Node<T> {
    data: T,
    next: Option<NodeRef<T>>,
}

struct Iter<T> {
    next: Option<NodeRef<T>>,
}

impl<T> Node<T> {
    fn tail(node: &NodeRef<T>) -> Option<NodeRef<T>> {
        if let Some(cur) = node.borrow().next.as_ref().cloned() {
            return Node::tail(&cur.clone());
        }
        Some(node.clone())
    }
}

impl<T> LinkedList<T>
where
    T: std::cmp::Eq,
    T: std::hash::Hash,
    T: std::clone::Clone,
    T: std::cmp::PartialOrd,
{
    fn new() -> Self {
        Self { head: None }
    }

    fn partition(&self, partition_value: T) -> LinkedList<T> {
        let mut new_list = LinkedList::new();
        let mut tmp_list: Vec<T> = vec![];
        for node in self.iter() {
            if node.borrow().data < partition_value {
                new_list.append(node.borrow().data.clone());
            } else {
                tmp_list.push(node.borrow().data.clone());
            }
        }
        for value in tmp_list.drain(..) {
            new_list.append(value);
        }
        new_list
    }

    fn append(&mut self, new_value: T) {
        if let Some(tail) = self.tail() {
            tail.borrow_mut().next = Some(Rc::new(RefCell::new(Node {
                data: new_value,
                next: None,
            })));
        } else {
            self.head = Some(Rc::new(RefCell::new(Node {
                data: new_value,
                next: None,
            })));
        }
    }

    fn tail(&self) -> Option<NodeRef<T>> {
        if let Some(cur) = self.head.as_ref().cloned() {
            if cur.borrow().next.is_none() {
                return Some(cur.clone());
            } else {
                return Node::tail(&cur.clone());
            }
        }
        None
    }

    fn iter(&self) -> Iter<T> {
        Iter {
            next: self.head.as_ref().cloned(),
        }
    }
}

impl<'a, T> Iterator for Iter<T> {
    type Item = NodeRef<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(cur) = self.next.as_ref().cloned() {
            self.next = cur.borrow().next.clone();
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
    fn test_partition() {
        let mut list1 = LinkedList::<i32>::new();
        list1.append(3);
        list1.append(5);
        list1.append(8);
        list1.append(5);
        list1.append(10);
        list1.append(2);
        list1.append(1);

        let list_partitioned = list1.partition(5);
        let mut iter = list_partitioned.iter();
        assert_eq!(iter.next().unwrap().borrow().data, 3);
        assert_eq!(iter.next().unwrap().borrow().data, 2);
        assert_eq!(iter.next().unwrap().borrow().data, 1);
        assert_eq!(iter.next().unwrap().borrow().data, 5);
        assert_eq!(iter.next().unwrap().borrow().data, 8);
        assert_eq!(iter.next().unwrap().borrow().data, 5);
        assert_eq!(iter.next().unwrap().borrow().data, 10);
    }
}

fn main() {
    let mut list = LinkedList::<String>::new();
    list.append(String::from("item1"));
    list.append(String::from("item2"));
    let _list_partitioned = list.partition("hi".to_string());
}
