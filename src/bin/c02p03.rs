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
            return Node::tail(&cur);
        }
        Some(node.clone())
    }
}

impl<T> LinkedList<T>
where
    T: std::cmp::Eq + std::hash::Hash + std::clone::Clone,
{
    fn new() -> Self {
        Self { head: None }
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
                return Some(cur);
            } else {
                return Node::tail(&cur);
            }
        }
        None
    }

    fn remove(&mut self, node_to_remove: &NodeRef<T>) {
        for node in self.iter() {
            let mut borrowed_node = node.borrow_mut();
            if let Some(next) = borrowed_node.next.as_ref().cloned() {
                if Rc::ptr_eq(&next, node_to_remove) {
                    borrowed_node.next = node_to_remove.borrow_mut().next.take()
                }
            }
        }
    }

    fn iter(&self) -> Iter<T> {
        Iter {
            next: self.head.as_ref().cloned(),
        }
    }
}

impl<T> Iterator for Iter<T> {
    type Item = NodeRef<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(cur) = self.next.as_ref().cloned() {
            self.next = cur.borrow().next.clone();
            return Some(cur);
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

fn main() {
    let mut list = LinkedList::<String>::new();
    list.append(String::from("item1"));
    list.append(String::from("item2"));
    for node in list.iter() {
        list.remove(&node.clone());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remove_node() {
        let mut list1 = LinkedList::<String>::new();
        list1.append(String::from("item1"));
        list1.append(String::from("item2"));
        list1.append(String::from("item3"));
        list1.append(String::from("item4"));
        list1.append(String::from("item5"));

        for (n, node) in list1.iter().enumerate() {
            if n == 3 {
                let to_remove = Some(node.clone());
                list1.remove(to_remove.as_ref().unwrap());
            }
        }

        for node in list1.iter() {
            assert_ne!(node.borrow().data, "item4");
        }
    }
}
