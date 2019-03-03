use std::cell::RefCell;
use std::fmt::Display;
use std::rc::Rc;

type NodeRef<T> = Rc<RefCell<Node<T>>>;
type DataRef<T> = Rc<RefCell<T>>;

struct LinkedList<T> {
    head: Option<NodeRef<T>>,
}

struct Node<T> {
    data: DataRef<T>,
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

    fn append(&mut self, new_value: DataRef<T>) {
        if let Some(tail) = self.tail() {
            tail.borrow_mut().next = Some(Rc::new(RefCell::new(Node {
                data: new_value.clone(),
                next: None,
            })));
        } else {
            self.head = Some(Rc::new(RefCell::new(Node {
                data: new_value.clone(),
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

    fn lists_intersect(&self, other: &Self) -> bool {
        for self_node in self.iter() {
            for other_node in other.iter() {
                if Rc::ptr_eq(&self_node.borrow().data, &other_node.borrow().data) {
                    return true;
                }
            }
        }
        false
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
            write!(w, "{}", n.borrow().data.borrow())?;
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
    fn test_intersection() {
        let datavec = vec![
            Rc::new(RefCell::new(1)),
            Rc::new(RefCell::new(2)),
            Rc::new(RefCell::new(3)),
            Rc::new(RefCell::new(4)),
            Rc::new(RefCell::new(5)),
        ];

        let mut intersecting_first = LinkedList::<i32>::new();
        for value in datavec.iter().take(3) {
            intersecting_first.append(value.clone());
        }

        let mut intersecting_second = LinkedList::<i32>::new();
        for value in datavec.iter().take(5).skip(2) {
            intersecting_second.append(value.clone());
        }

        assert_eq!(
            intersecting_first.lists_intersect(&intersecting_second),
            true
        );

        let mut nonintersecting_first = LinkedList::<i32>::new();
        for value in datavec.iter().take(3) {
            nonintersecting_first.append(value.clone());
        }

        let mut nonintersecting_second = LinkedList::<i32>::new();
        for value in datavec.iter().take(2).skip(3) {
            nonintersecting_second.append(value.clone());
        }

        assert_eq!(
            nonintersecting_first.lists_intersect(&nonintersecting_second),
            false
        );
    }

}

fn main() {
    let mut left = LinkedList::<i32>::new();
    let right = LinkedList::<i32>::new();
    left.append(Rc::new(RefCell::new(6)));
    left.lists_intersect(&right);
}
