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
    T: std::cmp::Eq
        + std::hash::Hash
        + std::clone::Clone
        + std::cmp::PartialOrd
        + std::cmp::PartialEq,
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

    fn iter(&self) -> Iter<T> {
        Iter {
            next: self.head.as_ref().cloned(),
        }
    }

    fn has_cycle(&self) -> Option<NodeRef<T>> {
        let mut tortoise_iter = self.iter();
        let mut hare_iter = self.iter();
        let mut tortoise = tortoise_iter.next();
        hare_iter.next(); // start 1 iteration ahead of tortoise
        let mut hare = hare_iter.next();
        let mut prev_tortoise = tortoise.as_ref().unwrap().clone();
        let mut prev_hare = hare.as_ref().unwrap().clone();
        while hare.is_some() && tortoise.is_some() {
            if Rc::ptr_eq(hare.as_ref().unwrap(), tortoise.as_ref().unwrap()) {
                if Rc::ptr_eq(&prev_tortoise, &prev_hare) {
                    return Some(prev_tortoise);
                } else {
                    return Some(hare.as_ref().unwrap().clone());
                }
            }
            hare = hare_iter.next();
            prev_hare = hare.as_ref().unwrap().clone();
            if hare.is_some() {
                hare = hare_iter.next();
            }
            prev_tortoise = tortoise.as_ref().unwrap().clone();
            tortoise = tortoise_iter.next();
        }
        None
    }
}

impl<T> Iterator for Iter<T> {
    type Item = NodeRef<T>;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.as_ref()?;
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
    let mut left = LinkedList::<i32>::new();
    left.append(6);
    left.has_cycle();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cycle() {
        let datavec = ['A', 'B', 'C', 'D', 'E'];

        let mut cycle_list = LinkedList::<char>::new();
        for value in datavec.iter() {
            cycle_list.append(*value);
        }

        let mut list_iter = cycle_list.iter();
        list_iter.next();
        list_iter.next();
        let third_node = list_iter.next();
        cycle_list.tail().unwrap().borrow_mut().next = Some(third_node.unwrap());

        let cycle_result = cycle_list.has_cycle();
        assert!(cycle_result.is_some());
        assert_eq!(cycle_result.as_ref().unwrap().borrow().data, 'C');

        let mut nocycle_list = LinkedList::<char>::new();
        for value in datavec.iter() {
            nocycle_list.append(*value);
        }

        let nocycle_result = nocycle_list.has_cycle();
        assert!(nocycle_result.is_none());

        // Second case
        let datavec2 = ['A', 'B', 'C', 'D', 'E', 'F'];

        let mut cycle_list2 = LinkedList::<char>::new();
        for value in datavec2.iter() {
            cycle_list2.append(*value);
        }

        let mut list_iter2 = cycle_list2.iter();
        list_iter2.next();
        list_iter2.next();
        let third_node2 = list_iter2.next();
        cycle_list2.tail().unwrap().borrow_mut().next = Some(third_node2.unwrap());

        let cycle_result2 = cycle_list2.has_cycle();
        assert!(cycle_result2.is_some());
        assert_eq!(cycle_result2.as_ref().unwrap().borrow().data, 'C');
    }
}
